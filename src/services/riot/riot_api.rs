use serde::{Serialize, Deserialize};
use serenity::prelude::TypeMapKey;
use crate::services::util::cache_tools::{Cache, CachedValue};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NameToId {
    pub id: String,
    #[serde(rename = "accountId")]
    pub account_id: String,
    pub puuid: String,
    pub name: String,
    #[serde(rename = "profileIconId")]
    pub profile_icon_id: u32,
    #[serde(rename = "revisionDate")]
    pub revision_date: u128,
    #[serde(rename = "summonerLevel")]
    pub summoner_level: u32
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChampionMastery {
    #[serde(rename = "championId")]
    pub champion_id: u32,
    #[serde(rename = "championLevel")]
    pub champion_level: u8,
    #[serde(rename = "championPoints")]
    pub champion_points: u32,
    #[serde(rename = "lastPlayTime")]
    pub last_play_time: u128,
    #[serde(rename = "championPointsSinceLastLevel")]
    pub champion_points_since_last_level: u32,
    #[serde(rename = "championPointsUntilNextLevel")]
    pub champion_points_until_next_level: u32,
    #[serde(rename = "chestGranted")]
    pub chest_granted: bool,
    #[serde(rename = "tokensEarned")]
    pub tokens_earned: u8,
    #[serde(rename = "summonerId")]
    pub summoner_id: String,

}



// riot api modeling stuff
pub struct RiotAPI {
    client: reqwest::Client,
    api_key: String,
    info: RiotAPIInfo,
    cache: RiotAPICache,
}

/// info such as routes for the riot api
pub struct RiotAPIInfo {
    dd_url: String,
}

/// data cache for the riot api
pub struct RiotAPICache {
    game_version: CachedValue<String>,
    summoner_id_cache: Cache<String, NameToId>,
    mastery_cache: Cache<String, Vec<ChampionMastery>>,
}

impl TypeMapKey for RiotAPI {
    type Value = RiotAPI;
}

impl RiotAPI {
    /// create a new riot api instance
    pub fn new(client: reqwest::Client, api_key: String) -> RiotAPI {
        let mut api = RiotAPI{
            client,
            api_key,
            info: RiotAPIInfo {
                dd_url: String::from("https://ddragon.leagueoflegends.com")
            },
            cache: RiotAPICache {
                game_version: CachedValue::new_expired(String::new(), 1000 * 60 * 45),
                summoner_id_cache: Cache::new(1000 * 60 * 30),
                mastery_cache: Cache::new(1000 * 60 * 10)
            }
        };

        return api;
    }

    /// get the league of legends game version
    pub async fn get_game_version(&mut self) -> Result<String, reqwest::Error> {
        let cache = &mut self.cache.game_version;
        if !cache.expired() {
            return Ok(cache.value.clone());
        }

        let url = format!("{}/api/versions.json", self.info.dd_url);
        let versions: Vec<String> = self.client.get(&url)
            .send()
            .await?
            .json()
            .await?;
        return Ok(String::new())
    }

    /// get the summoner id from the given name
    pub async fn get_summoner_id(&mut self, name: &str) -> Result<NameToId, reqwest::Error> {

        let key = name.to_string();
        let cache = &mut self.cache.summoner_id_cache;

        // get a cached value if it's not expired yet
        if !cache.is_expired(&key) {
            match cache.get(&key) {
                None => {}
                Some(val) => {
                    return Ok(val.value.clone())
                }
            }
        }

        let name_to_id: NameToId = self.client.get(format!("https://na1.api.riotgames.com/lol/summoner/v4/summoners/by-name/{}", name))
            .header("X-Riot-Token", &self.api_key)
            .send()
            .await?
            .json()
            .await?;

        cache.set(key, name_to_id.clone());
        Ok(name_to_id)
    }

    /// get the mastery data from the given name
    pub async fn get_mastery_from_name(&mut self, name: &str) -> Result<Vec<ChampionMastery>, reqwest::Error>{
        let name_to_id = self.get_summoner_id(name).await.expect("Failed to get summoner id");
        return self.get_mastery(name_to_id.id.as_str()).await;
    }

    /// get the mastery information for the user
    /// with the given summoner id
    pub async fn get_mastery(&mut self, id: &str) -> Result<Vec<ChampionMastery>, reqwest::Error> {

        let key = id.to_string();
        let cache = &mut self.cache.mastery_cache;

        // get a cached value if it's not expired yet
        if !cache.is_expired(&key) {
            match cache.get(&key) {
                None => {}
                Some(val) => {
                    return Ok(val.value.clone())
                }
            }
        }

        let champion_mastery: Vec<ChampionMastery> = self.client.get(format!("https://na1.api.riotgames.com/lol/champion-mastery/v4/champion-masteries/by-summoner/{}", id))
            .header("X-Riot-Token", &self.api_key)
            .send()
            .await?
            .json()
            .await?;

        cache.set(key, champion_mastery.clone());
        Ok(champion_mastery)
    }
}