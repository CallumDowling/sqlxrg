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

use actix_web::{
    body::BoxBody,
    http::{header::ContentType, StatusCode},
    HttpRequest, HttpResponse, Responder,
};
use serde::{
    ser::SerializeMap,
    Deserialize, Serialize, Serializer,
};

use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct ReqTransactionItem {
    pub query: Option<String>,
    pub statement: Option<String>,
    pub values: Option<Value>,
    #[serde(rename = "valuesBatch")]
    pub values_batch: Option<Vec<Value>>,
    pub tag: Option<String>
}

#[derive(Debug, Deserialize)]
pub struct Request {
    pub transaction: Vec<ReqTransactionItem>
}

#[derive(Debug)]
pub enum ResponseItem {
    //Contains results set
    QuerySuccess{result_set: Vec<Value>},
    //Rows updated
    StatementSuccess{rows_affected: u64, last_insert_id:u64}
}

#[derive(Debug)]
pub enum ResponseEnum {
    ResponseItem{tag: Option<String>, response_item: ResponseItem},
    Error{tag: Option<String>, error: Box<dyn std::error::Error>}
}

impl Serialize for ResponseEnum {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            ResponseEnum::ResponseItem{ tag, response_item} => match response_item {
                ResponseItem::QuerySuccess{result_set} => {
                    let mut map = serializer.serialize_map(Some(2))?;
                    if let Some(tag) = tag {
                        map.serialize_entry("tag", tag)?;
                    }
                    map.serialize_entry("success", "true")?;
                    map.serialize_entry("resultsSet", result_set)?;
                    map.end()
                }
                ResponseItem::StatementSuccess{rows_affected, last_insert_id} => {
                    let mut map = serializer.serialize_map(Some(2))?;
                    if let Some(tag) = tag {
                        map.serialize_entry("tag", tag)?;
                    }
                    map.serialize_entry("success", "true")?;
                    map.serialize_entry("rowsAffected", rows_affected)?;
                    map.serialize_entry("lastInsertId", last_insert_id)?;
                    map.end()
                }
            },
            ResponseEnum::Error{tag, error} => {
                let mut map = serializer.serialize_map(Some(2))?;
                if let Some(tag) = tag {
                    map.serialize_entry("tag", tag)?;
                }
                map.serialize_entry("success", "false")?;
                map.serialize_entry("error", &error.to_string())?;
                map.end()
            }
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Response {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<ResponseEnum>>,
}

impl Responder for Response {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        if let Some(items) = &self.results {
            if items
                .iter()
                .any(|x| matches!(x, ResponseEnum::Error{..}))
            {
                return HttpResponse::InternalServerError()
                    .content_type(ContentType::json())
                    .body(serde_json::to_string(&self).unwrap());
            } else {
                return HttpResponse::Ok()
                    .status(StatusCode::from_u16(200).unwrap())
                    .content_type(ContentType::json())
                    .body(serde_json::to_string(&self).unwrap());
            }
        } else {
            // No results should not be possible
            return HttpResponse::InternalServerError()
                .content_type(ContentType::json())
                .body(serde_json::to_string(&self).unwrap());
        }
    }
}