use http_cache_reqwest::{Cache, CacheMode, HttpCache, MokaManager};
use reqwest::Client;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use std::collections::HashMap;

use crate::{
    errors::Error,
    models::{Pokemon, Translation},
};

#[derive(Clone)]
pub struct PokeApi {
    client: ClientWithMiddleware,
    poke_api: String,
    yoda_api: String,
    shakespeare_api: String,
}

impl PokeApi {
    pub fn new() -> Self {
        PokeApi {
            client: ClientBuilder::new(Client::new())
                .with(Cache(HttpCache {
                    mode: CacheMode::Default,
                    manager: MokaManager::default(), // In-memory cache
                    options: None,
                }))
                .build(),
            poke_api: "https://pokeapi.co/api/v2/pokemon-species".to_string(),
            yoda_api: "https://api.funtranslations.com/translate/yoda".to_string(),
            shakespeare_api: "https://api.funtranslations.com/translate/shakespeare".to_string(),
        }
    }

    pub async fn search(&self, pokemon: &str) -> Result<Pokemon, Error> {
        if pokemon.len() > 24 {
            return Err(Error::new(
                crate::errors::ErrorStatus::BadRequest,
                "Pokemon name too long".to_string(),
            ));
        };

        let url = format!(
            "{}/{}",
            self.poke_api,
            pokemon
                .trim()
                .to_lowercase()
                .split_whitespace()
                .collect::<String>()
        );

        let res = self
            .client
            .get(url)
            .send()
            .await?
            .error_for_status()?
            .json::<Pokemon>()
            .await?;
        Ok(res)
    }

    pub async fn translate<'a>(&self, pokemon: &'a mut Pokemon) -> Result<&'a Pokemon, Error> {
        // Generate json req body
        let mut body = HashMap::new();
        body.insert("text", &pokemon.description_entries);

        let url: &str = {
            if pokemon.is_legendary || pokemon.habitat == "cave" {
                &self.yoda_api
            } else {
                &self.shakespeare_api
            }
        };

        let res = self
            .client
            .post(url)
            .json(&body)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;
        let deserialized_res = serde_json::from_value::<Translation>(res);

        match deserialized_res {
            Ok(e) => {
                pokemon.description_entries = e.contents.translated;
                pokemon.translation = e.contents.translation_type;
                Ok(pokemon)
            }
            Err(e) => panic!("{:#?}", e),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{errors::ErrorStatus, models::TranslationType};

    use super::PokeApi;

    #[tokio::test]
    async fn get_pokemon() {
        let poke_api = PokeApi::new();
        let res = poke_api.search("mewtwo").await.unwrap();
        assert_eq!(res.name, "mewtwo")
    }

    #[tokio::test]
    async fn test_translate_yoda_pokemon() {
        let poke_api = PokeApi::new();
        let mut res = poke_api.search("mewtwo").await.unwrap();
        let translate = poke_api.translate(&mut res).await;

        // Handle 429
        match translate {
            Ok(pokemon) => {
                assert_eq!(
                    pokemon.translation.to_owned().unwrap(),
                    TranslationType::Yoda
                )
            }
            Err(e) => match e.status {
                ErrorStatus::TooManyRequests => assert_eq!(res.name, "mewtwo"),
                e => panic!("{:#?}", e),
            },
        }
    }

    #[tokio::test]
    async fn test_translate_shakespare_pokemon() {
        let poke_api = PokeApi::new();
        let mut res = poke_api.search("snorlax").await.unwrap();
        let translate = poke_api.translate(&mut res).await;

        // Handle 429
        match translate {
            Ok(pokemon) => {
                assert_eq!(
                    pokemon.translation.to_owned().unwrap(),
                    TranslationType::Shakespeare
                )
            }
            Err(e) => match e.status {
                ErrorStatus::TooManyRequests => assert_eq!(res.name, "snorlax"),
                e => panic!("{:#?}", e),
            },
        }
    }

    #[tokio::test]
    async fn get_pokemon_with_whitespaces() {
        let poke_api = PokeApi::new();
        let res = poke_api.search(" mew two ").await.unwrap();
        assert_eq!(res.name, "mewtwo");
    }

    #[tokio::test]
    async fn uppercase_get_pokemon() {
        let poke_api = PokeApi::new();
        let res = poke_api.search("mewtwo").await;
        assert_eq!(res.is_ok(), true)
    }

    #[tokio::test]
    async fn pokemon_max_name_length() {
        let poke_api = PokeApi::new();
        let res = poke_api
            .search("mewtwopokedexooootestcaseooooooalrigthooooooooooo")
            .await;
        assert_eq!(res.is_err(), true)
    }
}
