use entities::walker::WalkerModifiers;

pub struct EffectError {
    pub message: Option<String>,
}

pub enum StructureTarget {
    FireRisk,
    DamageRisk,
}

pub enum HousingStructureTarget {
    Education,
    Religion,
    Entertainment,
    HealthCare,
}

pub enum Effect {
    Structure {
        target_type: StructureTarget,
        operation: fn(u8) -> u8,
    },

    HousingStructure {
        target_type: HousingStructureTarget,
        target_name: String,
        operation: fn(u8) -> u8,
    },

    ProductionStructure {
        new_production_rate: u8
    },

    Walker {
        new_modifiers: WalkerModifiers,
    },

    Resource {
        new_production_rate: u8
    },

    Cell {
        new_desirability: i8,
        new_ground_fertility: u8,
        new_water_availability: u8,
        new_construction_allowed: bool,
    },
}

pub struct ProcessEffects {
    pub effects: Vec<Effect>
}

impl ResponseType for ProcessEffects {
    type Item = ();
    type Error = EffectError;
}
