/// Custom round function for f64 in no_std environments
fn round_f64(x: f64) -> f64 {
    let i = if x < 0.0 {
        (x - 0.5) as i64
    } else {
        (x + 0.5) as i64
    };
    i as f64
}

use super::Coord;
use crate::{Error, HgtReader, resolutions::Resolution};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Tile<R: HgtReader> {
    /// north-south position of the [`Tile`]
    /// angle, ranges from −90° (south pole) to 90° (north pole), 0° is the Equator
    pub latitude: i8,
    /// east-west position of the [`Tile`]
    /// angle, ranges from -180° to 180°
    pub longitude: i16,
    pub resolution: Resolution,
    data_reader: R,
}

impl<R: HgtReader> Tile<R> {
    pub fn new<Reader: HgtReader>(res: Resolution, reader: Reader) -> Tile<Reader> {
        Tile {
            resolution: res,
            data_reader: reader,
            latitude: 0,
            longitude: 0,
        }
    }
    /// Retrieves the height for the specified coordinate from the HGT data file.
    ///
    /// This function converts the given coordinate into a `Coord` type, computes
    /// the corresponding file name, and attempts to open the HGT file. If the file
    /// cannot be processed or the coordinate's index is out of bounds, it logs an
    /// error and returns `None`.
    ///
    /// It calculates the row and column in the HGT data based on the coordinate,
    /// reads the elevation data at the computed index, and returns the height as
    /// an `Option<i16>`. If the data is invalid (e.g., -32768), it logs a warning
    /// and returns `None`.
    ///
    /// # Arguments
    ///
    /// * `coord` - A value that can be converted into a `Coord`, representing the
    ///   geographic coordinate to retrieve the height for.
    ///
    /// # Return
    ///
    /// * `Option<i16>` - The height value if successful, or `None` if an error
    ///   occurs or if the height data is invalid.

    pub fn get_height<Reader: HgtReader>(&mut self, coord: impl Into<Coord>) -> Result<i16, Error> {
        let coord: Coord = coord.into();
        let filename = coord.get_filename();

        self.data_reader
            .open_hgt_file(filename.as_str())
            .and_then(|_| {
                self.data_reader
                    .check_hgt_file(self.resolution.expected_file_length() as u64)
            })?;
        let coord_trunc = coord.trunc();
        let res_size = self.resolution.point_per_degree();
        let lat_diff: f64 = (1.0 - (coord.lat - coord_trunc.0 as f64)) * (res_size as f64 - 1.0);
        let lon_diff = (coord.lon - coord_trunc.1 as f64) * (res_size as f64 - 1.0);
        let row = round_f64(lat_diff) as usize;
        let col = round_f64(lon_diff) as usize;
        let index = (row * res_size + col) * 2;

        if index >= self.resolution.expected_file_length() {
            return Err(Error::IndexOutOfBounds);
        }
        let mut buffer = [0; 2];
        self.data_reader.read_hgt_data(index as u64, &mut buffer)?;

        let height = i16::from_be_bytes(buffer);
        if height == -32768 {
            Err(Error::InvalidData)
        } else {
            Ok(height)
        }
    }
}
