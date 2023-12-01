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

use actix_web::{error, web, HttpRequest};
use serde_json::{json, Map, Value};
use sqlx::{
    mysql::{MySqlConnectOptions, MySqlTypeInfo},
    types::chrono::{self, DateTime},
    Column, Decode, MySql, MySqlPool, Pool, Row, Transaction, TypeInfo, ValueRef,
};
use std::result::Result;

use crate::{
    req_res::{self, Response, ResponseEnum, ResponseItem},
    statics::CONNECTION_WATER_PARK,
};

#[allow(clippy::type_complexity)]
async fn do_query(
    tx: &mut Transaction<'_, MySql>,
    sql: &str,
    tag: &Option<String>,
    values: &Option<Value>,
) -> ResponseEnum {
    let mut qry = sqlx::query(sql);
    let mut arr = vec![];
    match values {
        Some(p) => match p {
            Value::Array(x) => {
                for str in x {
                    arr.push(str);
                }
                //arr.push(s.as_str());
            }
            _ => {
                return ResponseEnum::Error {
                    tag: tag.clone(),
                    error: Box::from("Values must be array."),
                };
            }
        },
        None => {}
    }

    for x in arr {
        match x {
            Value::String(s) => {
                qry = qry.bind(s);
            }
            Value::Bool(b) => {
                qry = qry.bind(b);
            }
            Value::Number(n) => {
                if let Some(x) = n.as_u64() {
                    qry = qry.bind(x);
                } else if let Some(x) = n.as_i64() {
                    qry = qry.bind(x);
                } else if let Some(x) = n.as_f64() {
                    qry = qry.bind(x);
                } else {
                    return ResponseEnum::Error {
                        tag: tag.clone(),
                        error: Box::from("Could not deserialise number field in values."),
                    };
                }
            }
            _ => {
                return ResponseEnum::Error{tag: tag.clone(), error:Box::from("Json value varient exhausted, please make sure all values are either a string, number, or boolean.")};
            }
        }
    }

    let results = match qry.fetch_all(&mut **tx).await {
        Ok(results) => results,
        Err(e) => {
            return ResponseEnum::Error {
                tag: tag.clone(),
                error: Box::from(e),
            }
        }
    };

    let mut all_rows = vec![];
    for row in results {
        let cols = row.columns();
        let mut map = Map::<String, Value>::new();
        for col in cols {
            let type_info: &MySqlTypeInfo = col.type_info();
            if type_info.is_null() {
                //May need to do something here
                //println!("Value: NULL");
            }

            //https://github.com/launchbadge/sqlx/issues/182
            let raw_value = match row.try_get_raw(col.ordinal()) {
                Ok(raw_value) => raw_value,
                Err(e) => {
                    return ResponseEnum::Error {
                        tag: tag.clone(),
                        error: Box::new(e),
                    };
                }
            };

            match raw_value.type_info().name() {
                "FLOAT" | "FLOAT4" | "FLOAT8" => {
                    match <f32 as Decode<sqlx::mysql::MySql>>::decode(raw_value) {
                        Ok(val) => {
                            map.insert(col.name().to_string(), json!(val));
                        }
                        Err(e) => {
                            return ResponseEnum::Error {
                                tag: tag.clone(),
                                error: e,
                            };
                        }
                    }
                }
                "REAL" | "NUMERIC" | "DECIMAL" | "DOUBLE" => {
                    match <f64 as Decode<sqlx::mysql::MySql>>::decode(raw_value) {
                        Ok(val) => {
                            map.insert(col.name().to_string(), json!(val));
                        }
                        Err(e) => {
                            return ResponseEnum::Error {
                                tag: tag.clone(),
                                error: e,
                            };
                        }
                    }
                }
                "INT8" | "BIGINT" | "INTEGER" => {
                    match <i64 as Decode<sqlx::mysql::MySql>>::decode(raw_value) {
                        Ok(val) => {
                            map.insert(col.name().to_string(), json!(val));
                        }
                        Err(e) => {
                            return ResponseEnum::Error {
                                tag: tag.clone(),
                                error: e,
                            };
                        }
                    }
                }
                "INT8 UNSIGNED" | "BIGINT UNSIGNED" | "INTEGER UNSIGNED" => {
                    match <u64 as Decode<sqlx::mysql::MySql>>::decode(raw_value) {
                        Ok(val) => {
                            map.insert(col.name().to_string(), json!(val));
                        }
                        Err(e) => {
                            return ResponseEnum::Error {
                                tag: tag.clone(),
                                error: e,
                            };
                        }
                    }
                }
                "INT" | "INT4" => match <i32 as Decode<sqlx::mysql::MySql>>::decode(raw_value) {
                    Ok(val) => {
                        map.insert(col.name().to_string(), json!(val));
                    }
                    Err(e) => {
                        return ResponseEnum::Error {
                            tag: tag.clone(),
                            error: e,
                        };
                    }
                },
                "INT UNSIGNED" | "INT4 UNSIGNED" => {
                    match <u32 as Decode<sqlx::mysql::MySql>>::decode(raw_value) {
                        Ok(val) => {
                            map.insert(col.name().to_string(), json!(val));
                        }
                        Err(e) => {
                            return ResponseEnum::Error {
                                tag: tag.clone(),
                                error: e,
                            };
                        }
                    }
                }
                "INT2" | "SMALLINT" => match <i16 as Decode<sqlx::mysql::MySql>>::decode(raw_value)
                {
                    Ok(val) => {
                        map.insert(col.name().to_string(), json!(val));
                    }
                    Err(e) => {
                        return ResponseEnum::Error {
                            tag: tag.clone(),
                            error: e,
                        };
                    }
                },
                "INT2 UNSIGNED" | "SMALLINT UNSIGNED" => {
                    match <u16 as Decode<sqlx::mysql::MySql>>::decode(raw_value) {
                        Ok(val) => {
                            map.insert(col.name().to_string(), json!(val));
                        }
                        Err(e) => {
                            return ResponseEnum::Error {
                                tag: tag.clone(),
                                error: e,
                            };
                        }
                    }
                }
                "INT1" | "TINYINT" => match <i8 as Decode<sqlx::mysql::MySql>>::decode(raw_value) {
                    Ok(val) => {
                        map.insert(col.name().to_string(), json!(val));
                    }
                    Err(e) => {
                        return ResponseEnum::Error {
                            tag: tag.clone(),
                            error: e,
                        };
                    }
                },
                "INT1 UNSIGNED" | "TINYINT UNSIGNED" => {
                    match <u8 as Decode<sqlx::mysql::MySql>>::decode(raw_value) {
                        Ok(val) => {
                            map.insert(col.name().to_string(), json!(val));
                        }
                        Err(e) => {
                            return ResponseEnum::Error {
                                tag: tag.clone(),
                                error: e,
                            };
                        }
                    }
                }
                "BOOL" | "BOOLEAN" => match <bool as Decode<sqlx::mysql::MySql>>::decode(raw_value)
                {
                    Ok(val) => {
                        map.insert(col.name().to_string(), json!(val));
                    }
                    Err(e) => {
                        return ResponseEnum::Error {
                            tag: tag.clone(),
                            error: e,
                        };
                    }
                },
                "DATE" => {
                    match <chrono::NaiveDate as Decode<sqlx::mysql::MySql>>::decode(raw_value) {
                        Ok(val) => {
                            map.insert(col.name().to_string(), json!(val.to_string()));
                        }
                        Err(e) => {
                            return ResponseEnum::Error {
                                tag: tag.clone(),
                                error: e,
                            };
                        }
                    }
                }
                "TIME" => {
                    match <chrono::NaiveTime as Decode<sqlx::mysql::MySql>>::decode(raw_value) {
                        Ok(val) => {
                            map.insert(col.name().to_string(), json!(val.to_string()));
                        }
                        Err(e) => {
                            return ResponseEnum::Error {
                                tag: tag.clone(),
                                error: e,
                            };
                        }
                    }
                }
                "DATETIME" | "DATETIME2" | "DATETIMEOFFSET" | "TIMESTAMP" | "TIMESTAMPTZ" => {
                    let date_time =
                        <chrono::NaiveDateTime as Decode<sqlx::mysql::MySql>>::decode(raw_value)
                            .map(|d| d.and_utc());

                    map.insert(
                        col.name().to_string(),
                        Value::String(date_time.as_ref().map_or_else(ToString::to_string, |x| {
                            DateTime::format(x, "%Y-%m-%d %H:%M:%S").to_string()
                        })),
                    );
                }
                "JSON" | "JSON[]" | "JSONB" | "JSONB[]" => {
                    match <Value as Decode<sqlx::mysql::MySql>>::decode(raw_value) {
                        Ok(val) => {
                            map.insert(col.name().to_string(), json!(val));
                        }
                        Err(e) => {
                            return ResponseEnum::Error {
                                tag: tag.clone(),
                                error: e,
                            };
                        }
                    }
                }
                // Deserialize as a string by default
                _ => match <String as Decode<sqlx::mysql::MySql>>::decode(raw_value) {
                    Ok(val) => {
                        map.insert(col.name().to_string(), json!(val));
                    }
                    Err(e) => {
                        return ResponseEnum::Error {
                            tag: tag.clone(),
                            error: e,
                        };
                    }
                },
            }
        }
        all_rows.push(json!(map));
    }
    ResponseEnum::ResponseItem {
        tag: tag.clone(),
        response_item: ResponseItem::QuerySuccess {
            result_set: all_rows,
        },
    }
}

#[allow(clippy::type_complexity)]
async fn do_single_statement(
    tx: &mut Transaction<'_, MySql>,
    sql: &str,
    tag: &Option<String>,
    values: Option<Value>,
) -> ResponseEnum {
    let mut stmt = sqlx::query(sql);
    let mut arr: Vec<Value> = vec![];

    match values {
        Some(p) => match p {
            Value::Array(x) => {
                for str in x {
                    arr.push(str);
                }
                //arr.push(s.as_str());
            }
            _ => {
                return ResponseEnum::Error {
                    tag: tag.clone(),
                    error: Box::from("Values must be array."),
                };
            }
        },
        None => {}
    }

    for x in arr {
        match x {
            Value::String(s) => {
                stmt = stmt.bind(s);
            }
            Value::Bool(b) => {
                stmt = stmt.bind(b);
            }
            Value::Number(n) => {
                if let Some(x) = n.as_u64() {
                    stmt = stmt.bind(x);
                } else if let Some(x) = n.as_i64() {
                    stmt = stmt.bind(x);
                } else if let Some(x) = n.as_f64() {
                    stmt = stmt.bind(x);
                } else {
                    return ResponseEnum::Error {
                        tag: tag.clone(),
                        error: Box::from("Could not deserialise number field in values."),
                    };
                }
            }
            _ => {
                return ResponseEnum::Error{tag: tag.clone(), error:Box::from("Json value varient exhausted, please make sure all values are either a string, number, or boolean.")};
            }
        }
    }

    match stmt.execute(&mut **tx).await {
        Ok(results) => ResponseEnum::ResponseItem {
            tag: tag.clone(),
            response_item: ResponseItem::StatementSuccess {
                rows_affected: results.rows_affected(),
                last_insert_id: results.last_insert_id(),
            },
        },
        Err(e) => ResponseEnum::Error {
            tag: tag.clone(),
            error: Box::from(e),
        },
    }
}

#[allow(clippy::type_complexity)]
async fn do_statements(
    tx: &mut Transaction<'_, MySql>,
    sql: &str,
    tag: &Option<String>,
    values_batch: Vec<Value>,
) -> Vec<ResponseEnum> {
    let mut responses = vec![];

    if values_batch.len() == 0 {
        responses.push(do_single_statement(tx, sql, tag, None).await);
    } else {
        for value in values_batch {
            responses.push(do_single_statement(tx, sql, tag, Some(value)).await);
        }
    }
    responses
}

async fn process(
    http_req: web::Json<req_res::Request>,
    pool: &Pool<MySql>,
) -> Result<Response, actix_web::Error> {
    let mut tx = pool
        .begin()
        .await
        .map_err(|err| error::ErrorInternalServerError(err.to_string()))?;
    //let mut results = vec![];
    let mut responses = vec![];
    for (_, trx_item) in http_req.transaction.iter().enumerate() {
        if trx_item.query.is_some() && trx_item.statement.is_some() {
            return Err(error::ErrorBadRequest(
                "exactly one of 'query' and 'statement' must be provided",
            ));
        }

        let values_batch = match (&trx_item.values, &trx_item.values_batch) {
            (None, None) => vec![],
            (None, Some(y)) => y.clone(),
            (Some(x), None) => vec![x.clone()],
            (Some(_), Some(_)) => {
                return Err(error::ErrorBadRequest(
                    "at most one of values and values_batch must be provided",
                ));
            }
        };

        if let Some(query) = &trx_item.query {
            let result = do_query(&mut tx, query, &trx_item.tag, &trx_item.values).await;

            match result {
                ResponseEnum::ResponseItem {
                    tag: _,
                    response_item: _,
                } => {
                    responses.push(result);
                }
                ResponseEnum::Error { tag: _, error: _ } => {
                    responses.push(result);
                    println!("Rollin back");
                    tx.rollback().await.unwrap();
                    return Ok(Response {
                        results: Some(responses),
                    });
                }
            }
        } else {
            if let Some(statement_text) = trx_item.statement.as_ref() {
                //do_query(&mut tx, query, &trx_item.values).await?
                //Turn values into value batck of 1
                let mut statment_results =
                    do_statements(&mut tx, statement_text, &trx_item.tag, values_batch).await;

                responses.append(&mut statment_results);

                if let Some(x) = responses.iter().rev().next() {
                    if let ResponseEnum::Error { tag: _, error: _ } = x {
                        println!("Rollin back");
                        tx.rollback().await.unwrap_or(());
                        return Ok(Response {
                            results: Some(responses),
                        });
                    }
                }
            }
        }
    }
    println!("Commitin");

    tx.commit().await.unwrap_or(());
    Ok(Response {
        results: Some(responses),
    })
}

pub async fn handler(
    req: HttpRequest,
    body: web::Json<req_res::Request>,
) -> Result<Response, actix_web::Error> {
    let _ = req
        .headers()
        .get("content-type")
        .ok_or(error::ErrorBadRequest("content-type header missing"))?
        .to_str()
        .map_err(|_| error::ErrorBadRequest("could not decode content-type into ascii bytes"))?
        .eq("application/json")
        .then(|| true)
        .ok_or(error::ErrorBadRequest("could not decode content-type into ascii bytes"));

    let connection_string = req
        .headers()
        .get("connection-string")
        .ok_or(error::ErrorBadRequest("connection-string header missing"))?
        .to_str()
        .map_err(|_| error::ErrorBadRequest("could not decode content-type into ascii bytes"))?;

    if let Ok(lock) = CONNECTION_WATER_PARK.try_read() {
        if let Some(pool) = lock.get(connection_string) {
            //Should check if pool actually works here, the remove it from hashmap if it doesn't. Eg changed password.
            return process(body, pool).await;
        } else {
            //Release read lock
            drop(lock);
            if let Ok(mut lock) = CONNECTION_WATER_PARK.try_write() {
                let opts: MySqlConnectOptions = connection_string
                    .parse::<MySqlConnectOptions>()
                    .map_err(|err| error::ErrorBadRequest(err.to_string()))?;
                if let Ok(pool) = MySqlPool::connect_with(opts).await {
                    lock.insert(connection_string.to_string(), pool);
                    return process(body, lock.get(connection_string).unwrap()).await;
                } else {
                    return Err(error::ErrorBadRequest("Connection string failed."));
                }
            } else {
                return Err(error::ErrorInternalServerError(
                    "Could not obtain write lock on water park.",
                ));
            }
        }
    } else {
        // Could not get read lock
        return Err(error::ErrorInternalServerError(
            "Could not get water park read lock",
        ));
    }
}
