// Original copyright notice
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

pub mod statics;
use actix_web::{
    web,
    App, HttpServer,  middleware::Logger
};
pub mod commandline;
pub mod logic;
pub mod req_res;
use crate::commandline::parse_cli;

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    let cli = parse_cli();
    //let mut connection_pools: Mutex<HashMap<String, Pool<MySql>>> = Mutex::new(HashMap::new());
    lazy_static::initialize(&statics::CONNECTION_WATER_PARK);
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let bind_addr = format!("{}:{}", cli.bind_host, cli.port);
    println!("sqlxrg - Listening on {}", &bind_addr);
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .route("/", web::post().to(logic::handler))
    })
    .bind(bind_addr)?
    .run()
    .await
}
