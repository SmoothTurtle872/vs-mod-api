pub mod v1 {
    use http::{Uri, uri::InvalidUri};

    pub static BASE_URL: &str = "http://mods.vintagestory.at/api";
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
        pub fn get_uri(&self) -> Result<Uri, InvalidUri> {
            let url = format!("{BASE_URL}/{}", self.get_url_end());
            url.parse::<Uri>()
        }
    }

    #[cfg(test)]
    mod tests {
        use http::Uri;

        use crate::api::v1::Endpoint;

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
                tags.get_uri().unwrap(),
                game_versions.get_uri().unwrap(),
                authors.get_uri().unwrap(),
                comments.get_uri().unwrap(),
                mods.get_uri().unwrap(),
                r#mod.get_uri().unwrap(),
            );

            assert_eq!(
                tags,
                Uri::builder()
                    .scheme("http")
                    .authority("mods.vintagestory.at")
                    .path_and_query("/api/tags")
                    .build()
                    .unwrap()
            );
            assert_eq!(
                game_versions,
                Uri::builder()
                    .scheme("http")
                    .authority("mods.vintagestory.at")
                    .path_and_query("/api/gameversions")
                    .build()
                    .unwrap()
            );
            assert_eq!(
                authors,
                Uri::builder()
                    .scheme("http")
                    .authority("mods.vintagestory.at")
                    .path_and_query("/api/authors")
                    .build()
                    .unwrap()
            );
            assert_eq!(
                comments,
                Uri::builder()
                    .scheme("http")
                    .authority("mods.vintagestory.at")
                    .path_and_query("/api/comments")
                    .build()
                    .unwrap()
            );
            assert_eq!(
                mods,
                Uri::builder()
                    .scheme("http")
                    .authority("mods.vintagestory.at")
                    .path_and_query("/api/mods")
                    .build()
                    .unwrap()
            );
            assert_eq!(
                r#mod,
                Uri::builder()
                    .scheme("http")
                    .authority("mods.vintagestory.at")
                    .path_and_query("/api/mod/6")
                    .build()
                    .unwrap()
            );
        }
    }
}

#[warn(incomplete_features)]
pub mod v2 {
    pub static BASE_URL: &str = "http://mods.vintagestory.at/api/v2";
}
