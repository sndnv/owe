use actix::{Actor, Context, Handler, ResponseType};
use effects::{Effect, EffectError, ProcessEffects};
use entities::{Entity, EntityActor};
use map::{ProcessTick, TickError, TickResult};
use production::CommodityProductionResult;

#[derive(PartialEq, Clone, Debug)]
pub struct ResourceProperties {
    pub name: String,
    pub max_amount: u32,
    pub replenish_amount: Option<u32>,
}

#[derive(PartialEq, Clone, Debug)]
pub struct ResourceState {
    pub current_amount: u32,
    pub production_rate: u8,
}

pub trait Resource: Entity<ResourceProperties, ResourceState> {
    fn produce_commodity(&mut self) -> CommodityProductionResult;
}

impl<T: Resource> Handler<ProcessTick> for EntityActor<T> {
    type Result = Result<TickResult, TickError>;

    fn handle(&mut self, msg: ProcessTick, _: &mut Context<Self>) -> Self::Result {
        let commodity_result = self.entity.produce_commodity();

        match commodity_result {
            Ok(commodity_data) => Ok(TickResult::Resource { commodity_data }),
            Err(commodity_error) => Err(TickError::Resource { commodity_error }),
        }
    }
}

impl<T: Resource> Handler<ProcessEffects> for EntityActor<T> {
    type Result = Result<(), EffectError>;

    fn handle(&mut self, msg: ProcessEffects, _: &mut Context<Self>) -> Self::Result {
        msg.effects.iter().for_each(|effect| {
            match effect {
                Effect::Resource { ref new_production_rate } => {
                    self.entity.set_state(
                        ResourceState {
                            production_rate: new_production_rate,
                            ..self.resource.get_state()
                        }
                    )
                }

                _ => () //do nothing
            }
        }); //TODO - results

        Ok(())
    }
}
