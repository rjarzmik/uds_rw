#[derive(Clone, Debug, Default)]
/// Unknown or unhandled UDS message
pub struct RawUds {
    /// Byte array of the message
    pub data: Vec<u8>,
}
