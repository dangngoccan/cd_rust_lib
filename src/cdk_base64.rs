use base64::prelude::*;
use std::io;

use base64::{engine::general_purpose::STANDARD, read::DecoderReader};

pub fn base64_encode()
{
    let mut input = io::stdin();
    let mut decoder = DecoderReader::new(&mut input, &STANDARD);
    io::copy(&mut decoder, &mut io::stdout());
}

pub fn base64_encode_test()
{
    // assert_eq!(BASE64_STANDARD.decode(b"+uwgVQA=")?, b"\xFA\xEC\x20\x55\0");
    assert_eq!(BASE64_STANDARD.encode(b"\xFF\xEC\x20\x55\0"), "/+wgVQA=");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        base64_encode();
    }

}