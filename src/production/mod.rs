pub mod exchange;

use entities::Entity;
use entities::walker::WalkerProperties;

#[derive(Debug)]
pub struct Commodity {
    pub name: String,
    pub amount: u32
}

pub trait CommodityProducer {
    fn produce_commodity(&mut self, entity: &Entity) -> Option<Commodity>;
}

pub trait WalkerProducer {
    fn produce_walker(&mut self, entity: &Entity) -> Option<WalkerProperties>;
}
