#![allow(clippy::result_unit_err)]

pub struct BoundedResponse {
    inner: String,
    len: usize,
    /// Number of characters before hard limit
    reserved: usize,
}

impl From<String> for BoundedResponse {
    fn from(string: String) -> Self {
        let len = string.chars().count();
        Self {
            inner: string,
            len,
            reserved: 0,
        }
    }
}

impl BoundedResponse {
    const LIMIT: usize = 2000;

    pub fn reserve(mut self, reserved: usize) -> Self {
        assert!(reserved < Self::LIMIT);
        self.reserved = reserved;
        self
    }

    fn add_len(&mut self, inc: usize) -> Result<(), ()> {
        self.len += inc;
        if self.len > Self::LIMIT - self.reserved {
            Err(())
        } else {
            Ok(())
        }
    }

    pub fn push_str(&mut self, s: &str) -> Result<(), ()> {
        let inc = s.chars().count();
        self.add_len(inc)?;
        self.inner.push_str(s);
        Ok(())
    }

    pub fn push_line(&mut self, s: &str) -> Result<(), ()> {
        self.push_str(s)?;
        self.push('\n')?;
        Ok(())
    }

    pub fn push(&mut self, c: char) -> Result<(), ()> {
        self.add_len(1)?;
        self.inner.push(c);
        Ok(())
    }

    pub fn finish(self) -> String {
        self.inner
    }
}

#[derive(Default)]
pub struct BoundedResponseVec {
    inner: Vec<String>,
    len: usize,
    /// Number of characters before hard limit
    reserved: usize,
}

impl From<Vec<String>> for BoundedResponseVec {
    fn from(value: Vec<String>) -> Self {
        let len = value.iter().fold(0usize, |acc, s| acc + s.chars().count());
        Self {
            inner: value,
            len,
            reserved: 0,
        }
    }
}

impl BoundedResponseVec {
    const LIMIT: usize = 2000;
    pub fn reserve(mut self, reserved: usize) -> Self {
        assert!(reserved < Self::LIMIT);
        self.reserved = reserved;
        self
    }

    fn add_len(&mut self, inc: usize) -> Result<(), ()> {
        self.len += inc;
        if self.len > Self::LIMIT - self.reserved {
            Err(())
        } else {
            Ok(())
        }
    }

    pub fn push(&mut self, string: String) -> Result<(), ()> {
        let inc = string.chars().count();
        self.add_len(inc)?;
        self.inner.push(string);
        Ok(())
    }

    pub fn finish(self) -> Vec<String> {
        self.inner
    }
}