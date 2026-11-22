use serde::{Deserialize, Deserializer, Serialize, Serializer, de};

pub mod delimiter_as_char {
    use super::*;
    pub fn serialize<S>(byte: &u8, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let ch = *byte as char;
        let mut buf = [0u8; 4];
        let s = ch.encode_utf8(&mut buf);
        s.serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<u8, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let mut chars = s.chars();

        let ch = chars
            .next()
            .ok_or_else(|| de::Error::custom("delimiter cannot be empty"))?;

        if chars.next().is_some() {
            return Err(de::Error::custom(format!(
                "delimiter must be a single character, got {:?}",
                s
            )));
        }

        if !ch.is_ascii() {
            return Err(de::Error::custom(format!(
                "delimiter must be an ASCII character, got '{}'",
                ch
            )));
        }

        Ok(ch as u8)
    }
}
