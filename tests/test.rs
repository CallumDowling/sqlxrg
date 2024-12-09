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




//Assumes the existence of a .env file with CARGO_TEST_CONNECTION_STRING
//Assumes the existence of a mariadb instance on port 3306 with an account that has appropriate grants corresponding to connection string

#[cfg(test)]
mod tests {

    extern crate dotenv;
    use actix_web::{test, web, App, http::header::ContentType};
    use serde_json::json;
    use sqlxrg::{logic, statics};
    use dotenv::dotenv;
    use dotenv_codegen::dotenv;


    #[actix_web::test]
    async fn test_all() {

        lazy_static::initialize(&statics::CONNECTION_WATER_PARK);
        dotenv().ok();
        
        let test_connection_string = dotenv!("CARGO_TEST_CONNECTION_STRING");
        let app = test::init_service(
            App::new()
                .route("/test", web::post().to(logic::handler))
        )
        .await;
//cargo test -- --nocapture  
//to preserve printlns
        let payload_0 = 
            json!({
                "transaction": [
                    {
                        "statement": "DROP DATABASE IF EXISTS rust_test;",
                        "tag": "drop_db"
                    },
                    {
                        "statement": "CREATE DATABASE rust_test;"
                    },
                    {
                        "query": "show databases;"
                    },
                    {
                        "statement": "USE rust_test;",
                        "tag": "change_db"
                    },
                    {
                        "statement": "CREATE TABLE test (
                            _id INT NOT NULL AUTO_INCREMENT,
                            _bool BOOL,
                            _tiny_int TINYINT,
                            _int INT, 
                            _big_int BIGINT,
                            _big_int_unsigned BIGINT UNSIGNED,
                            _float FLOAT,
                            _double DOUBLE,
                            _text TEXT,
                            _varchar_5 VARCHAR(5),
                            _date DATE,
                            _date_time DATETIME,
                            _time_stamp TIMESTAMP,
                            PRIMARY KEY (_id)
                        );"
                    },
                ]
            });
            
        let payload_1 = 
            json!({
                "transaction": [
                {
                    "statement": "INSERT INTO test (_bool, _tiny_int, _int, _big_int, _big_int_unsigned, _float, _double, _text, _varchar_5, _date, _date_time, _time_stamp) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
                    "values": [true, -100, -2147483648, -9223372036854775808i64, 18446744073709551615u64, 2.4, 4.2, "test", "abcde", "2008-07-04", "2023-11-29 14:03:15", "2023-11-30 00:03:15"]
                },
                {
                    "query": "SELECT _id, _bool, _tiny_int, _int, _big_int, _big_int_unsigned, _float, _double, _text, _varchar_5, _date, _date_time, _time_stamp FROM test;"
                },
                {   
                    "statement": "UPDATE test set _id=4 WHERE _text LIKE '%es%';"
                },
                {   
                    "tag": "fail on purpose",
                    "query": "UPDATE test set _id=4 WHERE _not_present=abc;"
                },
                ]
            });

        let payload_2 = 
            json!({
                "transaction": [
                {
                    "statement": "CREATE TABLE left_join_test_table_1 (_id_1 INT NOT NULL, _test_1 TEXT);"
                },
                {
                    "statement": "CREATE TABLE left_join_test_table_2 (_id_2 INT NOT NULL, _test_2 TEXT);"
                },
                {
                    "statement": "INSERT INTO left_join_test_table_2 (_id_2, _test_2) VALUES (1, 'should be present tbl2');"
                },
                {
                    "statement": "INSERT INTO left_join_test_table_2 (_id_2, _test_2) VALUES (2, 'should be present tbl2');"
                },

                {
                    "statement": "INSERT INTO left_join_test_table_1 (_id_1, _test_1) VALUES (1, 'should be present tbl1');"
                },
                {
                    "statement": "INSERT INTO left_join_test_table_1 (_id_1, _test_1) VALUES (3, 'should be absent');"
                },
                {
                    "query": "SELECT * FROM left_join_test_table_2 LEFT JOIN left_join_test_table_1 ON left_join_test_table_2._id_2=left_join_test_table_1._id_1;"
                },
                ]
            });

        //using null in values
        let payload_3 = 
            json!({
                "transaction": [
                {
                    "statement": "CREATE TABLE null_test (_id_1 INT NOT NULL, _test_1 TEXT NULL, _test_2 INT NULL);"
                },
                {
                    "statement": "INSERT INTO null_test(_id_1, _test_1, _test_2) VALUES(0, ?, ?);",
                    "values": ["test", 5]
                },
                {
                    "statement": "INSERT INTO null_test(_id_1, _test_1, _test_2) VALUES(1, ?, ?);",
                    "values": [null, null]
                },
                {
                    "query": "SELECT * FROM null_test;"
                },
                ]
            });

        let req_0 = test::TestRequest::post()
            .uri("/test")
            .insert_header(ContentType::json())
            .insert_header(("connection-string", format!("{}/rust_test",test_connection_string)))
            .set_json(payload_0)
            .to_request();
        let resp_0 = test::call_service(&app, req_0).await;

        let req_1 = test::TestRequest::post()
            .uri("/test")
            .insert_header(ContentType::json())
            .insert_header(("connection-string", format!("{}/rust_test",test_connection_string)))
            .set_json(payload_1)
            .to_request();
        let resp_1 = test::call_service(&app, req_1).await; 
            
        let req_2 = test::TestRequest::post()
            .uri("/test")
            .insert_header(ContentType::json())
            .insert_header(("connection-string", format!("{}/rust_test",test_connection_string)))
            .set_json(payload_2)
            .to_request();
        let resp_2 = test::call_service(&app, req_2).await; 
        
        let req_3 = test::TestRequest::post()
            .uri("/test")
            .insert_header(ContentType::json())
            .insert_header(("connection-string", format!("{}/rust_test",test_connection_string)))
            .set_json(payload_3)
            .to_request();
        let resp_3 = test::call_service(&app, req_3).await; 
       
        let mut json_body_0: serde_json::Value = test::read_body_json(resp_0).await;
        //Dropping rows_affected key, because there could be many items in the live db which will
        //fail the test
        //
        let nested = json_body_0.get_mut("results")
            .expect("should exist")
            .as_array_mut()
            .expect("should be an array")
            .get_mut(0)
            .expect("should have 0th element")
            .as_object_mut()
            .expect("should be and object");
        nested.remove("rowsAffected");

        let json_body_1: serde_json::Value = test::read_body_json(resp_1).await;
        let json_body_2: serde_json::Value = test::read_body_json(resp_2).await;
        let json_body_3: serde_json::Value = test::read_body_json(resp_3).await;
        
        println!("{}", json!(json_body_0));
        println!("pretty0:\n\n{}", serde_json::to_string_pretty(&json_body_0).unwrap());
        println!("{}", json!(json_body_1));
        println!("pretty1:\n\n{}", serde_json::to_string_pretty(&json_body_1).unwrap());
        println!("{}", json!(json_body_2));
        println!("pretty2:\n\n{}", serde_json::to_string_pretty(&json_body_2).unwrap());
        println!("{}", json!(json_body_3));
        println!("pretty3:\n\n{}", serde_json::to_string_pretty(&json_body_3).unwrap());
    
        let expected_0 = json!(
            {"results":[{"tag":"drop_db","success":"true","lastInsertId":0},{"success":"true","rowsAffected":1,"lastInsertId":0},{"success":"true","resultsSet":[{"Database":"information_schema"},{"Database":"mysql"},{"Database":"performance_schema"},{"Database":"rust_test"},{"Database":"sys"},{"Database":"test"}]},{"tag":"change_db","success":"true","rowsAffected":0,"lastInsertId":0},{"success":"true","rowsAffected":0,"lastInsertId":0}]}
        );

        let expected_1 = json!(
            {"results":[{"success":"true","rowsAffected":1,"lastInsertId":1},{"success":"true","resultsSet":[{"_id":1,"_bool":true,"_tiny_int":-100,"_int":-2147483648,"_big_int":-9223372036854775808i64,"_big_int_unsigned":18446744073709551615u64,"_float":2.4000000953674316,"_double":4.2,"_text":"test","_varchar_5":"abcde","_date":"2008-07-04","_date_time":"2023-11-29 14:03:15","_time_stamp":"2023-11-30 00:03:15"}]},{"success":"true","rowsAffected":1,"lastInsertId":0},{"tag":"fail on purpose","success":"false","error":"error returned from database: 1054 (42S22): Unknown column '_not_present' in 'where clause'"}]}
        );

        assert_eq!(json_body_0, json!(expected_0));
        assert_eq!(json_body_1, json!(expected_1));

     

    }
}
