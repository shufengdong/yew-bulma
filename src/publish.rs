use bytes::{BufMut, Bytes, BytesMut};
use core::fmt;
use std::fmt::Debug;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    PacketIdZero,
    PayloadTooLong,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
pub enum QoS {
    AtMostOnce = 0,
    AtLeastOnce = 1,
    ExactlyOnce = 2,
}

#[derive(Clone, PartialEq)]
pub struct Publish {
    pub qos: QoS,
    pub pkid: u16,
    pub topic: String,
    pub payload: Bytes,
    pub dup: bool,
    pub retain: bool,
}

impl Publish {
    pub fn new<S: Into<String>, P: Into<Vec<u8>>>(topic: S, qos: QoS, payload: P) -> Publish {
        Publish {
            dup: false,
            qos,
            retain: false,
            pkid: 0,
            topic: topic.into(),
            payload: Bytes::from(payload.into()),
        }
    }
    fn len(&self) -> usize {
        let mut len = 2 + self.topic.len();
        if self.qos != QoS::AtMostOnce {
            len += 2;
        }

        len += self.payload.len();
        len
    }
    pub fn write(&self, buffer: &mut BytesMut) -> Result<usize, Error> {
        let len = self.len();
        // reserve for maximum possible fixed header
        buffer.reserve(5 + len);

        buffer.put_u8(0x30 | (self.retain as u8) | (self.qos as u8) << 1 | (self.dup as u8) << 3);
        let count = write_remaining_length(buffer, len)?;
        write_mqtt_string(buffer, self.topic.as_str());

        if self.qos != QoS::AtMostOnce {
            let pkid = self.pkid;
            if pkid == 0 {
                return Err(Error::PacketIdZero);
            }
            buffer.put_u16(pkid);
        }

        buffer.extend_from_slice(&self.payload);
        Ok(1 + count + len)
    }
}

impl Debug for Publish {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Topic = {}, Qos = {:?}, Retain = {}, Pkid = {:?}, Payload Size = {}",
            self.topic,
            self.qos,
            self.retain,
            self.pkid,
            self.payload.len()
        )
    }
}

fn write_mqtt_string(stream: &mut BytesMut, string: &str) {
    write_mqtt_bytes(stream, string.as_bytes());
}

fn write_mqtt_bytes(stream: &mut BytesMut, bytes: &[u8]) {
    stream.put_u16(bytes.len() as u16);
    stream.extend_from_slice(bytes);
}

fn write_remaining_length(stream: &mut BytesMut, len: usize) -> Result<usize, Error> {
    if len > 268_435_455 {
        return Err(Error::PayloadTooLong);
    }

    let mut done = false;
    let mut x = len;
    let mut count = 0;

    while !done {
        let mut byte = (x % 128) as u8;
        x /= 128;
        if x > 0 {
            byte |= 128;
        }

        stream.put_u8(byte);
        count += 1;
        done = x == 0;
    }

    Ok(count)
}