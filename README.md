
# SRTM for Rust

Reads elevation data from .hgt files in Rust. Supports resolutions of 0.5 angle second, 1 angle second (SRTM1) and 3 angle-seconds (SRTM3).
Works also in no_std enviroment.

You have to implement a file-handling type by implement the trait HgtReader.




## Usage/Examples
see  example/std_linux.rs  for an example running under Linux with std::io
see  tests/no_std_test.rs for an example runninig in no_std enviroment (as template for embedded devices)


## Authors

- [@mwesterm](https://www.github.com/mwesterm)


## License

[MIT](https://choosealicense.com/licenses/mit/)

