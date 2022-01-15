use crate::bitmap::container::Container;
use crate::bitmap::store::{ArrayStore, BitmapStore, Store};
use crate::RoaringBitmap;
use bytemuck::cast_slice_mut;
use byteorder::{LittleEndian, ReadBytesExt};
use std::collections::BTreeMap;
use std::convert::TryFrom;
use std::io;
use std::io::Read;

const SERIAL_COOKIE_NO_RUNCONTAINER: u32 = 12346;
const SERIAL_COOKIE: u16 = 12347;

/// Modification of RoaringBitmap that does deserialization lazily
pub struct LazyRoaringBitmap<'a> {
    containers: BTreeMap<u16, ContainerInfo>,
    data: &'a [u8],
}

struct ContainerInfo {
    len: usize,
    offset: u32,
}

impl<'a> LazyRoaringBitmap<'_> {
    pub fn deserialize_from(data: &'a [u8]) -> io::Result<LazyRoaringBitmap<'a>> {
        let mut reader = data;
        let (size, _has_offsets) = {
            let cookie = reader.read_u32::<LittleEndian>()?;
            if cookie == SERIAL_COOKIE_NO_RUNCONTAINER {
                (reader.read_u32::<LittleEndian>()? as usize, true)
            } else if (cookie as u16) == SERIAL_COOKIE {
                return Err(io::Error::new(io::ErrorKind::Other, "run containers are unsupported"));
            } else {
                return Err(io::Error::new(io::ErrorKind::Other, "unknown cookie value"));
            }
        };

        if size > u16::MAX as usize + 1 {
            return Err(io::Error::new(io::ErrorKind::Other, "size is greater than supported"));
        }

        let mut description_bytes = vec![0u8; size * 4];
        reader.read_exact(&mut description_bytes)?;
        let mut description_bytes = &description_bytes[..];

        let mut offsets = vec![0u8; size * 4];
        reader.read_exact(&mut offsets)?;
        let mut offsets = &offsets[..];

        let mut containers = BTreeMap::new();

        for _ in 0..size {
            let key = description_bytes.read_u16::<LittleEndian>()?;
            let len = usize::from(description_bytes.read_u16::<LittleEndian>()?) + 1;
            let offset = offsets.read_u32::<LittleEndian>()?;

            containers.insert(key, ContainerInfo { len, offset });
        }

        Ok(LazyRoaringBitmap { containers, data })
    }

    pub fn load(&self) -> io::Result<RoaringBitmap> {
        let containers: io::Result<Vec<_>> = self
            .containers
            .keys()
            .map(|key| {
                self.get_container(*key).map(|o| {
                    o.unwrap_or_else(|| Container { key: *key, store: Default::default() })
                })
            })
            .collect();
        Ok(RoaringBitmap { containers: containers? })
    }

    pub fn get_container(&self, key: u16) -> io::Result<Option<Container>> {
        self.containers
            .get(&key)
            .map(|info| {
                let len = info.len;
                let offset = info.offset as usize;
                let store = if len <= 4096 {
                    let mut reader = &self.data[offset..offset + (len * 2)];
                    let mut values = vec![0; len as usize];
                    reader.read_exact(cast_slice_mut(&mut values))?;
                    values.iter_mut().for_each(|n| *n = u16::from_le(*n));
                    let array = ArrayStore::try_from(values)
                        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
                    Store::Array(array)
                } else {
                    let mut reader = &self.data[offset..offset + (1024 * 8)];
                    let mut values = Box::new([0; 1024]);
                    reader.read_exact(cast_slice_mut(&mut values[..]))?;
                    values.iter_mut().for_each(|n| *n = u64::from_le(*n));
                    let bitmap = BitmapStore::try_from(len as u64, values)
                        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
                    Store::Bitmap(bitmap)
                };
                Ok(Container { key, store })
            })
            .transpose()
    }
}

impl RoaringBitmap {
    pub fn lazy_is_disjoint(&self, other: &LazyRoaringBitmap) -> io::Result<bool> {
        for container in &self.containers {
            if let Some(other_container) = other.get_container(container.key)? {
                if !container.is_disjoint(&other_container) {
                    return Ok(false);
                }
            }
        }
        Ok(true)
    }
}
