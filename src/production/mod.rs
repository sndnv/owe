use entities::walker::Walker;

#[derive(Debug)]
pub struct Commodity {
    name: String,
    amount: u32
}

//trait implemented on an entity that governs its commodity production
pub trait CommodityProducer {
    fn produce_commodity(&self) -> Commodity;
}

//trait implemented on an entity that governs its walker production
pub trait WalkerProducer {
    fn produce_walker(&self) -> Walker;
}
