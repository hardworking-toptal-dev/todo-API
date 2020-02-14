# To-do list app using ACTIX

A simple web API that can manage "to-do" items stored in a database, using the Rust Web Framework [Actix](https://actix.rs). 

# Usage

## Prerequisites

```
$ rustc -V
# rustc 1.41.0 (5e1a79984 2020-01-27)
```

```
$ cargo -V
# cargo 1.41.0 (626f0f40e 2019-12-03)
```

## Server

From the root directory of this project:
```
cargo run
# Started http server: 127.0.0.1:8080
```

## Web client

With REST client like [Postman](https://www.postman.com/), [Insomnia](https://insomnia.rest/) and others.

| API                        | Description             | Request body  | Response body        |
| -------------------------- | ----------------------- | ------------- | -------------------- |
| GET /api/TodoItems         | Get all to-do items     | None          | Array of to-do items |
| GET /api/TodoItems/{id}    | Get an item by ID       | None          | To-do item           |
| POST /api/TodoItems        | Add a new item          | To-do item    | To-do item           |
| PUT /api/TodoItems/{id}    | Update an existing item | To-do item    | None                 |
| DELETE /api/TodoItems/{id} | Delete an item          | None          | None                 |

# Resources used to learn how to do this project

* [Actix web rust web framework](https://actix.rs)
* [Actix web examples](https://github.com/actix/examples)
* [Rust language web site](https://www.rust-lang.org/)
* [The Rust Programming Language book](https://doc.rust-lang.org/book/)
* [The Cargo Book](https://doc.rust-lang.org/cargo/index.html)

# License

This project is licensed by MIT license (http://www.apache.org/licenses/LICENSE-2.0)

# Contribute

Welcome to contribute!