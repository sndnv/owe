use entities::Entity;

//trait implemented on an entity that governs its (non-entity) resource production
pub trait ResourceProducer {
    fn produce_resource(&self) -> (); //TODO - return type?
}

//trait implemented on an entity that governs its walker production
pub trait WalkerProducer {
    fn produce_walker(&self) -> Entity;
}
