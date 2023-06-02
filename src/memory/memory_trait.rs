use crate::types::ReadWriteable;

pub trait Memory {
    type Error;

    fn read<T: ReadWriteable>(&mut self) -> Result<T, Self::Error>;
    fn read_if_present<T: ReadWriteable>(&mut self) -> Result<Option<T>, Self::Error>;
    fn write<T: ReadWriteable>(&mut self, value: T) -> Result<(), Self::Error>;
    fn seek(&mut self, pos: i16) -> Result<(), Self::Error>;
}
