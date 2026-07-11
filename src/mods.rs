use reqwest::Url;
use time::UtcDateTime;

pub struct Mod {
    pub modid: u32,
    pub assetid: u32,
    pub name: String,
    pub text: String,
    pub author: String,
    pub url_alias: Option<Url>,
    pub logo_file_name: Option<String>,
    pub logo_file: Option<String>,
    pub logo_file_db: Option<String>,
    pub homepage_url: Option<Url>,
    pub sourcecode_url: Option<Url>,
    pub trailer_video_url: Option<Url>,
    pub issue_tracker_url: Option<Url>,
    pub wiki_url: Option<Url>,
    pub downloads: u32,
    pub follows: u32,
    pub trendingpoints: u32,
    pub comments: u32,
    pub side: String,
    pub r#type: String,
    pub created: UtcDateTime,
    pub last_released: UtcDateTime,
    pub last_modified: UtcDateTime,
    pub tags: Vec<String>,
    pub releases: Vec<Release>,
    pub screenshots: Vec<Screenshot>,
}

pub struct Release {
    pub releaseid: u32,
    pub main_file: Url,
    pub file_name: String,
    pub fileid: u32,
    pub downloads: u32,
    pub tags: Vec<String>,
    pub modid_str: String,
    pub mod_version: String,
    pub created: UtcDateTime,
    pub changelog: String,
}
pub struct Screenshot {
    pub filid: u32,
    pub main_file: Url,
    pub file_name: String,
    pub thumbnail_filname: String,
    pub created: UtcDateTime,
}
