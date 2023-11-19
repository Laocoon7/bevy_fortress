use std::marker::PhantomData;

use bevy::{
    asset::{io::Reader, AssetLoader, AsyncReadExt, LoadContext},
    prelude::*,
    utils::BoxedFuture,
};
use ron::error::SpannedError;
use serde::Deserialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GenericLoaderError {
    #[error("Encountered an io error while loading asset: {0}")]
    Io(#[from] std::io::Error),
    #[error("Encountered a deserialization error while loading asset: {0}")]
    Ron(#[from] SpannedError),
}

/// Creates a new `AssetLoader` which will automatically load assets.
/// All Assets used with this loader must have an `impl From<Record> for Asset`
pub struct GenericLoader<R: for<'a> Deserialize<'a> + Send + Sync + 'static + Into<O>, O: Asset> {
    extensions: Vec<&'static str>,
    _phantom: PhantomData<(R, O)>,
}
impl<R: for<'a> Deserialize<'a> + Send + Sync + 'static + Into<O>, O: Asset> GenericLoader<R, O> {
    pub fn new(extensions: Vec<&'static str>) -> Self {
        Self {
            extensions,
            _phantom: PhantomData,
        }
    }
}

impl<R: for<'a> Deserialize<'a> + Send + Sync + 'static + Into<O>, O: Asset> AssetLoader
    for GenericLoader<R, O>
{
    type Asset = O;
    type Error = GenericLoaderError;
    type Settings = ();

    fn extensions(&self) -> &[&str] { &self.extensions }

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a Self::Settings,
        _load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            // Read bytes from the reader
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;

            // Deserialize to an `AtlasRecord`
            let record = ron::de::from_bytes::<R>(&bytes)?;

            Ok(record.into())
        })
    }
}
