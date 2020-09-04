use crate::traits::Encode;

// === General encoding functions that might be useful ===

pub(crate) enum LenType {
    L8,
    L16,
    L32,
    L64,
}

macro_rules! impl_encode_int {
    ($t:ty) => {
        impl Encode for $t {
            fn encode(&self) -> Vec<u8> {
                self.to_be_bytes().to_vec()
            }
        }
    };
}

impl_encode_int!(usize);
impl_encode_int!(u8);
impl_encode_int!(u16);
impl_encode_int!(u32);
impl_encode_int!(u64);

fn encode_length(len_type: LenType, l: usize, out: &mut Vec<u8>) {
    match len_type {
        LenType::L8 => out.extend((l as u8).encode()),
        LenType::L16 => out.extend((l as u16).encode()),
        LenType::L32 => out.extend((l as u32).encode()),
        LenType::L64 => out.extend((l as u64).encode()),
    }
}

pub(crate) fn encode_slice(len_type: LenType, b: &[u8], mut out: &mut Vec<u8>) {
    encode_length(len_type, b.len(), &mut out);
    out.extend(b);
}

pub(crate) fn encode_u32_slice(len_type: LenType, b_in: &[u32], mut out: &mut Vec<u8>) {
    encode_length(len_type, b_in.len(), &mut out);
    for b in b_in.iter() {
        out.extend(&b.to_be_bytes());
    }
}

impl<T> Encode for Option<T>
where
    T: Encode,
{
    fn encode(&self) -> Vec<u8> {
        let mut out = Vec::new();
        match self {
            Some(v) => {
                out.push(0x1);
                out.extend(v.encode());
            }
            None => {
                out.push(0x0);
            }
        }
        out
    }
}
