// Copyright (c) RoochNetwork
// SPDX-License-Identifier: Apache-2.0

use std::{future::Future, pin::Pin, task::Poll};

use bytes::Bytes;
use futures::future::FutureExt;
use tendermint::abci::{
    response::{self, Echo},
    Event, EventAttributeIndexExt, Request, Response,
};

use tower::Service;
use tower_abci::BoxError;

use moveos::moveos::MoveOS;

pub struct MoveExt {
    pub moveos: MoveOS,
}

impl MoveExt {
    pub fn new(moveos: MoveOS) -> Self {
        Self { moveos }
    }

    pub fn info(&self) -> response::Info {
        response::Info {
            data: "tower abci kv example".to_string(),
            version: "0.1.0".to_string(),
            app_version: 1,
            last_block_height: 0u32.into(),
            last_block_app_hash: vec![0u8].try_into().unwrap(),
        }
    }

    pub fn query(&self, data: bytes::Bytes) -> response::Query {
        let key = String::from_utf8(data.to_vec()).unwrap();
        let value = "value".to_string();
        response::Query {
            log: "exists".to_string(),
            key: key.into_bytes().into(),
            value: value.into_bytes().into(),
            ..Default::default()
        }
    }

    pub fn deliver_tx(&mut self, tx: Bytes) -> response::DeliverTx {
        let tx = String::from_utf8(tx.to_vec()).unwrap();
        let tx_parts = tx.split('=').collect::<Vec<_>>();

        let (key, _value) = match (tx_parts.first(), tx_parts.get(1)) {
            (Some(key), Some(value)) => (*key, *value),
            _ => (tx.as_ref(), tx.as_ref()),
        };

        response::DeliverTx {
            events: vec![Event::new(
                "moveos",
                vec![
                    ("kev", key).index(),
                    ("index_key", "index is working").index(),
                    ("noindex_key", "index is working").index(),
                ],
            )],
            ..Default::default()
        }
    }

    pub fn commit(&mut self) -> response::Commit {
        let retain_height = 0u32.into();

        response::Commit {
            data: Bytes::new(),
            retain_height,
        }
    }
}

impl Service<Request> for MoveExt {
    type Error = BoxError;
    type Response = Response;
    type Future = Pin<Box<dyn Future<Output = Result<Response, Self::Error>> + Send + 'static>>;

    fn poll_ready(&mut self, _cx: &mut std::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request) -> Self::Future {
        tracing::info!(?req);

        let rsp = match req {
            Request::Info(_) => Response::Info(self.info()),
            Request::Query(query) => Response::Query(self.query(query.data)),
            Request::DeliverTx(tx) => Response::DeliverTx(self.deliver_tx(tx.tx)),
            Request::Commit => Response::Commit(self.commit()),

            // TODO
            Request::Flush => Response::Flush,
            Request::Echo(msg) => Response::Echo(Echo {
                message: msg.message,
            }),
            Request::InitChain(_ic) => Response::InitChain(Default::default()),
            Request::BeginBlock(_bb) => Response::BeginBlock(Default::default()),
            Request::CheckTx(_ct) => Response::CheckTx(Default::default()),
            Request::EndBlock(_eb) => Response::EndBlock(Default::default()),
            Request::ListSnapshots => Response::ListSnapshots(Default::default()),
            Request::OfferSnapshot(_os) => Response::OfferSnapshot(Default::default()),
            Request::LoadSnapshotChunk(_lsc) => Response::LoadSnapshotChunk(Default::default()),
            Request::ApplySnapshotChunk(_asc) => Response::ApplySnapshotChunk(Default::default()),
            Request::SetOption(_) => Response::SetOption(response::SetOption {
                code: tendermint::abci::Code::Ok,
                log: String::new(),
                info: String::new(),
            }),
        };

        tracing::info!(?rsp);

        async move { Ok(rsp) }.boxed()
    }
}
