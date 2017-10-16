#[derive(PartialEq, Debug)]
pub struct ResourceProperties {
    pub name: String,
    pub max_level: u8,
    pub replenish_time: Option<u16>
}

#[derive(PartialEq, Debug)]
pub struct ResourceState {
    pub current_level: u8
}
