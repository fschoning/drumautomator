use edger_bevy::bevy::asset::{AssetLoader, AsyncReadExt, LoadContext, io::Reader};
use edger_bevy::bevy::utils::BoxedFuture;

use notation_dsl::prelude::parse_get_tab;

use crate::tab::tab_asset::{TabAsset, TabError};

#[derive(Default)]
pub struct GetTabAssetLoader;

pub type LoadError = anyhow::Error;
pub type LoadResult = anyhow::Result<TabAsset, LoadError>;

impl AssetLoader for GetTabAssetLoader {
    type Asset = TabAsset;
    type Settings = ();
    type Error = LoadError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let text = String::from_utf8(bytes.to_vec())?;
        let tab_asset = match parse_get_tab(&text) {
            Ok(tab) => TabAsset::from(tab),
            Err(err) => TabAsset::from(TabError::GetTabFailed(err.to_string())),
        };
        Ok(tab_asset)
    }
    fn extensions(&self) -> &[&str] {
        &["rs"]
    }
}
