use std::ops::Deref;

use bytes::BytesMut;

use crate::{RespDecode, RespEncode, RespError};

use super::{extract_simple_frame_data, CRLF_LEN};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub struct SimpleString(pub(crate) String);

impl SimpleString {
    pub fn new(s: impl Into<String>) -> Self {
        SimpleString(s.into())
    }
}

// "+OK\r\n"
impl RespDecode for SimpleString {
    const PREFIX: &'static str = "+";
    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        let end = extract_simple_frame_data(&buf, Self::PREFIX)?;
        // 分割之后databuf得到了[0,len]的部分，buf得到了剩余的部分为空[len,len]
        let databuf = buf.split_to(end + CRLF_LEN);
        let data = String::from_utf8_lossy(&databuf[Self::PREFIX.len()..end]);
        Ok(SimpleString::new(data.to_string()))
    }
    fn expect_length(buf: &[u8]) -> Result<usize, RespError> {
        let end = extract_simple_frame_data(buf, Self::PREFIX)?;
        Ok(end + CRLF_LEN)
    }
}

// - simple string: "+OK\r\n"
impl RespEncode for SimpleString {
    fn encode(self) -> Vec<u8> {
        format!("+{}\r\n", self.0).into_bytes()
    }
}

impl From<&str> for SimpleString {
    fn from(s: &str) -> Self {
        SimpleString(s.to_string())
    }
}

//自动解引用允许将一个类型视为它所引用的类型的别名，从而可以直接使用引用类型的方法和操作
// 比如将SimpleString 当做String 使用
impl Deref for SimpleString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<str> for SimpleString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::{resp::frame::RespFrame, RespError};

    use super::*;
    use anyhow::Result;
    use bytes::BufMut;

    #[test]
    fn test_simple_string_decode() -> Result<()> {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b"+OK\r\n");

        let frame = SimpleString::decode(&mut buf)?;
        assert_eq!(frame, SimpleString::new("OK".to_string()));

        println!("after 1st decode:{}", buf.len());
        // buf = BytesMut::new();
        buf.extend_from_slice(b"+hello\r");
        println!("before 2rd decode:{}", buf.len());
        let ret = SimpleString::decode(&mut buf);
        assert_eq!(ret.unwrap_err(), RespError::NotComplete);

        buf.put_u8(b'\n');
        let frame = SimpleString::decode(&mut buf)?;
        assert_eq!(frame, SimpleString::new("hello".to_string()));

        Ok(())
    }

    #[test]
    fn test_simple_string_encode() {
        //  enum_dispatch 实现了From trait ， 所以可以直接使用into
        let frame: RespFrame = SimpleString::new("OK".to_string()).into();
        assert_eq!(frame.encode(), b"+OK\r\n");
    }
}
