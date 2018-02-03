#[derive(PartialEq, Clone, Debug)]
pub struct ResourceProperties {
    pub name: String,
    pub max_amount: u32,
    pub replenish_amount: Option<u32>,
}

#[derive(PartialEq, Clone, Debug)]
pub struct ResourceState {
    pub current_amount: u32
}
