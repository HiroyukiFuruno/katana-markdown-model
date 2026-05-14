use super::types::{RawSnippet, TextFingerprint};
use sha2::{Digest, Sha256};

impl RawSnippet {
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }

    pub fn fingerprint(&self) -> TextFingerprint {
        TextFingerprint::for_text(&self.text)
    }
}

impl TextFingerprint {
    pub fn for_text(text: &str) -> Self {
        let digest = Sha256::digest(text.as_bytes());
        Self {
            algorithm: "sha256".to_string(),
            value: to_hex(&digest),
        }
    }
}

fn to_hex(bytes: &[u8]) -> String {
    const HIGH_NIBBLE_SHIFT: u8 = 4;
    const LOW_NIBBLE_MASK: u8 = 0x0f;
    const HEX_CHARS_PER_BYTE: usize = 2;
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(bytes.len() * HEX_CHARS_PER_BYTE);
    for byte in bytes {
        output.push(HEX[(byte >> HIGH_NIBBLE_SHIFT) as usize] as char);
        output.push(HEX[(byte & LOW_NIBBLE_MASK) as usize] as char);
    }
    output
}
