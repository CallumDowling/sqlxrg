# 🌿 Introduction

**sqlxrg** is a web api that translates json encoded sql transactions, currently supporting mysql/mariadb

Based https://github.com/proofrock/sqliterg by Germano Rizzo. This project uses most the same same request / response format, but connects with a mysql/mariadb instance instead of creating an sqlite db. Accordingly most the the config settings are gone. 

As a quick example, after launching:

```bash
sqlxrg --bind_host localhost --port 8083
```

It's possible to make a POST call to `http://localhost:8083`.

With headers:
```
    content-type: application/json
    connection-string: mariadb://<user>:<pass>@<host>:<port>/<database>
        where database is optional and host/port is releative to where this software is set deployed
```

with the following body:

```json
{
  "transaction": [
    {
      "statement": "CREATE OR REPLACE TABLE example(id INT AUTO_INCREMENT, name TEXT, PRIMARY KEY(id));",
      "tag": "create"
    },
    {
      "statement": "INSERT INTO example(name) VALUES('test')",
      "tag": "insert"
    },
    {
      "query": "select * from example;",
      "tag": "select"
    }
  ]
}
```

Obtaining an answer of:

```json
{
  "results": [
    {
      "tag": "create",
      "success": "true",
      "rowsAffected": 0,
      "lastInsertId": 0
    },
    {
      "tag": "insert",
      "success": "true",
      "rowsAffected": 1,
      "lastInsertId": 1
    },
    {
      "tag": "select",
      "success": "true",
      "resultsSet": [
        {
          "id": 1,
          "name": "test"
        }
      ]
    }
  ]
}
```
N.B. tag is optional 

It is also possible and recommended to use variable bindings like so:
```json
{
  "transaction": [
    {
        "statement": "CREATE TABLE null_test (_id_1 INT NOT NULL, _test_1 TEXT NULL, _test_2 INT NULL);"
    },
    {
        "statement": "INSERT INTO null_test(_id_1, _test_1, _test_2) VALUES(0, ?, ?);",
        "values": ["test", 5]
    }
  ]
}
```

###
Some sql types returned may fail, see tests for what is covered

### Security
* Reverse proxy should be used, all comms should be https as db connection is included in headers
* Mysql user should have restricted permissions

# 🥇 Credits

Thanks Germano Rizzo for creating sqliterg
