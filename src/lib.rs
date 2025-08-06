#![no_std]
pub use coords::Coord;
pub use resolutions::Resolution;
pub use tiles::Tile;
// MUST be the first module
mod fmt;

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
}

pub trait HgtReader {
    fn open_hgt_file(&mut self, file_name: &str) -> Result<(), Error>;
    fn check_hgt_file(&self, expt_len: u64) -> Result<(), Error>;
    fn read_hgt_data(&mut self, pos: u64, buffer: &mut [u8; 2]) -> Result<(), Error>;
    fn close_hgt_file(&mut self) -> Result<(), Error>;
}
