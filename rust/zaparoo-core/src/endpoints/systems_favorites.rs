// Zaparoo Frontend
// Copyright (c) 2026 Wizzo Pty Ltd and the Zaparoo Project contributors.
// SPDX-License-Identifier: LicenseRef-PolyForm-Noncommercial-1.0.0
//
// `SystemsFavoritesEndpoint` — the systems list filtered to favorited media.

use crate::client::{Client, ClientError};
use crate::media_types::{SystemsParams, SystemsResult};
use crate::store::{Endpoint, Tag};
use futures_util::future::BoxFuture;
use std::sync::Arc;

#[derive(Debug)]
pub struct SystemsFavoritesEndpoint;

impl Endpoint for SystemsFavoritesEndpoint {
    type Args = ();
    type Output = SystemsResult;
    const NAME: &'static str = "SystemsFavorites";

    fn fetch(
        client: Arc<Client>,
        _args: Self::Args,
    ) -> BoxFuture<'static, Result<Self::Output, ClientError>> {
        Box::pin(async move { client.systems_favorites(SystemsParams {}).await })
    }

    fn provides(_args: &Self::Args, _output: &Self::Output) -> Vec<Tag> {
        vec![Tag::any(Self::NAME), Tag::MEDIA_DB]
    }
}
