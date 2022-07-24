use serde::{Serialize, Deserialize};
use serenity::prelude::TypeMapKey;
use crate::services::util::cache_tools::{Cache};


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
    summoner_id_cache: Cache<String, NameToId>,
    mastery_cache: Cache<String, Vec<ChampionMastery>>
}

impl TypeMapKey for RiotAPI {
    type Value = RiotAPI;
}

impl RiotAPI {

    /// create a new riot api instance
    pub fn new(client: reqwest::Client, api_key: String) -> RiotAPI {
        return RiotAPI{
            client,
            api_key,
            summoner_id_cache: Cache::new(1000 * 60 * 30),
            mastery_cache: Cache::new(1000 * 60 * 10)
        }
    }

    /// get the summoner id from the given name
    pub async fn get_summoner_id(&mut self, name: &str) -> Result<NameToId, reqwest::Error> {

        let key = name.to_string();

        // get a cached value if it's not expired yet
        if !self.summoner_id_cache.is_expired(&key) {
            match self.summoner_id_cache.get(&key) {
                None => {}
                Some(val) => {
                    return Ok(val.value.clone())
                }
            }
        }

        let name_to_id = NameToId {
            id: "".to_string(),
            account_id: "".to_string(),
            puuid: "".to_string(),
            name: "".to_string(),
            profile_icon_id: 0,
            revision_date: 0,
            summoner_level: 0
        };

        let name_to_id: NameToId = self.client.get(format!("https://na1.api.riotgames.com/lol/summoner/v4/summoners/by-name/{}", name))
            .header("X-Riot-Token", &self.api_key)
            .json(&name_to_id)
            .send()
            .await?
            .json()
            .await?;

        self.summoner_id_cache.set(key, name_to_id.clone());
        Ok(name_to_id)
    }

    pub async fn get_mastery_from_name(&mut self, name: &str) -> Result<Vec<ChampionMastery>, reqwest::Error>{
        let name_to_id = self.get_summoner_id(name).await.expect("Failed to get summoner id");
        return self.get_mastery(name_to_id.id.as_str()).await;
    }
    pub async fn get_mastery(&mut self, id: &str) -> Result<Vec<ChampionMastery>, reqwest::Error> {

        let key = id.to_string();

        // get a cached value if it's not expired yet
        if !self.mastery_cache.is_expired(&key) {
            match self.mastery_cache.get(&key) {
                None => {}
                Some(val) => {
                    return Ok(val.value.clone())
                }
            }
        }

        let champion_mastery: Vec<ChampionMastery> = Vec::new();

        let champion_mastery: Vec<ChampionMastery> = self.client.get(format!("https://na1.api.riotgames.com/lol/champion-mastery/v4/champion-masteries/by-summoner/{}", id))
            .header("X-Riot-Token", &self.api_key)
            .json(&champion_mastery)
            .send()
            .await?
            .json()
            .await?;

        self.mastery_cache.set(key, champion_mastery.clone());
        Ok(champion_mastery)
    }
}