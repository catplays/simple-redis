use std::ops::Deref;

use bytes::BytesMut;

use crate::{RespDecode, RespEncode, RespError};

use super::{extract_simple_frame_data, CRLF_LEN};



#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub struct SimpleError(pub(crate) String);


impl SimpleError {
    pub fn new(s: impl Into<String>) -> Self {
        SimpleError(s.into())
    }
}


// - error: "-Error message\r\n"
impl RespEncode for SimpleError {
    fn encode(self) -> Vec<u8> {
        format!("-{}\r\n", self.0).into_bytes()
    }
}


// "-Error message\r\n"
impl RespDecode for SimpleError {
    const PREFIX: &'static str = "-";

    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        let end = extract_simple_frame_data(buf, Self::PREFIX)?;
        let databuf = buf.split_to(end + CRLF_LEN);
        let data = String::from_utf8_lossy(&databuf[Self::PREFIX.len()..end]);
        Ok(SimpleError::new(data.to_string()))
    }
    fn expect_length(buf: &[u8]) -> Result<usize, RespError> {
        let end = extract_simple_frame_data(buf, Self::PREFIX)?;
        Ok(end + CRLF_LEN)
    }
}

impl From<&str> for SimpleError {
    fn from(s: &str) -> Self {
        SimpleError(s.to_string())
    }
}


impl Deref for SimpleError {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::resp::frame::RespFrame;

    use super::*;
    use anyhow::Result;
    use bytes::BytesMut;



    #[test]
    fn test_simple_error_decode() -> Result<()> {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b"-Error message\r\n");

        let frame = SimpleError::decode(&mut buf)?;
        assert_eq!(frame, SimpleError::new("Error message".to_string()));

        Ok(())
    }

    #[test]
     fn test_simple_error_encode() {
        //  enum_dispatch 实现了From trait ， 所以可以直接使用into
        let frame: RespFrame = SimpleError::new("ErrorMsg".to_string()).into();
        assert_eq!(frame.encode(), b"-ErrorMsg\r\n");
    }

}