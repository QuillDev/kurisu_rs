use std::path::Path;
use reqwest::Error;
use serenity::prelude::TypeMapKey;
use crate::download_file;
use crate::services::riot::models::{ChampionMastery, NameToId};
use crate::services::util::cache_tools::{Cache, CachedValue};

/// type map for the league api
impl TypeMapKey for LeagueAPI {
    type Value = LeagueAPI;
}

// riot api modeling stuff
pub struct LeagueAPI {
    client: reqwest::Client,
    api_key: String,
    info: LeagueApiInfo,
    cache: LeagueApiCache,
}

/// info such as routes for the riot api
struct LeagueApiInfo {
    dd_url: String,
}

/// data cache for the riot api
struct LeagueApiCache {
    game_version: CachedValue<String>,
    summoner_id_cache: Cache<String, NameToId>,
    mastery_cache: Cache<String, Vec<ChampionMastery>>,
}

impl LeagueAPI {
    /// create a new riot api instance
    pub fn new(client: reqwest::Client, api_key: String) -> LeagueAPI {
        let api = LeagueAPI {
            client,
            api_key,
            // info for endpoints in the riot api
            info: LeagueApiInfo {
                dd_url: String::from("https://ddragon.leagueoflegends.com")
            },
            // cache info for stuff from the riot api
            cache: LeagueApiCache {
                game_version: CachedValue::new_expired(String::new(), 1000 * 60 * 45),
                summoner_id_cache: Cache::new(1000 * 60 * 30),
                mastery_cache: Cache::new(1000 * 60 * 10),
            },
        };

        return api;
    }

    /// get the league of legends game version
    pub async fn get_game_version(&mut self) -> Result<String, reqwest::Error> {
        let cache = &mut self.cache.game_version;
        if !cache.expired() {
            return Ok(cache.value.clone());
        }

        // make the request to get the version
        let url = format!("{}/api/versions.json", self.info.dd_url);
        let versions: Vec<String> = self.client.get(&url)
            .send()
            .await?
            .json()
            .await?;

        let version = versions.get(0).expect("Did not get version information");
        return Ok(version.to_string().clone());
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
                    return Ok(val.value.clone());
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

    /// get the latest data dragon data from riot
    pub async fn get_latest_dd_data(&mut self) -> Result<String, String> {
        return match self.get_game_version().await {
            Ok(ver) => {
                // check if the file exists
                let path_string = format!("./cache/{}.tgz", ver);
                let path = Path::new(&path_string);
                if path.exists() {
                    return Ok(String::from("File already downloaded."));
                }

                // download the file
                let download = download_file(&self.client, format!("{}/cdn/dragontail-{}.tgz", self.info.dd_url, ver).as_str(), path).await;
                return match download {
                    Ok(_) => Ok(String::from("File downloaded!")),
                    Err(err) => Err(err),
                };
            }
            Err(err) => Err(err.to_string())
        };
    }

    /// get the mastery data from the given name
    pub async fn get_mastery_from_name(&mut self, name: &str) -> Result<Vec<ChampionMastery>, Error> {
        return match self.get_summoner_id(name).await {
            Ok(data) => self.get_mastery(data.id.as_str()).await,
            Err(err) => Err(err)
        };
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
                    return Ok(val.value.clone());
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