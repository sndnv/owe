use std::collections::HashMap;

#[derive(PartialEq, Debug)]
pub struct Walker {
    pub name: String,
    pub patrol: Option<u8>,
    pub life: Option<u16>,
    pub commodities: HashMap<String, u32>
}
