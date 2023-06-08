use std::{
    fs::File,
    io::{Read, Result as IoResult, Seek, SeekFrom, Write},
    path::Path,
};

use crate::types::{Offset, ReadWriteable};

use super::Memory;

#[derive(Debug)]
pub struct FileMemory {
    file: File,
}

impl FileMemory {
    pub fn new(file: impl AsRef<Path>) -> IoResult<Self> {
        Ok(FileMemory {
            file: File::options()
                .read(true)
                .write(true)
                .create(true)
                .open(file.as_ref())?,
        })
    }
}

impl Memory for FileMemory {
    type Error = std::io::Error;

    fn read<T: ReadWriteable>(&mut self) -> Result<T, Self::Error> {
        // NOTE: With full const generics, we should be able to define the
        // buffer size as exactly T::NUM_BYTES, which might(?) allow
        // the compiler to optimize this better and will be clearer

        let mut buffer = [0_u8; 8];
        self.file.read_exact(&mut buffer[0..T::NUM_BYTES])?;
        let value = T::from_bytes(&buffer);
        Ok(value)
    }

    fn read_if_present<T: ReadWriteable>(&mut self) -> Result<Option<T>, Self::Error> {
        let mut buffer = [0_u8; 8];
        match self.file.read_exact(&mut buffer[0..T::NUM_BYTES]) {
            Ok(_) => {
                let value = T::from_bytes(&buffer);
                Ok(Some(value))
            }
            Err(err) if err.kind() == std::io::ErrorKind::UnexpectedEof => Ok(None),
            Err(err) => Err(err),
        }
    }

    fn write<T: ReadWriteable>(&mut self, value: T) -> Result<(), Self::Error> {
        let mut buffer = [0_u8; 8];
        value.into_bytes(&mut buffer[0..T::NUM_BYTES]);
        self.file.write_all(&buffer[0..T::NUM_BYTES])?;
        Ok(())
    }

    fn seek(&mut self, pos: Offset) -> Result<(), Self::Error> {
        self.file.seek(SeekFrom::Current(pos.0 as i64))?;
        Ok(())
    }
}
