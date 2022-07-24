use std::sync::Arc;
use lazy_static::lazy_static;
use serenity::prelude::TypeMapKey;
use tokio::sync::Mutex;
use crate::RiotAPI;

pub struct Services {
    pub riot_api: RiotAPI,
}

impl Services {
    pub fn new(mut riot_api: RiotAPI) -> Services {
        return Services { riot_api };
    }
}

impl TypeMapKey for Services {
    type Value = Services;
}