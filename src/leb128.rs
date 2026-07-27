pub struct Leb128;

impl Leb128 {
    pub fn decode_u32(bytes: &[u8], offset: &mut usize) -> Result<u32, &'static str> {
        let mut result = 0u32;
        let mut shift = 0;
        loop {
            if *offset >= bytes.len() {
                return Err("Unexpected EOF while decoding u32 LEB128");
            }
            let byte = bytes[*offset];
            *offset += 1;
            result |= ((byte & 0x7f) as u32) << shift;
            if (byte & 0x80) == 0 {
                break;
            }
            shift += 7;
            if shift >= 35 {
                return Err("LEB128 u32 overflow");
            }
        }
        Ok(result)
    }

    pub fn decode_i32(bytes: &[u8], offset: &mut usize) -> Result<i32, &'static str> {
        let mut result = 0i32;
        let mut shift = 0;
        let mut byte;
        loop {
            if *offset >= bytes.len() {
                return Err("Unexpected EOF while decoding i32 LEB128");
            }
            byte = bytes[*offset];
            *offset += 1;
            result |= ((byte & 0x7f) as i32) << shift;
            shift += 7;
            if (byte & 0x80) == 0 {
                break;
            }
        }
        if shift < 32 && (byte & 0x40) != 0 {
            result |= !0 << shift;
        }
        Ok(result)
    }
}
