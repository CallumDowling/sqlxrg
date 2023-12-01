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

use std::{collections::HashMap, sync::RwLock};

use lazy_static::lazy_static;
use sqlx::{Pool, MySql};
lazy_static! {
    #[derive(Debug)]
    pub static ref CONNECTION_WATER_PARK: RwLock<HashMap<String, Pool<MySql>>> =
    {
        let map:HashMap<String, Pool<MySql>> = HashMap::new();
        let lock = RwLock::new(map);
        lock
    };
}
