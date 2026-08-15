use serde::Deserialize;
use serde_json::Value;

use types::{Author, Comment, GameVersion, Mod, ModsListMod, Tag};

#[derive(Deserialize)]
pub(super) struct Payload {
    #[serde(alias = "statuscode")]
    status_code: String,
    #[serde(
        alias = "authors",
        alias = "tags",
        alias = "gameversions",
        alias = "mods",
        alias = "comments",
        alias = "mod"
    )]
    data: Option<Value>,
}

impl Payload {
    pub fn get_data(self) -> Result<String, super::Error> {
        match self.data {
            Some(data) => Ok(data.to_string()),
            None => Err(super::Error::PayloadError(self.status_code)),
        }
    }
}

/// An enum contains all of the data types, corresponding to each `Endpoint`
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Data {
    Authors(Vec<Author>),
    Tags(Vec<Tag>),
    GameVersions(Vec<GameVersion>),
    Comments(Vec<Comment>),
    Mods(Vec<ModsListMod>),
    Mod(Mod),
}

pub mod types {
    use serde::Deserialize;

    #[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
    pub struct Author {
        #[serde(alias = "userid")]
        pub user_id: u64,
        pub name: String,
    }

    #[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
    pub struct Tag {
        #[serde(alias = "tagid")]
        pub tag_id: String,
        pub name: String,
        pub color: String,
    }

    #[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
    pub struct GameVersion {
        #[serde(alias = "tagid")]
        pub tag_id: i64,
        pub name: String,
        pub color: String,
    }

    #[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
    pub struct Comment {
        #[serde(alias = "commentid")]
        pub comment_id: u64,
        #[serde(alias = "assetid")]
        pub asset_id: u64,
        #[serde(alias = "userid")]
        pub user_id: u64,
        pub text: String,
        pub created: String,
        #[serde(alias = "lastmodified")]
        pub last_modified: String,
    }

    /// This is not the same as Mod, as the api returns less data for `Endpoint::Mods` compared to `Endpoint::Mod`
    #[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
    pub struct ModsListMod {
        #[serde(alias = "modid")]
        pub mod_id: u64,
        #[serde(alias = "assetid")]
        pub asset_id: u64,
        pub downloads: u64,
        pub follows: u64,
        #[serde(alias = "trendingpoints")]
        pub trending_points: i64,
        pub comments: u64,
        pub name: String,
        pub summary: String,
        #[serde(alias = "modidstrs")]
        pub mod_id_strings: Vec<String>,
        pub author: String,
        #[serde(alias = "urlalias")]
        pub url_alias: Option<String>,
        pub side: ModSide,
        pub r#type: ModType,
        pub logo: Option<String>,
        pub tags: Vec<String>,
        #[serde(alias = "lastreleased")]
        pub last_released: String,
    }

    #[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
    pub struct Mod {
        #[serde(alias = "modid")]
        pub mod_id: u64,
        #[serde(alias = "assetid")]
        pub asset_id: u64,
        pub name: String,
        pub text: String,
        pub author: String,
        #[serde(alias = "urlalias")]
        pub url_alias: Option<String>,
        #[serde(alias = "logofilename")]
        pub logo_file_name: Option<String>,
        #[serde(alias = "logofile")]
        pub logo_file: Option<String>,
        #[serde(alias = "logofiledb")]
        pub logo_file_db: Option<String>,
        #[serde(alias = "homepageurl")]
        pub home_page_url: Option<String>,
        #[serde(alias = "sourcecodeurl")]
        pub soruce_code_url: Option<String>,
        #[serde(alias = "issuetrackerurl")]
        pub issue_tracker_url: Option<String>,
        #[serde(alias = "wikiurl")]
        pub wiki_url: Option<String>,
        pub downloads: u64,
        pub follows: u64,
        #[serde(alias = "trendingpoints")]
        pub trending_points: i64,
        pub comments: u64,
        pub side: ModSide,
        pub r#type: ModType,
        pub created: String,
        #[serde(alias = "lastreleased")]
        pub last_released: String,
        #[serde(alias = "lastmodified")]
        pub last_modified: String,
        pub tags: Vec<String>,
        pub releases: Vec<Release>,
        pub screenshots: Vec<Screenshot>,
    }

    #[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
    pub struct Release {
        #[serde(alias = "releaseid")]
        pub release_id: u64,
        #[serde(alias = "mainfile")]
        pub main_file: String,
        #[serde(alias = "filename")]
        pub file_name: String,
        #[serde(alias = "fileid")]
        pub file_id: u64,
        pub downloads: u64,
        pub tags: Vec<String>,
        #[serde(alias = "modidstr")]
        pub mod_id_string: String,
        #[serde(alias = "modversion")]
        pub mod_version: String,
        pub created: String,
        pub changelog: Option<String>,
    }

    #[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
    pub struct Screenshot {
        #[serde(alias = "fileid")]
        pub file_id: u64,
        #[serde(alias = "mainfile")]
        pub main_file: String,
        #[serde(alias = "filename")]
        pub file_name: String,
        #[serde(alias = "thumbnailfilename")]
        pub thumbnail_file_name: String,
        pub created: String,
    }

    #[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
    pub enum ModSide {
        #[serde(alias = "client")]
        Client,
        #[serde(alias = "server")]
        Server,
        #[serde(alias = "both")]
        Both,
    }

    #[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
    pub enum ModType {
        #[serde(alias = "mod")]
        Mod,
        #[serde(alias = "externaltool")]
        ExternalTool,
        #[serde(alias = "other")]
        Other,
    }
}
