// Copyright (c) RoochNetwork
// SPDX-License-Identifier: Apache-2.0

use moveos::moveos::MoveOS;
use statedb::StateDB;
use structopt::StructOpt;
use tower::ServiceBuilder;
use tower_abci::{split, Server};

use cosmoves::{moves::MoveExt, opt::Opt};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let opt = Opt::from_args();

    let db = StateDB::new();
    let moveos = MoveOS::new(db).unwrap();

    // Construct our ABCI application
    let service = MoveExt::new(moveos);

    // split it into components
    let (consensus, mempool, snapshot, info) = split::service(service, 1);

    let server = Server::builder()
        .consensus(consensus)
        .snapshot(snapshot)
        .mempool(
            ServiceBuilder::new()
                .load_shed()
                .buffer(10)
                .service(mempool),
        )
        .info(ServiceBuilder::new().load_shed().buffer(10).service(info))
        .finish()
        .unwrap();

    server
        .listen(format!("{}:{}", opt.host, opt.port))
        .await
        .unwrap()
}
