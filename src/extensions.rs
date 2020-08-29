
// Section 7
// // See IANA registry for registered values
// uint16 ExtensionType;
// struct {
//     ExtensionType extension_type;
//     opaque extension_data<0..2^16-1>;
// } Extension;

type ExtensionType = u16;

pub(crate) struct Extension {
    extension_type: ExtensionType,
    extension_data: Vec<u8>,
}
