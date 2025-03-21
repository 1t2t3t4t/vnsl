use std::io::Read;

use anyhow::Result;
use base64::{prelude::BASE64_STANDARD_NO_PAD, Engine};
use flate2::{
    read::{GzDecoder, GzEncoder},
    Compression,
};

pub fn compress_str(content: &str) -> Result<String> {
    let mut enc = GzEncoder::new(content.as_bytes(), Compression::best());
    let mut compressed_out = Vec::new();
    enc.read_to_end(&mut compressed_out)?;
    Ok(BASE64_STANDARD_NO_PAD.encode(compressed_out))
}

pub fn decompress_str(base64_str: &str) -> Result<String> {
    let mut comp_str = Vec::new();
    BASE64_STANDARD_NO_PAD.decode_vec(base64_str.as_bytes(), &mut comp_str)?;
    let mut dec = GzDecoder::new(&comp_str as &[u8]);
    let mut out = String::new();
    dec.read_to_string(&mut out)?;
    Ok(out)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_compress_decompress() {
        let str = "Hello World";
        let comp = super::compress_str(str).unwrap();
        let result = super::decompress_str(&comp).unwrap();
        assert_eq!(str, &result);
    }
}
