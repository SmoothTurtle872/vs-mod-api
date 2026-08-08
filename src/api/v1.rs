pub mod processed;
mod raw;

use reqwest;

static URL_BASE: &str = "https://mods.vintagestory.at/api/";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Endpoint {
    Authors,
    Mods(Option<ModSearchSettings>),
    Comments(Option<u64>),
    Tags,
    GameVersions,
    Mod(u64),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModSearchSettings {
    pub text: Option<String>,
    pub tag_ids: Option<Vec<u64>>,
    pub game_versions: Option<Vec<i64>>,
    pub author: Option<u64>,
    pub order_by: Option<OrderBy>,
    pub order_direction: Option<OrderDirection>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrderBy {
    Created,
    LastReleased,
    Downloads,
    Follows,
    Comments,
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
    pub async fn get_data(&self, client: &reqwest::Client) -> Result<raw::Data, Error> {
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

        let payload: raw::Payload =
            serde_json::from_str(&client.get(url).send().await?.text().await?)?;

        let data = payload.get_data()?;

        match self {
            Self::Authors => {
                let data: Vec<raw::data::Author> = serde_json::from_str(&data)?;
                return Ok(raw::Data::Authors(data));
            }
            Self::Tags => {
                let data: Vec<raw::data::Tag> = serde_json::from_str(&data)?;
                return Ok(raw::Data::Tags(data));
            }
            Self::GameVersions => {
                let data: Vec<raw::data::GameVersion> = serde_json::from_str(&data)?;
                return Ok(raw::Data::GameVersions(data));
            }
            Self::Comments(_) => {
                let data: Vec<raw::data::Comment> = serde_json::from_str(&data)?;
                return Ok(raw::Data::Comments(data));
            }
            Self::Mods(_) => {
                let data: Vec<raw::data::ModsListMod> = serde_json::from_str(&data)?;
                return Ok(raw::Data::Mods(data));
            }
            Self::Mod(_) => {
                let data: raw::data::Mod = serde_json::from_str(&data)?;
                return Ok(raw::Data::Mod(data));
            }
        }
    }
}

#[derive(Debug)]
pub enum Error {
    RequestError(reqwest::Error),
    JsonError(serde_json::Error),
    /// This contains the status code, however it is given as a string in the mod API
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
