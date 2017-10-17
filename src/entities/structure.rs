use std::collections::HashMap;

#[derive(PartialEq, Debug)]
pub struct Risk {
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
pub struct Size {
    pub width: u8,
    pub height: u8
}

#[derive(PartialEq, Debug)]
pub struct StructureProperties {
    pub name: String,
    pub size: Size,
    pub max_employees: u8,
    pub cost: u32,
    pub desirability: (i8, i8, i8, i8, i8, i8),
    pub structure_type: Type
}

#[derive(PartialEq, Debug)]
pub struct StructureState {
    pub current_employees: u8,
    pub risk: Risk,
    pub commodities: HashMap<String, u32>
}
