pub mod data;

use reqwest;
use std::fmt;

static URL_BASE: &str = "https://mods.vintagestory.at/api/";

/// An enum representing the possile endpoints of the VintageStory Mod Database API.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Endpoint {
    /// Returns all authors
    Authors,
    /// Returns a list of all mods, or a limited selection if settings are specified
    Mods(Option<ModSearchSettings>),
    /// Returns all comments from any asset id (Not mod id) specified. If none is specified, it returns the latest 100 comments
    Comments(Option<u64>),
    /// Returns all tags
    Tags,
    /// Returns all game versions
    GameVersions,
    /// Each mod is requested by its mod id (u64)
    /// Will result in a `Error::PayloadError("404")` if no mod is found
    Mod(u64),
}

/// The settings for `Endpoint::Mods`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModSearchSettings {
    pub text: Option<String>,
    pub tag_ids: Option<Vec<u64>>,
    pub game_versions: Option<Vec<i64>>,
    pub author: Option<u64>,
    pub order_by: Option<OrderBy>,
    pub order_direction: Option<OrderDirection>,
}

/// The method to order the results of `Endpoint::Mods`
/// If none is specified, `OrderBy::Created`
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrderBy {
    /// Sort by date created
    /// This is the default if none is specified
    Created,
    /// Sort by date 'last updated' (This is what I am assuming, I am not 100% sure)
    LastReleased,
    /// Sort by downloads
    Downloads,
    /// Sort by follows
    Follows,
    /// Sort by comment count
    Comments,
    /// Sort by trending points
    TrendingPoints,
}

impl std::fmt::Display for OrderBy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Created => write!(f, "created"),
            Self::LastReleased => write!(f, "lastreleased"),
            Self::Downloads => write!(f, "downloads"),
            Self::Follows => write!(f, "follows"),
            Self::Comments => write!(f, "comment"),
            Self::TrendingPoints => write!(f, "asset.created"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrderDirection {
    Ascending,
    Descending,
}

impl std::fmt::Display for OrderDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ascending => write!(f, "asc"),
            Self::Descending => write!(f, "desc"),
        }
    }
}

impl Endpoint {
    pub async fn get_data(&self, client: &reqwest::Client) -> Result<data::Data, Error> {
        let url = format!(
            "{URL_BASE}{}",
            match self {
                Self::Authors => "authors".to_string(),
                Self::Comments(id) => match id {
                    None => "comments".to_string(),
                    Some(id) => format!("comments/{id}"),
                },
                Self::GameVersions => "gameversions".to_string(),
                Self::Mod(id) => format!("mod/{id}"),
                Self::Mods(settings) => {
                    match settings {
                        Some(settings) => {
                            let mut base = "mods?".to_string();
                            match &settings.text {
                                None => {}
                                Some(value) => base = format!("{base}text={value}&"),
                            }
                            match &settings.author {
                                None => {}
                                Some(value) => base = format!("{base}author={value}&"),
                            }
                            match &settings.order_by {
                                None => {}
                                Some(value) => base = format!("{base}orderby={value}&"),
                            }
                            match &settings.order_direction {
                                None => {}
                                Some(value) => base = format!("{base}orderdirection={value}&"),
                            }
                            match &settings.tag_ids {
                                None => {}
                                Some(values) => {
                                    for value in values {
                                        base = format!("{base}tagids[]={value}&")
                                    }
                                }
                            }
                            match &settings.game_versions {
                                None => {}
                                Some(values) => {
                                    if values.len() == 1 {
                                        base = format!("{base}gv={}&", values[0])
                                    }
                                    for value in values {
                                        base = format!("{base}gameversions[]={value}&")
                                    }
                                }
                            }
                            base.to_string()
                        }
                        None => "mods".to_string(),
                    }
                }
                Self::Tags => "tags".to_string(),
            }
        );

        let payload: data::Payload =
            serde_json::from_str(&client.get(url).send().await?.text().await?)?;

        let data = payload.get_data()?;

        Ok(match self {
            Self::Authors => {
                let data: Result<Vec<data::types::Author>, serde_json::Error> =
                    serde_json::from_str(&data);
                let data = data?;
                data::Data::Authors(data)
            }
            Self::Tags => {
                let data: Result<Vec<data::types::Tag>, serde_json::Error> =
                    serde_json::from_str(&data);
                let data = data?;
                data::Data::Tags(data)
            }
            Self::GameVersions => {
                let data: Result<Vec<data::types::GameVersion>, serde_json::Error> =
                    serde_json::from_str(&data);
                let data = data?;
                data::Data::GameVersions(data)
            }
            Self::Comments(_) => {
                let data: Result<Vec<data::types::Comment>, serde_json::Error> =
                    serde_json::from_str(&data);
                let data = data?;
                data::Data::Comments(data)
            }
            Self::Mods(_) => {
                let data: Result<Vec<data::types::ModsListMod>, serde_json::Error> =
                    serde_json::from_str(&data);
                let data = data?;
                data::Data::Mods(data)
            }
            Self::Mod(_) => {
                let data: Result<data::types::Mod, serde_json::Error> = serde_json::from_str(&data);
                let data = data?;
                data::Data::Mod(data)
            }
        })
    }
}

/// The errors that may occur while requesting an `Endpoint`.
#[derive(Debug)]
pub enum Error {
    RequestError(reqwest::Error),
    JsonError(serde_json::Error),
    /// This contains the status code, however it is given as a string in the mod API
    /// If this is a 404 error and you requested a mod, it is likely because the mod doesn't exist
    /// TODO | Change to reqwest::StatusCode
    PayloadError(String),
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Self::RequestError(value)
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::JsonError(value)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RequestError(err) => write!(f, "Error Processing Request: {err}"),
            Self::JsonError(err) => write!(f, "Error Parsing Json: {err}"),
            Self::PayloadError(err) => write!(f, "Error in Payload, status code {err}"),
        }
    }
}

impl std::error::Error for Error {}
