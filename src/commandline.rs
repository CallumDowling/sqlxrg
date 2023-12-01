// Origininal copyright notice
// Copyright (c) 2023-, Germano Rizzo <oss /AT/ germanorizzo /DOT/ it>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// This work was inspired by sqliterg by Germano Rizzo and heavily modified to suite mysql.
// Modifications c) 2023-, Nexon Asia Pacific also under apache 2.0 license

use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
#[command(
    help_template = "{name} {version}\n {about-section}\n {usage-heading} {usage}\n {all-args} {tab}"
)]
pub struct AppConfig {
    #[arg(
        long,
        value_name = "HOST",
        default_value = "0.0.0.0",
        help = "The host to bind"
    )]
    pub bind_host: String,
    #[arg(
        short,
        long,
        value_name = "PORT",
        default_value = "12321",
        help = "Port for the web service"
    )]
    pub port: u16
   
}

pub fn parse_cli() -> AppConfig {
    let ret = AppConfig::parse();

    ret
}
