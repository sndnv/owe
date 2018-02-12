use entities::walker::{WalkerProperties, WalkerState};

pub mod exchange;

pub struct Commodity {
    pub name: String,
    pub amount: u32,
}

pub struct CommodityProductionData {
    pub commodity: Commodity,
    pub used_commodities: Vec<Commodity>,
}

pub struct CommodityProductionError {
    pub message: Option<String>,
    pub missing_commodities: Vec<Commodity>,
}

pub type CommodityProductionResult = Result<Option<CommodityProductionData>, CommodityProductionError>;

pub struct WalkerProductionData {
    pub state: WalkerState,
    pub props: WalkerProperties,
}

pub struct WalkerProductionError {
    pub message: Option<String>,
}

pub type WalkerProductionResult = Result<Option<WalkerProductionData>, WalkerProductionError>;
