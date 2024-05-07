use std::io::{Read,Write};
use std::time::Duration;
// adding comment to test something

/// High level read/write trait for payload connections to implement
pub trait Stream {
    type StreamError: std::fmt::Debug;

    /// Write
    ///
    /// # Arguments
    ///
    /// `data` - Data to write
    fn write(&self, data: Vec<u8>) -> Result<(), Self::StreamError>;

    /// Write multiple single bytes
    ///
    /// # Arguments
    ///
    /// `data` - Data to write
    fn write_bytes(&self, data: Vec<u8>) -> Result<(), Self::StreamError>;

    /// Read Command result
    ///
    /// # Arguments
    ///
    /// `command` - Command to read result from
    /// `rx_len`  - Amount of data to read
    fn read(&self, command: &mut Vec<u8>, rx_len: usize) -> Result<Vec<u8>,Self::StreamError>;

    /// Reads command result with timeout
    ///
    /// # Arguments
    ///
    /// `command` - Command to read result from
    /// `rx_len`  - Amount of data to read
    /// `timeout` - Timeout for the read operation
    fn read_timeout(&self, command: &mut Vec<u8>, rx_len: usize, timeout: Duration) -> Result<Vec<u8>,Self::StreamError>;

    /// Writes I2C command and reads result
    ///
    /// # Arguments
    ///
    /// `command` - Command to write and read from
    /// `rx_len`  - Amount of data to read
    /// `delay`   - Delay between writing and reading, time-out for UDP read
    fn transfer(&self, command: Vec<u8>, rx_len: usize, delay: Option<Duration>) -> Result<Vec<u8>,Self::StreamError>;
}