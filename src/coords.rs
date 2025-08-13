use core::f64;

use core::fmt::Write;
use heapless::String;

/// Represents geographic coordinates (latitude and longitude).
///
/// The struct stores latitude (`lat`) and longitude (`lon`) as floating point numbers.
/// Latitude: -90 to 90 (north/south), Longitude: -180 to 180 (east/west).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Default)]
pub struct Coord {
    /// Geographic latitude (north/south) in degrees.
    pub lat: f64,
    /// Geographic longitude (east/west) in degrees.
    pub lon: f64,
}

impl Coord {
    /// Creates a new `Coord` if the values are valid.
    ///
    /// Returns `Some(Coord)` if latitude is in [-90, 90] and longitude is in [-180, 180], otherwise `None`.
    pub fn opt_new(lat: impl Into<f64>, lon: impl Into<f64>) -> Option<Self> {
        let lat = lat.into();
        let lon = lon.into();
        if (-90. ..=90.).contains(&lat) && (-180. ..=180.).contains(&lon) {
            Some(Self { lat, lon })
        } else {
            None
        }
    }

    /// Creates a new `Coord` and enforces valid values.
    ///
    /// Panics if the values are outside the allowed ranges.
    pub fn new(lat: impl Into<f64>, lon: impl Into<f64>) -> Self {
        Self::opt_new(lat, lon).expect("latitude must be between -90 and 90 degrees, longitude must be between -180 and 180 degrees")
    }

    /*
    // Methods for future extensions to modify coordinates:
    // pub fn with_lat(self, lat: impl Into<f64>) -> Self {
    //     Self::new(lat, self.lon)
    // }
    // pub fn with_lon(self, lon: impl Into<f64>) -> Self {
    //     Self::new(self.lat, lon)
    // }
    // pub fn add_to_lat(self, lat: impl Into<f64>) -> Self {
    //     self.with_lat(self.lat + lat.into())
    // }
    // pub fn add_to_lon(self, lon: impl Into<f64>) -> Self {
    //     self.with_lon(self.lon + lon.into())
    // }
     */

    /// Truncates latitude and longitude to integers.
    /// Returns: (truncated latitude as i8, truncated longitude as i16)
    pub fn trunc(&self) -> (i8, i16) {
        let lat_trunc = self.lat as i8;
        let lon_trunc = self.lon as i16;
        (lat_trunc, lon_trunc)
    }

    /// Returns the filename of the SRTM elevation file covering this point.
    ///
    /// The format is e.g. "N49E008.hgt".
    ///
    /// # Example
    /// ```
    /// use srtm_embedded::Coord;
    /// let coord = Coord::new(87.235, 10.4234423);
    /// let filename = coord.get_filename();
    /// assert_eq!(filename, "N87E010.hgt");
    /// ```
    pub fn get_filename(self) -> String<12> {
        // Determine the sign for latitude and longitude
        let lat_ch = if self.lat >= 0. { 'N' } else { 'S' };
        let lon_ch = if self.lon >= 0. { 'E' } else { 'W' };
        let (lat, lon) = self.trunc();
        let (lat, lon) = (lat.abs(), lon.abs());
        let mut output = String::<12>::new(); // Maximum length of the filename
        write!(
            output,
            "{lat_ch}{}{lat}{lon_ch}{}{lon}.hgt",
            if lat < 10 { "0" } else { "" },
            if lon < 10 {
                "00"
            } else if lon < 100 {
                "0"
            } else {
                ""
            },
        )
        .unwrap(); // Ignore error, since String is large enough
        output
    }
}

/// Allows conversion from a tuple of two f64 values to a `Coord`.
/// The tuple is expected to contain valid latitude and longitude values.
impl From<(f64, f64)> for Coord {
    fn from(value: (f64, f64)) -> Self {
        let (lat, lon) = (value.0, value.1);
        Coord { lat, lon }
    }
}

/// Allows conversion from a tuple of two Strings to a `Coord`.
/// Expects both strings to represent valid floating point numbers.
impl From<(String<8>, String<8>)> for Coord {
    fn from(value: (String<8>, String<8>)) -> Self {
        let (lat, lon) = (
            value.0.parse::<f64>().unwrap(),
            value.1.parse::<f64>().unwrap(),
        );
        Coord { lat, lon }
    }
}

/// Allows conversion from a tuple of two &str to a `Coord`.
/// Expects both strings to represent valid floating point numbers.
impl From<(&str, &str)> for Coord {
    fn from(value: (&str, &str)) -> Self {
        let (lat, lon) = (
            value.0.parse::<f64>().unwrap(),
            value.1.parse::<f64>().unwrap(),
        );
        Coord { lat, lon }
    }
}
