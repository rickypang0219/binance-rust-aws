use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct ListenKey {
    pub listenKey: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct ExchangeInfo {
    pub exchangeFilters: Vec<String>, // Adjust type as necessary
    pub rateLimits: Vec<RateLimit>,
    pub serverTime: u64,
    pub assets: Vec<Asset>,
    pub symbols: Vec<Symbol>,
    pub timezone: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct RateLimit {
    pub interval: String,
    pub intervalNum: u32,
    pub limit: u32,
    pub rateLimitType: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Asset {
    pub asset: String,
    pub marginAvailable: bool,
    pub autoAssetExchange: String, // Nullable field
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Symbol {
    pub symbol: String,
    pub pair: String,
    pub contractType: String,
    pub deliveryDate: u64,
    pub onboardDate: u64,
    pub status: String,
    pub maintMarginPercent: String,
    pub requiredMarginPercent: String,
    pub baseAsset: String,
    pub quoteAsset: String,
    pub marginAsset: String,
    pub pricePrecision: u32,
    pub quantityPrecision: u32,
    pub baseAssetPrecision: u32,
    pub quotePrecision: u32,
    pub underlyingType: String,
    pub underlyingSubType: Vec<String>,
    pub settlePlan: Option<u32>,
    pub triggerProtect: String,
    pub filters: Vec<Filters>,
    pub orderTypes: Vec<String>,
    pub timeInForce: Vec<String>,
    pub liquidationFee: String,
    pub marketTakeBound: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "filterType")]
pub enum Filters {
    #[serde(rename = "PRICE_FILTER")]
    #[serde(rename_all = "camelCase")]
    PriceFilter {
        min_price: String,
        max_price: String,
        tick_size: String,
    },
    #[serde(rename = "LOT_SIZE")]
    #[serde(rename_all = "camelCase")]
    LotSize {
        max_qty: String,
        min_qty: String,
        step_size: String,
    },
    #[serde(rename = "MARKET_LOT_SIZE")]
    #[serde(rename_all = "camelCase")]
    MarketLotSize {
        max_qty: String,
        min_qty: String,
        step_size: String,
    },
    #[serde(rename = "MAX_NUM_ORDERS")]
    #[serde(rename_all = "camelCase")]
    MaxNumOrder { limit: u64 },
    #[serde(rename = "MAX_NUM_ALGO_ORDERS")]
    #[serde(rename_all = "camelCase")]
    MaxNumAlgoOrders { limit: u64 },
    #[serde(rename = "MIN_NOTIONAL")]
    #[serde(rename_all = "camelCase")]
    MinNotional { notional: String },

    #[serde(rename = "PERCENT_PRICE")]
    #[serde(rename_all = "camelCase")]
    PercentPrice {
        multiplier_up: String,
        multiplier_down: String,
        multiplier_decimal: String,
    },

    #[serde(rename = "POSITION_RISK_CONTROL")]
    #[serde(rename_all = "camelCase")]
    PositionRiskControl { position_control_side: String },
}
