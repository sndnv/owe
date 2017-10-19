use std::collections::HashMap;

#[derive(PartialEq, Clone, Debug)]
pub struct WalkerProperties {
    pub name: String,
    pub patrol: Option<u8>,
    pub max_life: Option<u16>
}

#[derive(PartialEq, Clone, Debug)]
pub struct WalkerState {
    pub commodities: HashMap<String, u32>,
    pub current_life: Option<u16>
}
