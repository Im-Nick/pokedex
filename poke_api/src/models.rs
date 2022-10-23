use rand::seq::SliceRandom;
use std::collections::HashMap;

use serde::{
    de::{self},
    Deserialize, Deserializer, Serialize,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Pokemon {
    pub name: String,
    #[serde(alias = "flavor_text_entries")]
    #[serde(deserialize_with = "deserialize_description")]
    pub description_entries: String,
    #[serde(deserialize_with = "deserialize_habitat")]
    pub habitat: String,
    pub is_legendary: bool,
    #[serde(skip_deserializing)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translation: Option<TranslationType>,
}

#[derive(Deserialize)]
struct DescriptionEntries {
    flavor_text: String,
    language: DescriptionLanguages,
}

#[derive(Deserialize)]
struct DescriptionLanguages {
    name: String,
}

fn deserialize_description<'de, D: Deserializer<'de>>(deserializer: D) -> Result<String, D::Error> {
    let mut tmp: Vec<DescriptionEntries> = Vec::deserialize(deserializer)?;
    tmp.retain(|x| x.language.name == "en");
    let random_description = tmp.choose(&mut rand::thread_rng());
    match random_description {
        Some(e) => Ok(e
            .flavor_text
            .replace(|c: char| !c.is_ascii(), "")
            .replace(|c: char| c.is_whitespace(), " ")
            .replace(r"\f", " ")),
        None => Err(de::Error::custom("Failed to fetch random description!")),
    }
}

#[derive(Deserialize)]
struct PokemonHabitat {
    name: String,
}

fn deserialize_habitat<'de, D: Deserializer<'de>>(deserializer: D) -> Result<String, D::Error> {
    let tmp: PokemonHabitat = PokemonHabitat::deserialize(deserializer)?;
    Ok(tmp.name)
}

#[derive(Serialize, Debug, Clone, Deserialize, PartialEq)]
pub enum TranslationType {
    Shakespeare,
    Yoda,
}

impl TranslationType {
    fn from(value: &str) -> Option<Self> {
        match value {
            "shakespeare" => Some(TranslationType::Shakespeare),
            "yoda" => Some(TranslationType::Yoda),
            _ => None,
        }
    }
}

#[derive(Serialize, Debug, Default, Deserialize)]
pub struct Translation {
    #[serde(deserialize_with = "deserialize_translation")]
    pub contents: TranslationContent,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct TranslationContent {
    pub translated: String,
    pub translation_type: Option<TranslationType>,
}

fn deserialize_translation<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<TranslationContent, D::Error> {
    let tmp: HashMap<String, String> = HashMap::deserialize(deserializer)?;
    let translation = tmp.get("translated");
    let translation_type: String = tmp.get("translation").unwrap_or(&"".to_owned()).to_owned();

    match translation {
        Some(e) => Ok(TranslationContent {
            translated: e.to_string(),
            translation_type: TranslationType::from(&translation_type),
        }),
        None => Err(de::Error::missing_field("translated")),
    }
}
