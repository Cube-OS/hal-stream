use std::io::Result;
use std::time::Duration;

/// High level read/write trait for payload connections to implement
pub trait Stream {
    /// Writes an I2C command
    ///
    /// # Arguments
    ///
    /// `command` - Command to write
    fn write(&self, command: Vec<u8>) -> Result<()>;

    /// Reads command result
    ///
    /// # Arguments
    ///
    /// `command` - Command to read result from
    /// `rx_len`  - Amount of data to read
    fn read(&self, command: &mut Vec<u8>, rx_len: usize) -> Result<Vec<u8>>;

    /// Reads command result with timeout
    ///
    /// # Arguments
    ///
    /// `command` - Command to read result from
    /// `rx_len`  - Amount of data to read
    /// `timeout` - Timeout for the read operation
    fn read_timeout(&self, command: &mut Vec<u8>, rx_len: usize, timeout: Duration) -> Result<Vec<u8>>;

    /// Writes I2C command and reads result
    ///
    /// # Arguments
    ///
    /// `command` - Command to write and read from
    /// `rx_len`  - Amount of data to read
    /// `delay`   - Delay between writing and reading
    fn transfer(&self, command: Vec<u8>, rx_len: usize, delay: Duration) -> Result<Vec<u8>>;
}