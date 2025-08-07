#![no_std]
pub use coords::Coord;
pub use resolutions::Resolution;
pub use tiles::Tile;

pub mod coords;
pub mod resolutions;
pub mod tiles;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    NotFound,
    ReadError,
    ParseLatLong,
    FileNotFound,
    Filesize,
    FileRead,
    IndexOutOfBounds,
    InvalidData,
}

/// HgtReader is a trait for reading SRTM elevation data.
/// It provides the necessary methods for opening a file,
/// checking its size, reading data at a given position,
/// and closing the file.
pub trait HgtReader {
    /// Opens a file with the given name.
    /// Returns `Ok(())` when the file is successfully opened,
    /// otherwise returns an `Error`.
    fn open_hgt_file(&mut self, file_name: &str) -> Result<(), Error>;

    /// Checks the size of the file.
    /// The expected length of the file is given in bytes.
    /// Returns `Ok(())` if the file size matches the expected length,
    /// otherwise returns an `Error`.
    fn check_hgt_file(&self, expt_len: u64) -> Result<(), Error>;

    /// Reads data from the file at the given position.
    /// The position is given in bytes.
    /// The data is read into the provided buffer.
    /// The buffer must be of size 2.
    /// Returns `Ok(())` if the data is successfully read,
    /// otherwise returns an `Error`.
    fn read_hgt_data(&mut self, pos: u64, buffer: &mut [u8; 2]) -> Result<(), Error>;

    /// Closes the file.
    /// Returns `Ok(())` if the file is successfully closed,
    /// otherwise returns an `Error`.
    fn close_hgt_file(&mut self) -> Result<(), Error>;
}
