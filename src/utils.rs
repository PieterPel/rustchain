use std::fmt;

pub trait ValidData: fmt::Display + Default {}

impl ValidData for String {}

#[derive(Default)]
pub struct StringBytes {
    pub bytes: Vec<u8>,
}

impl fmt::Display for StringBytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in &self.bytes {
            write!(f, "{:02X} ", byte)?;
        }
        Ok(())
    }
}

impl ValidData for StringBytes {}
