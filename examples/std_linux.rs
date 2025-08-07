use log::{debug, error, info};
use srtm_embedded::{HgtReader, Resolution, Tile, coords};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::result::Result;

struct HgtReaderStd {
    file: Option<File>,
    file_name: String,
    is_open: bool,
}

impl HgtReaderStd {
    pub fn new() -> Self {
        HgtReaderStd {
            file: None,
            file_name: String::new(),
            is_open: false,
        }
    }
}

impl HgtReader for HgtReaderStd {
    /// Opens an HGT file with the specified file name.
    ///
    /// # Arguments
    ///
    /// * `file_name` - The name of the HGT file to be opened.
    ///
    /// # Return Value
    ///
    /// Returns `Ok(())` if the file was successfully opened, otherwise an `Error`.
    ///
    /// # Errors
    ///
    /// This function returns an error if the file cannot be opened.
    fn open_hgt_file(&mut self, file_name: &str) -> std::result::Result<(), srtm_embedded::Error> {
        if self.file_name != file_name || !self.is_open {
            self.close_hgt_file().ok(); // Ensure any previously open file is closed
            //            let file_name = format!("data/{}", file_name);
            debug!("Opening file: {}", file_name);
            self.file = File::open(file_name)
                .map(Some)
                .map_err(|_| srtm_embedded::Error::FileNotFound)?;
            self.file_name = file_name.to_string();
            self.is_open = true;
        }
        Ok(())
    }
    fn check_hgt_file(&self, expt_len: u64) -> Result<(), srtm_embedded::Error> {
        if let Some(ref file) = self.file {
            let metadata = file
                .metadata()
                .map_err(|_| srtm_embedded::Error::Filesize)?;
            let file_size = metadata.len();
            if file_size != expt_len {
                error!(
                    "File size mismatch: expected {}, got {}",
                    expt_len, file_size
                );
                return Err(srtm_embedded::Error::Filesize);
            }
        } else {
            return Err(srtm_embedded::Error::NotFound);
        }
        Ok(())
    }
    /// Reads HGT data from the file at the specified position.
    fn read_hgt_data(
        &mut self,
        pos: u64,
        buff: &mut [u8; 2],
    ) -> std::result::Result<(), srtm_embedded::Error> {
        if let Some(ref mut file) = self.file {
            file.seek(SeekFrom::Start(pos))
                .map_err(|_| srtm_embedded::Error::Filesize)?;

            file.read_exact(buff)
                .map_err(|_| srtm_embedded::Error::Filesize)?;
            Ok(())
        } else {
            Err(srtm_embedded::Error::NotFound)
        }
    }
    fn close_hgt_file(&mut self) -> Result<(), srtm_embedded::Error> {
        if self.is_open {
            self.file = None;
            self.is_open = false;
            Ok(())
        } else {
            Err(srtm_embedded::Error::NotFound)
        }
    }
}

fn main() {
    // Initialize logging
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("debug"));
    info!("Starting the HGT Reader example...");
    // Example coordinates
    let latitude = 49.1;
    let longitude = 8.2;
    let coords = coords::Coord::new(latitude, longitude);
    let filename = coords.get_filename();
    debug!("Generated filename: {}", filename);
    let expected_filename = "N49E008.hgt"; // Assuming this is the expected format
    assert_eq!(filename, expected_filename, "Filename creation failed");
    let reader = HgtReaderStd::new();
    let mut tile = Tile::<HgtReaderStd>::new(Resolution::SRTM1, reader);
    let height = tile.get_height::<HgtReaderStd>((latitude, longitude));
    assert_eq!(height, Ok(127), "Height retrieval failed");
    debug!("Height: {:?}", height);
}
