use std::io::{Stdout, Write};

pub trait Writer: Write {
    #[cfg(test)]
    fn get_buffer(&self) -> Option<String>;
}

impl Writer for Stdout {
    #[cfg(test)]
    fn get_buffer(&self) -> Option<String> {
        None
    }
}

impl Writer for Vec<u8> {
    #[cfg(test)]
    fn get_buffer(&self) -> Option<String> {
        Some(String::from_utf8(self.clone()).unwrap())
    }
}
