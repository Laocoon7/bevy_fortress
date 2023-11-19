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
pub enum FnLoaderError {
    #[error("Encountered an io error while loading asset: {0}")]
    Io(#[from] std::io::Error),
    #[error("Encountered a deserialization error while loading asset: {0}")]
    Ron(#[from] SpannedError),
}

/// Creates a new `AssetLoader` which will provide a callback for loading assets.
pub struct FnLoader<
    R: for<'a> Deserialize<'a> + Send + Sync + 'static,
    O: Asset,
    F: Fn(&mut LoadContext, R) -> O + Send + Sync + 'static,
> {
    extensions: Vec<&'static str>,
    f: F,
    _phantom: PhantomData<(R, O)>,
}
impl<
        R: for<'a> Deserialize<'a> + Send + Sync + 'static,
        O: Asset,
        F: Fn(&mut LoadContext, R) -> O + Send + Sync + 'static,
    > FnLoader<R, O, F>
{
    pub fn new(extensions: Vec<&'static str>, f: F) -> Self {
        Self {
            extensions,
            f,
            _phantom: PhantomData,
        }
    }
}

impl<
        R: for<'a> Deserialize<'a> + Send + Sync + 'static,
        O: Asset,
        F: Fn(&mut LoadContext, R) -> O + Send + Sync + 'static,
    > AssetLoader for FnLoader<R, O, F>
{
    type Asset = O;
    type Error = FnLoaderError;
    type Settings = ();

    fn extensions(&self) -> &[&str] { &self.extensions }

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a Self::Settings,
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            // Read bytes from the reader
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;

            // Deserialize to an `AtlasRecord`
            let record = ron::de::from_bytes::<R>(&bytes)?;

            Ok((self.f)(load_context, record))
        })
    }
}
