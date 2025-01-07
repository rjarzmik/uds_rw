#[derive(Clone, Debug, Default)]
/// Read DID request
pub struct ReadDIDReq {
    /// Diagnostic Identifier
    pub did: u16,
}

#[derive(Clone, Debug, Default)]
/// Read DID response
pub struct ReadDIDRsp {
    /// Diagnostic Identifier
    pub did: u16,
    /// Diagnostic Identifier value
    pub user_data: Vec<u8>,
}

#[derive(Clone, Debug, Default)]
/// Write DID request
pub struct WriteDIDReq {
    /// Diagnostic Identifier
    pub did: u16,
    /// Diagnostic Identifier value
    pub user_data: Vec<u8>,
}

#[derive(Clone, Debug, Default)]
/// Write DID response
pub struct WriteDIDRsp {
    /// Diagnostic Identifier
    pub did: u16,
}
