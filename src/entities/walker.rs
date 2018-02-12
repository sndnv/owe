use actix::{Actor, Context, Handler, ResponseType, SyncAddress};
use effects::{Effect, EffectError, ProcessEffects};
use entities::{Entity, EntityActor};
use map::{ProcessTick, TickError, TickResult};
use std::collections::HashMap;

#[derive(PartialEq, Clone, Debug)]
pub struct WalkerCommodities {
    pub data: HashMap<String, u32>,
}

#[derive(PartialEq, Clone, Debug)]
pub struct WalkerModifiers {
    pub interaction_distance: u8,
    pub movement_speed: u8,
    pub patrol: u8,
    pub max_life: u8,
    pub attack_rate: u8,
    pub attack_damage: u8,
}

#[derive(PartialEq, Clone, Debug)]
pub struct WalkerProperties {
    pub name: String,
    pub interaction_distance: u16,
    pub movement_speed: u16,
    pub patrol: Option<u16>,
    pub max_life: Option<u16>,
    pub attack_rate: Option<u8>,
    pub attack_damage: Option<u8>,
    pub effects: Vec<Effect>,
}

#[derive(PartialEq, Clone, Debug)]
pub struct WalkerState {
    pub modifiers: WalkerModifiers,
    pub current_life: Option<u16>,
    pub commodities: Option<WalkerCommodities>,
}

impl WalkerModifiers {
    pub fn default() -> WalkerModifiers {
        WalkerModifiers {
            interaction_distance: 100,
            movement_speed: 100,
            patrol: 100,
            max_life: 100,
            attack_rate: 100,
            attack_damage: 100,
        }
    }
}

impl WalkerCommodities {
    pub fn default() -> WalkerCommodities {
        WalkerCommodities {
            data: HashMap::new(),
        }
    }
}

pub trait Walker: Entity<WalkerProperties, WalkerState> {}

impl<T: Walker> Handler<ProcessTick> for EntityActor<T> {
    type Result = Result<TickResult, TickError>;

    fn handle(&mut self, msg: ProcessTick, _: &mut Context<Self>) -> Self::Result {
        //TODO - movement
        //TODO - interaction

        Ok(TickResult::Walker {}) //TODO - result
    }
}

impl<T: Walker> Handler<ProcessEffects> for EntityActor<T> {
    type Result = Result<(), EffectError>;

    fn handle(&mut self, msg: ProcessEffects, _: &mut Context<Self>) -> Self::Result {
        msg.effects.iter().for_each(|effect| {
            match effect {
                Effect::Walker { ref new_modifiers } => {
                    self.entity.set_state(
                        WalkerState {
                            modifiers: new_modifiers,
                            ..self.walker.get_state()
                        }
                    )
                }

                _ => () //do nothing
            }
        }); //TODO - results

        Ok(())
    }
}
