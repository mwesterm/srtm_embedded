#![no_std]
#[cfg(test)]
mod tests {

    use srtm_embedded::{HgtReader, Resolution, Tile, coords};
    extern crate alloc;
    use alloc::{format, string::String};

    const SYS_OPENAT: usize = 257;
    const SYS_READ: usize = 0;
    const SYS_PREAD64: usize = 17;

    const SYS_CLOSE: usize = 3;
    const AT_FDCWD: isize = -100; // "Current working directory" für openat

    pub unsafe fn open(path: *const u8, flags: usize, mode: usize) -> isize {
        // openat(AT_FDCWD, path, flags, mode)
        let ret: isize;
        unsafe {
            core::arch::asm!(
                "syscall",
                in("rax") SYS_OPENAT,
                in("rdi") AT_FDCWD,
                in("rsi") path,
                in("rdx") flags,
                in("r10") mode,
                lateout("rax") ret,
            );
        }

        ret
    }

    pub unsafe fn pread(fd: isize, buf: *mut u8, count: usize, offset: usize) -> isize {
        let ret: isize;
        unsafe {
            core::arch::asm!(
                "syscall",
                in("rax") SYS_PREAD64,
                in("rdi") fd,
                in("rsi") buf,
                in("rdx") count,
                in("r10") offset,
                lateout("rax") ret,
            );
        }
        ret
    }
    pub unsafe fn close(fd: isize) -> isize {
        let ret: isize;
        unsafe {
            core::arch::asm!(
                "syscall",
                in("rax") SYS_CLOSE,
                in("rdi") fd,
                lateout("rax") ret,
            );
        }
        ret
    }
    struct HgtReaderNoStd {
        file: isize,
        file_name: String,
        is_open: bool,
    }

    impl HgtReaderNoStd {
        pub fn new() -> Self {
            HgtReaderNoStd {
                file: -1,
                file_name: String::new(),
                is_open: false,
            }
        }
    }

    impl HgtReader for HgtReaderNoStd {
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
        fn open_hgt_file(&mut self, file_name: &str) -> Result<(), srtm_embedded::Error> {
            // If the file is already open
            if self.is_open {
                // If the current file name doesn't match, close the file
                if self.file_name != file_name {
                    self.close_hgt_file()?;
                    self.is_open = false;
                } else {
                    // Otherwise, the file is already open with the correct name
                    return Ok(());
                }
            }
            self.file = unsafe { open(file_name.as_ptr(), SYS_READ, 0) };
            if self.file == -1 {
                return Err(srtm_embedded::Error::FileNotFound);
            }
            self.file_name = format!("{}", file_name);
            self.is_open = true;
            Ok(())
        }
        fn check_hgt_file(&self, _expt_len: u64) -> Result<(), srtm_embedded::Error> {
            //no filesizecheck here
            Ok(())
        }
        /// Reads HGT data from the file at the specified position.
        fn read_hgt_data(
            &mut self,
            pos: u64,
            buff: &mut [u8; 2],
        ) -> Result<(), srtm_embedded::Error> {
            if self.is_open == false {
                return Err(srtm_embedded::Error::NotFound);
            }
            let res = unsafe { pread(self.file, buff.as_mut_ptr(), 2, pos.try_into().unwrap()) };
            if res == -1 {
                return Err(srtm_embedded::Error::ReadError);
            }
            Ok(())
        }

        fn close_hgt_file(&mut self) -> Result<(), srtm_embedded::Error> {
            if self.is_open {
                unsafe {
                    close(self.file.try_into().unwrap());
                }
                self.file = -1;
                self.is_open = false;
                Ok(())
            } else {
                Err(srtm_embedded::Error::NotFound)
            }
        }
    }

    #[test]
    fn test_create_filename() {
        let latitude = 49.1;
        let longitude = 8.2;
        let coords = coords::Coord::new(latitude, longitude);
        let filename = coords.get_filename();
        let expected_filename = "N49E008.hgt"; // Assuming this is the expected format
        assert_eq!(filename, expected_filename, "Filename creation failed");
        let reader = HgtReaderNoStd::new();
        let mut tile = Tile::<HgtReaderNoStd>::new(Resolution::SRTM3, reader);
        let height = tile.get_height::<HgtReaderNoStd>((latitude, longitude));
        assert_eq!(height, Ok(126), "Height retrieval failed");
        assert_ne!(height, Ok(128), "Height retrieval failed");
    }
}
