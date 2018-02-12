use actix::{Actor, Context, Handler, ResponseType};
use effects::{Effect, EffectError, HousingStructureTarget, ProcessEffects, StructureTarget};
use entities::{Entity, EntityActor};
use map::{ProcessTick, TickError, TickResult};
use production::{CommodityProductionResult, WalkerProductionResult};
use std::collections::hash_map::Entry;
use std::collections::HashMap;

#[derive(PartialEq, Clone, Debug)]
pub struct Risk {
    pub fire: u8,
    pub damage: u8,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Education {
    pub data: HashMap<String, u8>,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Religion {
    pub data: HashMap<String, u8>,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Entertainment {
    pub data: HashMap<String, u8>,
}

#[derive(PartialEq, Clone, Debug)]
pub struct HealthCare {
    pub data: HashMap<String, u8>,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Government {
    pub taxes_paid: u32,
    pub data: HashMap<String, u8>,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Commodities {
    pub data: HashMap<String, u32>,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Housing {
    pub education: Education,
    pub religion: Religion,
    pub entertainment: Entertainment,
    pub health_care: HealthCare,
    pub government: Government,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Production {
    pub current_employees: u8,
    pub production_rate: u8,
}

#[derive(PartialEq, Clone, Debug)]
pub enum Type {
    Housing,
    Entertainment,
    Military,
    Industry,
    Monument,
    Religion,
    Education,
    HealthCare,
    CivilService,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Size {
    pub width: u8,
    pub height: u8,
}

#[derive(PartialEq, Clone, Debug)]
pub struct StructureProperties {
    pub name: String,
    pub size: Size,
    pub max_people: u8,
    pub cost: u32,
    pub desirability: (i8, i8, i8, i8, i8, i8),
    pub structure_type: Type,
    pub effects: Vec<Effect>,
}

#[derive(PartialEq, Clone, Debug)]
pub struct StructureState {
    pub risk: Option<Risk>,
    pub commodities: Option<Commodities>,
    pub housing: Option<Housing>,
    pub production: Option<Production>,
}

pub trait Structure: Entity<StructureProperties, StructureState> {
    fn produce_commodity(&mut self) -> CommodityProductionResult;
    fn produce_walker(&mut self) -> WalkerProductionResult;
}

impl<T: Structure> Handler<ProcessTick> for EntityActor<T> {
    type Result = Result<TickResult, TickError>;

    fn handle(&mut self, msg: ProcessTick, _: &mut Context<Self>) -> Self::Result {
        let commodity_result = self.entity.produce_commodity();
        let walker_result = self.entity.produce_walker();

        match (commodity_result, walker_result) {
            (Ok(commodity_data), Ok(walker_data)) => {
                Ok(TickResult::Structure { commodity_data, walker_data })
            }

            (Err(commodity_error), Ok(_)) => {
                Err(TickError::Structure { commodity_error: Some(commodity_error), walker_error: None })
            }

            (Ok(_), Err(walker_error)) => {
                Err(TickError::Structure { commodity_error: None, walker_error: Some(walker_error) })
            }

            (Err(commodity_error), Err(walker_error)) => {
                Err(TickError::Structure { commodity_error: Some(commodity_error), walker_error: Some(walker_error) })
            }
        }
    }
}

impl<T: Structure> Handler<ProcessEffects> for EntityActor<T> {
    type Result = Result<(), EffectError>;

    fn handle(&mut self, msg: ProcessEffects, _: &mut Context<Self>) -> Self::Result {
        let results = msg.effects.iter().for_each(|effect| {
            match effect {
                Effect::Structure { ref target_type, operation } => {
                    let state = self.entity.get_state();

                    match state.risk {
                        Some(risk_state) => {
                            let updated_risk_state = match target_type {
                                StructureTarget::FireRisk => {
                                    Risk { fire: (operation)(risk_state.fire), ..risk_state }
                                }

                                StructureTarget::DamageRisk => {
                                    Risk { damage: (operation)(risk_state.damage), ..risk_state }
                                }
                            };

                            self.entity.set_state(
                                StructureState {
                                    risk: updated_risk_state,
                                    ..state
                                }
                            );
                            Ok(())
                        }

                        None => {
                            let message = format!(
                                "Structure effect cannot be applied to [{}]",
                                self.entity.get_props().name
                            );

                            Err(EffectError { message: Some(message) })
                        }
                    }
                }

                Effect::HousingStructure { ref target_type, ref target_name, operation } => {
                    let mut state = self.entity.get_state();
                    let apply_effect = |target_type: HousingStructureTarget, target_name: String, data: &mut HashMap<String, u8>, | -> Self::Result {
                        match data.entry(target_name) {
                            Entry::Occupied(mut entry) => {
                                let entry = entry.into_mut();
                                entry = (operation)(entry.get());
                                Ok(())
                            }

                            Entry::Vacant(_) => {
                                let message = format!(
                                    "Entry for [{}] not found in [{}] data for [{}]",
                                    target_name,
                                    target_type,
                                    self.entity.get_props().name
                                );
                                Err(EffectError { message: Some(message) })
                            }
                        }
                    };

                    match state.housing {
                        Some(housing_state) => {
                            match target_type {
                                HousingStructureTarget::Education => {
                                    apply_effect(target_type, target_name, housing_state.education.data)
                                }

                                HousingStructureTarget::Religion => {
                                    apply_effect(target_type, target_name, housing_state.religion.data)
                                }

                                HousingStructureTarget::Entertainment => {
                                    apply_effect(target_type, target_name, housing_state.entertainment.data)
                                }

                                HousingStructureTarget::HealthCare => {
                                    apply_effect(target_type, target_name, housing_state.health_care.data)
                                }
                            }
                        }

                        None => {
                            let message = format!(
                                "Housing structure effect cannot be applied to [{}]",
                                self.entity.get_props().name
                            );

                            Err(EffectError { message: Some(message) })
                        }
                    }
                }

                Effect::ProductionStructure { ref new_production_rate } => {
                    let state = self.entity.get_state();

                    match state.production {
                        Some(production_state) => {
                            self.entity.set_state(
                                StructureState {
                                    production: Some(
                                        Production {
                                            production_rate: new_production_rate,
                                            ..production_state
                                        }
                                    ),
                                    ..state
                                }
                            );

                            Ok(())
                        }

                        None => {
                            let message = format!(
                                "Production structure effect cannot be applied to [{}]",
                                self.entity.get_props().name
                            );

                            Err(EffectError { message: Some(message) })
                        }
                    }
                }

                effect => {
                    let message = format!(
                        "Effect [{}] cannot be applied to [{}]",
                        effect,
                        self.entity.get_props().name
                    );

                    Err(EffectError { message: Some(message) })
                }
            }
        }).collect();

        Ok(()) //TODO - results
    }
}
