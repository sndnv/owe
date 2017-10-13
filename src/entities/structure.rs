use std::collections::HashMap;

#[derive(PartialEq, Debug)]
pub struct Employees {
    pub required: u8,
    pub current: u8 //TODO - move to enum
}

#[derive(PartialEq, Debug)]
pub struct Risk { //TODO - move to enum
    pub fire: u8,
    pub damage: u8
}

#[derive(PartialEq, Debug)]
pub enum Type {
    Housing,
    Entertainment,
    Military,
    Industry,
    Monument,
    Religion,
    Education,
    HealthCare,
    CivilService
}

#[derive(PartialEq, Debug)]
pub struct Structure {
    pub name: String,
    pub size: (u8, u8),
    pub employees: Employees,
    pub cost: u32,
    pub desirability: (i8, i8, i8, i8, i8, i8),
    pub risk: Risk,
    pub commodities: HashMap<String, u32>, //TODO - move to enum
    pub structure_type: Type
}
