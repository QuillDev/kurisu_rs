use serde::{Serialize, Deserialize};

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