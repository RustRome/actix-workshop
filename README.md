# RustLab: WebService with Rust and Actix Web Workshop

* This repo contains a set exercises to guide you into writing WebService with Rust and Actix. It's based on the recent release of Actix-Web 1.0

# Contents

  - [Suggested prerequisites](#Suggested-prerequisites)
  - [Tools and Documentation](#Tools-and-Documentation)
    - [Tools](#Tools)
    - [Documentation](#Documentation)
  - [Repo Structure](#Repo-Structure)
  - [Worflow](#Worflow)
  - [Exercises](#Exercises)
    - [WarmUp](#WarmUp)
      - [Hello World](#Hello-World)
        - [Description](#Description)
        - [Test Spec](#Test-Spec)
        - [Resources](#Resources)
      - [Serving static files](#Serving-static-files)
      - [Handling parameters](#Handling-parameters)
      - [Serving JSON](#Serving-JSON)
      - [JSON Payload](#JSON-Payload)
      - [Async handlers](#Async-handlers)
      - [State and Middleware](#State-and-Middleware)
    - [Contact Book](#Contact-Book)
      - [Create contact](#Create-contact)
      - [Get contact](#Get-contact)
      - [List contacts](#List-contacts)
      - [Delete contact](#Delete-contact)


# Suggested prerequisites

This workshop is an introduction on Actix Web for writing web service. Knowledge of libraries needed for completing the workshop like `actix-web`, `serde`, `futures` and `diesel` is not required as they will be briefly introduced during the workshop in order to complete single tasks. You may approch the workshop with any level of experience with Rust, but basic understanding of functions, structs, traits and generics is recommended.


# Tools and Documentation


## Tools
- [Rust](https://www.rust-lang.org/tools/install) Minimum supported Rust version: 1.34 or later
- [Diesel Cli](http://diesel.rs/guides/getting-started) for database migrations (Optional) 


To install the cli without these dependencies, omit the unneeded dependencies from the following command:

```
cargo install diesel_cli --no-default-features --features "sqlite"
```

If you are using a system without an easy way to install sqlite (for example Windows), you can use a bundled version instead:

```
cargo install diesel_cli --no-default-features --features "sqlite-bundled"
```

## Documentation 

- [Actix-Web](https://docs.rs/actix-web/1.0.2/actix_web/)
- [Actix-Web Examples](https://github.com/actix/examples)
- [Diesel](http://diesel.rs/guides/getting-started)
- [Futures](https://docs.rs/futures/0.1.27/futures/)
- [Serde JSON](https://docs.serde.rs/serde_json/)

# Repo Structure

```
.
├── Cargo.lock
├── Cargo.toml
├── migrations
├── README.md
├── src
│   ├── bin
│   │   └── main.rs
│   ├── lib.rs
│   ├── middleware.rs
│   ├── test.rs
│   └── util.rs
└── static
    └── index.html
```

* **main.rs**: The main function with the actix-web application ([App](https://docs.rs/actix-web/1.0.2/actix_web/struct.App.html)) boostrap.
* **lib.rs**: The entry point for the workshop. It contains  the function `config_app` which it will be used for the configuration of the application like routes and global state by using the [ServiceConfig](https://docs.rs/actix-web/1.0.2/actix_web/web/struct.ServiceConfig.html) API.
* **middleware.rs**: A mock actix-web middlewere which passthrough the requests.
* **utils.rs**: Collections of functions that can be used during the workshop.
* **test.rs**:  Tests to implement.
* **index.html*: simple html page.


# Workflow

The project contains a test suite written in `src/test.rs` that you need to make it pass.

Run `cargo test` in the root directory of the project in order to run the test suite.

Initatially all the tests are marked as `#[ignore]`. Open `src/test.rs` and comment or remove the ignore attribute in order to let the test run and work on the implementation of the test one by one.


The `main.rs` file contains the bootstrap of the application tha will be configured with the external function `config_app` in `lib.rs` file, which is the entry point for this workshop.

Apart from the entry point there is not fixed structure for the project. Feel free to apply the structure/modularization which you feel is better for your implementation. 


# Exercises

The exercises are divided in two groups. 

- [WarmUp](#WarmUp): Small excercises for getting familiar with Actix-Web APIs.
- [Contact Book](#Contact-Book) Build REST APIs for a simple contact book application.

## WarmUp

This group is composed by 6 tests which cover from the basic Actix Web, like responding with plain text, to more complex one like implementing a Middleware with a state.

### Hello World


The first exercise requires to an handler with path `/` which returns a plain text `Hello RustLab`  when invoked.
An handler in actix-web is a function where parameters implements [FromRequest](https://docs.rs/actix-web/1.0.2/actix_web/trait.FromRequest.html) trait for extracting information from the `Request` and the return value implements [Responder](https://docs.rs/actix-web/1.0.2/actix_web/trait.Responder.html) trait for the conversion to an http response.

Example of an handler

```rust
use actix_web::{web, App};

fn index(req: HttpRequest) -> String {
   format!("Got thing: {:?}", req)
}

fn main() {
    let app = App::new().service(
        web::resource("/").to(index));
}

```

Open the file `lib.rs` and register the handler on the struct [ServiceConfig](https://docs.rs/actix-web/1.0.2/actix_web/web/struct.ServiceConfig.html).


**Test Spec**

Open the file `test.rs` and remove/comment the `#[ignore]` attributes on the function `index`.

Run `cargo test warmup::index` for executing this single test.


| Path   |  Method     |  Response(Text)        | Status |
| :-----:| :------:    | :--------------: | :-------:|
|  `/`   |  **GET**    |  Hello RustLab   |  200   |


**Resources**

List of resources/documentation for completing the task:

- [ServiceConfig](https://docs.rs/actix-web/1.0.2/actix_web/web/struct.ServiceConfig.html)
- [web::resource](https://docs.rs/actix-web/1.0.2/actix_web/web/fn.resource.html)
- [web](https://docs.rs/actix-web/1.0.2/actix_web/web/index.html)


### Serving static files


This exercise requires to register an handler with path `/static` which returns the content of the file  `static/index.html`

Open the file `lib.rs` and register the handler on the struct [ServiceConfig](https://docs.rs/actix-web/1.0.2/actix_web/web/struct.ServiceConfig.html).


**Test Spec**

Open the file `test.rs` and remove/comment the `#[ignore]` attributes on the function `static_content`.

Run `cargo test warmup::static_content` for executing this single test.


| Path       |  Method     |  Response(Text)        | Status |
| :-----:    | :------:    | :--------------: | :-------:|
|  `/static` |  **GET**    |  Content of the file `static/index.html`   |  200   |

**Resources**

List of resources/documentation for completing the task:

- [Files](https://docs.rs/actix-files/0.1.2/actix_files/struct.Files.html)



### Handling parameters

Register an handler with path `/hello/{name}`, where `{name}` is a dynamic path parameter, and return a plain text of the concatenation of `Hello` and the `{name}` parameter received in input.


Open the file `lib.rs` and register the handler on the struct [ServiceConfig](https://docs.rs/actix-web/1.0.2/actix_web/web/struct.ServiceConfig.html).


**Test Spec**

Open the file `test.rs` and remove/comment the `#[ignore]` attributes on the function `hello_params`.

Run `cargo test warmup::hello_params` for executing this single test.


| Path       |  Method     |  Response(Text)        | Status |
| :-----:    | :------:    | :--------------: | :-------:|
|  `/hello/{name}` |  **GET**    |   Hello `{name}`  |  200   |

**Resources**

List of resources/documentation for completing the task:

- [Path](https://docs.rs/actix-web/1.0.2/actix_web/web/struct.Path.html)



### Serving JSON

Register an handler with path `/hello_json/{name}`, where `{name}` is a dynamic path parameter, but this time do not return the plain text. Returns a JSON with only 1 field `message` which contains the concatenation of `Hello` and the `{name}` parameter received in input.

```JSON
{
  "message": "Hello ${name}"
}
```


Open the file `lib.rs` and register the handler on the struct [ServiceConfig](https://docs.rs/actix-web/1.0.2/actix_web/web/struct.ServiceConfig.html).


**Test Spec**

Open the file `test.rs` and remove/comment the `#[ignore]` attributes on the function `hello_params_json`.

Run `cargo test warmup::hello_params_json` for executing this single test.


| Path       |  Method     |  Response(JSON)        | Status |
| :-----:    | :------:    | :--------------: | :-------:|
|  `/hello_json/{name}` |  **GET**    |   `{ "message" : "Hello ${name}" }`  |  200   |

**Resources**

List of resources/documentation for completing the task:

- [Path](https://docs.rs/actix-web/1.0.2/actix_web/web/struct.Path.html)
- [Json](https://docs.rs/actix-web/1.0.2/actix_web/web/struct.Json.html)
- [HttpResponse](https://docs.rs/actix-web/1.0.2/actix_web/web/struct.HttpResponse.html)
  
### JSON Payload

Register an handler with path `/json_body`, which receive in input a JSON like this:
```JSON
{
  "name" : {$name:String}
}
```
 Returns a JSON with only 1 field `message` which contains the concatenation of `Hello` and the `name` field in the JSON received in input.

```JSON
{
  "message": "Hello ${name}"
}
```


Open the file `lib.rs` and register the handler on the struct [ServiceConfig](https://docs.rs/actix-web/1.0.2/actix_web/web/struct.ServiceConfig.html).


**Test Spec**

Open the file `test.rs` and remove/comment the `#[ignore]` attributes on the function `json_body`.

Run `cargo test warmup::json_body` for executing this single test.


| Path       |  Method     | Payload |  Response(JSON)  | Status |
| :-----:    | :------:    | :----: |:--------------: | :-------:|
|  `/json_body` |  **POST**  | `{ "name" : ${name} }` |   `{ "message" : "Hello ${name}" }`  |  200  |

**Resources**

List of resources/documentation for completing the task:

- [Path](https://docs.rs/actix-web/1.0.2/actix_web/web/struct.Path.html)
- [Json](https://docs.rs/actix-web/1.0.2/actix_web/web/struct.Json.html)
- [HttpResponse](https://docs.rs/actix-web/1.0.2/actix_web/web/struct.HttpResponse.html)
  

### Async handlers


Register an handler with path `/async_error`  and returns a JSON which represent a the output of a possible error occurred. The JSON body should look like this:

```JSON
{
  "msg": "some error",
  "status": 400
} 
```

and the HTTP status code should be `400` Bad Request.

Although it is not mandatory for making the test pass, it is recommended in order to get familiar with Actix-Web APIs to implement the handler in async way. Up to this point we implemented only sync handler registered on `Resources` or `Routes` with the API `.to(handler)`. Use on the same structs the API `to_async(handler)` to register async handler functions, which should returns an `impl Future<>`.

Example of async handler:


```rust
use actix_web::{web, App, Error};
use futures::{future,Future};

fn index() -> impl Future<Item= &'static str, Error= Error> {
  future::ok("Hello Async")
}
fn main() {
  let app = App::new().service(
      web::resource("/async") 
          .route(web::get().to_async(index)) 
  );
}    
```


Open the file `lib.rs` and register the async handler on the struct [ServiceConfig](https://docs.rs/actix-web/1.0.2/actix_web/web/struct.ServiceConfig.html).


**Test Spec**

Open the file `test.rs` and remove/comment the `#[ignore]` attributes on the function `async_json_error`.

Run `cargo test warmup::async_json_error` for executing this single test.


| Path       |  Method     |  Response(JSON)        | Status |
| :-----:    | :------:    | :--------------: | :-------:|
|  `/async_error` |  **GET**    |   `{"msg":"some error","status":400}`  |  400   |

**Resources**

List of resources/documentation for completing the task:


- [Json](https://docs.rs/actix-web/1.0.2/actix_web/web/struct.Json.html)
- [HttpResponse](https://docs.rs/actix-web/1.0.2/actix_web/web/struct.HttpResponse.html)
- [Futures](https://docs.rs/futures/0.1.27/futures/)


### State and Middleware


Register an handler with path `/requests`  and returns a JSON which represent the total number of requests received by the application till that moment. The expected JSON is

```JSON
{
  "count": count
} 
```

To keep track of number of request received by the application you should use a counter in the global state attacched to the application with the API `.data()` available in [ServiceConfig](https://docs.rs/actix-web/1.0.2/actix_web/web/struct.ServiceConfig.html). Check the official documentation for an example of global state [here](https://docs.rs/actix-web/1.0.2/actix_web/web/struct.Data.html). 


The counter should be incremented in the method `CounterMiddleware::call` in the provided middleware and returned in the handler registered with path `/requests` in the JSON payload.

Use the API `.app_data` in [ServiceRequest](https://docs.rs/actix-web/1.0.2/actix_web/dev/struct.ServiceRequest.html) for accessing the global state within the middleware.




Open the file `lib.rs` and register handler on the struct [ServiceConfig](https://docs.rs/actix-web/1.0.2/actix_web/web/struct.ServiceConfig.html). Modify the file `middleware.rs` in order to put the logic of the counter for each request in the method
`CounterMiddleware::call`.


**Test Spec**

Open the file `test.rs` and remove/comment the `#[ignore]` attributes on the function `middleware`.

Run `cargo test warmup::middleware` for executing this single test.


| Path       |  Method     |  Response(JSON)        | Status |
| :-----:    | :------:    | :--------------: | :-------:|
|  `/requests` |  **GET**    |   `{ "count": count }`  |  200   |

**Resources**

List of resources/documentation for completing the task:


- [Json](https://docs.rs/actix-web/1.0.2/actix_web/web/struct.Json.html)
- [HttpResponse](https://docs.rs/actix-web/1.0.2/actix_web/web/struct.HttpResponse.html)
- [Futures](https://docs.rs/futures/0.1.27/futures/)
- [Data](https://docs.rs/actix-web/1.0.2/actix_web/web/struct.Data.html)
- [ServiceRequest](https://docs.rs/actix-web/1.0.2/actix_web/dev/struct.ServiceRequest.html)


## Contact Book

We will create a simple contact book that will persist data using diesel to a sqlite database.
for this exercise you will need to setup `diesel` with the support of sqlite on your machine.


Diesel cli install

```
cargo install diesel_cli --no-default-features --features "sqlite"
```

Setup the environment variable for configure the database location for diesel cli, application and tests
```
echo -e "DATABASE_URL=/tmp/workshop.db \nTEST_DATABASE_URL=./target/workshop.db" > .env
```

Than run commands for setup diesel on the project

```
diesel setup
```

And prepare the envoriment for the first database structure configurations.

```
diesel migration generate create_contacts
```

This commad will create two files `migrations/XXX_DATE_XXX_create_contacts/up.sql` and `migrations/XXX_DATE_XXX_create_contacts/down.sql`, in the `up.sql` you will need to write all the scritps for creating the tables ecc and in the `down.sql` the scripts to undo all is done in by the `up.sql`.

The setup of diesel is finished here, though there are two additional diesel commands that you need in the development process that are: `diesel migration run` for create the structure in the database from the scripts and `diesel migration redo` that can be used to regenerate the database structure in case of scripts update.


### Create contact

Register an handler with path `/api/contacts`, that receive as post a JSON with the field `name` and `email` and returns a JSON with three fields:`id`,`name`,`email` the sent JSON should be persisted and a sequential id generated using the persistence.  

Example Post JSON  

```JSON
{
  "name": "Mark",
  "email": "mark@foo.com"
}
```

Example Return JSON  

```JSON
{
  "id": 1,
  "name": "Mark",
  "email": "mark@foo.com"
}
```

Create the script for create the contact table and run the diesel migrations with `diesel migration run`.  
Open the file `lib.rs` and add the schema module generated by diesel in the `schema.rs` with `mod schema`.
Open the file `lib.rs` and register the handler on the struct [ServiceConfig](https://docs.rs/actix-web/1.0.2/actix_web/web/struct.ServiceConfig.html).

**Test Spec**

Open the file `workshop_test.rs` and remove/comment the `#[ignore]` attributes on the function `create_contact`.

Run `cargo test contacts_tests::create_contact` for executing this single test.



| Path       |  Method     | Request(JSON) |  Response(JSON)        | Status |
| :-----:    | :------:    | :---------: | :--------------: | :-------:|
|  `/api/contacts` |  **POST**  | `{ "name": "Mark", "email": "mark@foo.com"}` |  `{ "id": 1,"name": "Mark","email": "mark@foo.com"}`  |  200   |

**Resources**

List of resources/documentation for completing the task:

- [Path](https://docs.rs/actix-web/1.0.2/actix_web/web/struct.Path.html)
- [Json](https://docs.rs/actix-web/1.0.2/actix_web/web/struct.Json.html)
- [HttpResponse](https://docs.rs/actix-web/1.0.2/actix_web/web/struct.HttpResponse.html)
- [Diesel Getting Started](http://diesel.rs/guides/getting-started/)
- [Async Blocking calls](https://docs.rs/actix-web/1.0.2/actix_web/web/fn.block.html)
- [Insert API](http://docs.diesel.rs/diesel/fn.insert_into.html)
- [First Result](http://docs.diesel.rs/diesel/query_dsl/trait.RunQueryDsl.html#method.first)
- [Select](http://docs.diesel.rs/diesel/fn.select.html)

### Get contact

Register an handler with path `/api/contacts/{id}`, where `id` is the unique identifier of a contact and returns a JSON with three fields:`id`,`name`,`email` that is the result of the loaded contact from the db with the specific id

Example Return JSON  

```JSON
{
  "id": 1,
  "name": "Mark",
  "email": "mark@foo.com"
}
```

Open the file `lib.rs` and register the handler on the struct [ServiceConfig](https://docs.rs/actix-web/1.0.2/actix_web/web/struct.ServiceConfig.html).

**Test Spec**

Open the file `workshop_test.rs` and remove/comment the `#[ignore]` attributes on the function `get_contact`.

Run `cargo test contacts_tests::get_contact` for executing this single test.



| Path       |  Method     |  Response(JSON)        | Status |
| :-----:    | :------:    | :--------------: | :-------:|
|  `/api/contacts/{id}` |  **GET**  | `{ "id": 1,"name": "Mark","email": "mark@foo.com"}`  |  200   |
|  `/api/contacts/{id}` |  **GET**  | ``  |  404   |

**Resources**

List of resources/documentation for completing the task:

- [Path](https://docs.rs/actix-web/1.0.2/actix_web/web/struct.Path.html)
- [Json](https://docs.rs/actix-web/1.0.2/actix_web/web/struct.Json.html)
- [HttpResponse](https://docs.rs/actix-web/1.0.2/actix_web/web/struct.HttpResponse.html)
- [Diesel Getting Started](http://diesel.rs/guides/getting-started/)
- [Async Blocking calls](https://docs.rs/actix-web/1.0.2/actix_web/web/fn.block.html)
- [First Result](http://docs.diesel.rs/diesel/query_dsl/trait.RunQueryDsl.html#method.first)
- [Select](http://docs.diesel.rs/diesel/fn.select.html)


### List contacts

Register an handler with path `/api/contacts`, that returns a JSON with an array of objects three fields:`id`,`name`,`email` that is the result of load of all the persistent Contacts

Example Return JSON  

```JSON
[{
  "id": 1,
  "name": "Mark",
  "email": "mark@foo.com"
}]
```

Open the file `lib.rs` and register the handler on the struct [ServiceConfig](https://docs.rs/actix-web/1.0.2/actix_web/web/struct.ServiceConfig.html).

**Test Spec**

Open the file `workshop_test.rs` and remove/comment the `#[ignore]` attributes on the function `list_contact`.

Run `cargo test contacts_tests::list_contact` for executing this single test.



| Path       |  Method     |  Response(JSON)        | Status |
| :-----:    | :------:    | :--------------: | :-------:|
|  `/api/contacts` |  **GET**  | `[{ "id": 1,"name": "Mark","email": "mark@foo.com"}]`  |  200   |

**Resources**

List of resources/documentation for completing the task:

- [Path](https://docs.rs/actix-web/1.0.2/actix_web/web/struct.Path.html)
- [Json](https://docs.rs/actix-web/1.0.2/actix_web/web/struct.Json.html)
- [HttpResponse](https://docs.rs/actix-web/1.0.2/actix_web/web/struct.HttpResponse.html)
- [Diesel Getting Started](http://diesel.rs/guides/getting-started/)
- [Async Blocking calls](https://docs.rs/actix-web/1.0.2/actix_web/web/fn.block.html)
- [Load Results](http://docs.diesel.rs/diesel/query_dsl/trait.RunQueryDsl.html#method.load)
- [Select](http://docs.diesel.rs/diesel/fn.select.html)

### Delete contact
Register an handler with path `/api/contacts/{id}`, that on delete remove the specific contact that is identified by the id.

Open the file `lib.rs` and register the handler on the struct [ServiceConfig](https://docs.rs/actix-web/1.0.2/actix_web/web/struct.ServiceConfig.html).

**Test Spec**

Open the file `workshop_test.rs` and remove/comment the `#[ignore]` attributes on the function `delete_contact`.

Run `cargo test contacts_tests::delete_contact` for executing this single test.


| Path       |  Method     |  Status |
| :-----:    | :------:    |  :-------:|
|  `/api/contacts/{id}` |  **DELETE**  |   204   |
|  `/api/contacts/{id}` |  **DELETE**  |   404   |

**Resources**

List of resources/documentation for completing the task:

- [Path](https://docs.rs/actix-web/1.0.2/actix_web/web/struct.Path.html)
- [Json](https://docs.rs/actix-web/1.0.2/actix_web/web/struct.Json.html)
- [HttpResponse](https://docs.rs/actix-web/1.0.2/actix_web/web/struct.HttpResponse.html)
- [Diesel Getting Started](http://diesel.rs/guides/getting-started/)
- [Async Blocking calls](https://docs.rs/actix-web/1.0.2/actix_web/web/fn.block.html)
- [Delete Records](http://docs.diesel.rs/diesel/fn.delete.html)



