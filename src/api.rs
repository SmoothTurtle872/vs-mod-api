pub mod v1 {
    use reqwest::Url;
    use url::ParseError;

    pub static BASE_URL: &str = "https://mods.vintagestory.at/api";
    #[derive(Debug)]
    pub enum Endpoint {
        Tags,
        GameVersions,
        Authors,
        Comments(Option<u32>),
        Mods,
        Mod(u32),
    }

    pub mod ordering {
        #[derive(Debug)]
        pub enum OrderBy {
            AssetCreated,
            LastReleased,
            Downloads,
            Follows,
            Comments,
            TrendingPoints,
        }

        #[derive(Debug)]
        pub enum OrderDirection {
            Descending,
            Ascending,
        }
    }

    impl Endpoint {
        /// Get the end of the URL which is added to the Base URL.
        /// Used in the `Endpoint::get_uri` function, but is public for debug purposes, and custom usages
        pub fn get_url_end(&self) -> String {
            match self {
                Self::Authors => "authors".to_string(),
                Self::Tags => "tags".to_string(),
                Self::GameVersions => "gameversions".to_string(),
                Self::Comments(assetid) => match assetid {
                    Some(id) => format!("comments/{}", id),
                    None => "comments".to_string(),
                },
                Self::Mods => "mods".to_string(),
                Self::Mod(id) => format!("mod/{}", id),
            }
        }

        /// Get the URI of the requested endpoint
        pub fn get_url(&self) -> Result<Url, ParseError> {
            Url::parse(&format!("{BASE_URL}/{}", self.get_url_end()))
        }
    }

    #[cfg(test)]
    mod tests {
        use url::Url;

        use super::Endpoint;

        #[test]
        fn endpoints() {
            let (tags, game_versions, authors, comments, mods, r#mod) = (
                Endpoint::Tags,
                Endpoint::GameVersions,
                Endpoint::Authors,
                Endpoint::Comments(None),
                Endpoint::Mods,
                Endpoint::Mod(6),
            );

            let (tags, game_versions, authors, comments, mods, r#mod) = (
                tags.get_url().unwrap(),
                game_versions.get_url().unwrap(),
                authors.get_url().unwrap(),
                comments.get_url().unwrap(),
                mods.get_url().unwrap(),
                r#mod.get_url().unwrap(),
            );

            assert_eq!(
                tags,
                Url::parse("http://mods.vintagestory.at/api/tags").unwrap()
            );
            assert_eq!(
                game_versions,
                Url::parse("http://mods.vintagestory.at/api/gameversions").unwrap()
            );
            assert_eq!(
                authors,
                Url::parse("http://mods.vintagestory.at/api/authors").unwrap()
            );
            assert_eq!(
                comments,
                Url::parse("http://mods.vintagestory.at/api/comments").unwrap()
            );
            assert_eq!(
                mods,
                Url::parse("http://mods.vintagestory.at/api/mods").unwrap()
            );
            assert_eq!(
                r#mod,
                Url::parse("http://mods.vintagestory.at/api/mod/6").unwrap()
            );
        }
    }
}

#[warn(incomplete_features)]
pub mod v2 {
    pub static BASE_URL: &str = "http://mods.vintagestory.at/api/v2";
}
