use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use std::{
    io::{Cursor, Read},
    string::FromUtf8Error,
};

#[derive(Debug, Clone)]
pub struct LiveLinkFacePayload {
    pub version: i32,
    pub uuid: String,
    pub name: String,

    pub frame_number: i32,
    pub sub_frame: f32,
    pub fps: i32,
    pub denominator: i32,

    pub blend_shapes: [f32; 61],
}

impl Default for LiveLinkFacePayload {
    fn default() -> Self {
        Self {
            version: 0,
            uuid: String::new(),
            name: String::new(),

            frame_number: 0,
            sub_frame: 0.0,
            fps: 0,
            denominator: 0,

            blend_shapes: [0.0; 61],
        }
    }
}

pub enum LiveLinkFacePayloadParseError {
    Error(),
}

impl From<std::io::Error> for LiveLinkFacePayloadParseError {
    fn from(_value: std::io::Error) -> Self {
        return LiveLinkFacePayloadParseError::Error();
    }
}

impl From<FromUtf8Error> for LiveLinkFacePayloadParseError {
    fn from(_value: FromUtf8Error) -> Self {
        return LiveLinkFacePayloadParseError::Error();
    }
}

impl LiveLinkFacePayload {
    pub fn from_raw(bytes: &[u8]) -> Result<Self, LiveLinkFacePayloadParseError> {
        let mut cursor = Cursor::new(bytes);

        // version = struct.unpack('<i', bytes_data[0:4])[0]
        let version = cursor.read_i32::<LittleEndian>()?;

        // uuid = bytes_data[4:41].decode("utf-8")
        let mut uuid_bytes = [0u8; 37];
        cursor.read_exact(&mut uuid_bytes)?;

        let uuid = String::from_utf8(uuid_bytes.to_vec())?;

        // name_length = struct.unpack('!i', bytes_data[41:45])[0]
        let name_length = cursor.read_i32::<BigEndian>()?;

        if name_length < 0 {
            return Err(LiveLinkFacePayloadParseError::Error());
        }

        // name = bytes_data[45:name_end_pos].decode("utf-8")
        let mut name_bytes = vec![0u8; name_length as usize];
        cursor.read_exact(&mut name_bytes)?;

        let name = String::from_utf8(name_bytes)?;

        let name_end_pos = cursor.position() as usize;

        // Not enough shapes, could be a purposefully empty shape
        if bytes.len() <= name_end_pos + 16 {
            return Ok(LiveLinkFacePayload::default());
        }

        // struct.unpack("!if2ib", ...)
        let frame_number = cursor.read_i32::<BigEndian>()?;

        let sub_frame = cursor.read_f32::<BigEndian>()?;

        let fps = cursor.read_i32::<BigEndian>()?;

        let denominator = cursor.read_i32::<BigEndian>()?;

        let data_length = cursor.read_i8()?;

        if data_length != 61 {
            return Err(LiveLinkFacePayloadParseError::Error());
        }

        let mut blend_shapes = [0.0_f32; 61];

        for value in blend_shapes.iter_mut() {
            *value = cursor.read_f32::<BigEndian>()?;
        }

        Ok(LiveLinkFacePayload {
            version,
            uuid,
            name,

            frame_number,
            sub_frame,
            fps,
            denominator,

            blend_shapes,
        })
    }
}
