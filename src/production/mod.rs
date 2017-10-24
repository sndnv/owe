pub mod exchange;

use entities::Entity;
use entities::walker::WalkerProperties;
use std::fmt;

#[derive(Debug)]
pub struct Commodity {
    pub name: String,
    pub amount: u32
}

pub trait Producer {
    fn produce_commodity(&mut self, entity: &Entity) -> Option<Commodity>;
    fn produce_walker(&mut self, entity: &Entity) -> Option<WalkerProperties>;
    fn clone_boxed(&self) -> Box<Producer>;
}

impl Clone for Box<Producer> {
    fn clone(&self) -> Box<Producer> {
        self.clone_boxed()
    }
}

impl PartialEq for Producer {
    fn eq(&self, _: &Producer) -> bool {
        true //a producer defines only behavior and should not affect equality
    }
}

impl fmt::Debug for Box<Producer> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Producer {{}}")
    }
}
