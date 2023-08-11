// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use async_trait::async_trait;
use bytes::Bytes;
use http::StatusCode;

use super::backend::VercelArtifactsBackend;
use super::error::parse_error;
use crate::raw::*;
use crate::*;

pub struct VercelArtifactsWriter {
    backend: VercelArtifactsBackend,
    op: OpWrite,

    path: String,
}

impl VercelArtifactsWriter {
    pub fn new(backend: VercelArtifactsBackend, op: OpWrite, path: String) -> Self {
        VercelArtifactsWriter { backend, op, path }
    }
}

#[async_trait]
impl oio::Write for VercelArtifactsWriter {
    async fn write(&mut self, bs: Bytes) -> Result<()> {
        let resp = self
            .backend
            .vercel_artifacts_put(
                self.path.as_str(),
                self.op.content_length().unwrap(),
                AsyncBody::Bytes(bs),
            )
            .await?;

        let status = resp.status();

        match status {
            StatusCode::OK | StatusCode::ACCEPTED => {
                resp.into_body().consume().await?;
                Ok(())
            }
            _ => Err(parse_error(resp).await?),
        }
    }

    async fn sink(&mut self, _size: u64, _s: oio::Streamer) -> Result<()> {
        Err(Error::new(
            ErrorKind::Unsupported,
            "Write::sink is not supported",
        ))
    }

    async fn abort(&mut self) -> Result<()> {
        Ok(())
    }

    async fn close(&mut self) -> Result<()> {
        Ok(())
    }
}