// Copyright (c) RoochNetwork
// SPDX-License-Identifier: Apache-2.0

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Opt {
    /// Bind the TCP sever to this host.
    #[structopt(short, long, default_value = "127.0.0.1")]
    pub host: String,

    #[structopt(short, long, default_value = "26658")]
    pub port: u16,
}
