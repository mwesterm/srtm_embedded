const EXTENT: usize = 3600;

#[derive(PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Debug, Default)]
pub enum Resolution {
    SRTM05,
    #[default]
    SRTM1,
    SRTM3,
}

impl Resolution {
    /// Returns the number of data points per degree of latitude or longitude
    /// for the given resolution.
    ///
    /// # Returns
    ///
    /// * `usize` - The number of data points per degree, which varies based on
    ///   the resolution type:
    ///   * `SRTM05`: 7201 points per degree
    ///   * `SRTM1`: 3601 points per degree
    ///   * `SRTM3`: 1201 points per degree

    pub const fn point_per_degree(&self) -> usize {
        match self {
            Resolution::SRTM05 => EXTENT * 2 + 1,
            Resolution::SRTM1 => EXTENT + 1,
            Resolution::SRTM3 => EXTENT / 3 + 1,
        }
    }
    /// Calculates the expected length of an HGT file for the given
    /// resolution, which is the number of data points per degree of
    /// latitude or longitude, times the number of data points per degree
    /// of longitude, times two (for the two bytes of data per point).
    pub const fn expected_file_length(&self) -> usize {
        let points = self.point_per_degree();
        points * points * 2
    }
}
