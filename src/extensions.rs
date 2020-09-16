use crate::encode_util::*;
use crate::traits::Encode;

// Section 7
// // See IANA registry for registered values
// uint16 ExtensionType;
// struct {
//     ExtensionType extension_type;
//     opaque extension_data<0..2^16-1>;
// } Extension;

type ExtensionType = u16;

#[derive(Debug, Clone)]
pub(crate) struct Extension {
    extension_type: ExtensionType,
    extension_data: Vec<u8>,
}

impl Extension {
    pub(crate) fn encode(&self) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend(&self.extension_type.encode());
        encode_slice(LenType::L16, &self.extension_data, &mut out);
        out
    }
}

pub(crate) fn encode_extensions(extensions: &[Extension]) -> Vec<u8> {
    let mut out = Vec::new();
    let encoded_extensions: Vec<u8> = extensions.iter().map(|e| e.encode()).flatten().collect();
    encode_slice(LenType::L32, &encoded_extensions, &mut out);
    out
}
