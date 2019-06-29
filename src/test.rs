#[cfg(test)]
mod common {
    use crate::{config_app, create_state, middleware::Counter};
    use actix_http::HttpService;
    use actix_http_test::{TestServer, TestServerRuntime};
    use actix_web::{web, App};

    pub fn create_server() -> TestServerRuntime {
        let state = create_state();

        let data = web::Data::new(state);

        TestServer::new(move || {
            HttpService::new(
                App::new()
                    .register_data(data.clone())
                    .configure(|builder| config_app(builder)),
            )
        })
    }

    pub fn create_server_with_middleware() -> TestServerRuntime {
        let state = create_state();

        let data = web::Data::new(state);

        TestServer::new(move || {
            HttpService::new(
                App::new()
                    .register_data(data.clone())
                    .configure(|builder| config_app(builder))
                    .wrap(Counter),
            )
        })
    }
}

#[cfg(test)]
mod warmup {

    use actix_web::http::StatusCode;
    use futures::future::Future;
    use rand::Rng;
    use serde_json::{json, Value};

    use super::common::{create_server, create_server_with_middleware};

    #[test]
    // #[ignore]
    fn index() {
        let mut srv = create_server();

        let req = srv.get("/");
        let mut response = srv.block_on(req.send()).unwrap();
        assert!(
            response.status().is_success(),
            format!(
                "{} - {:?}",
                response.status(),
                srv.block_on(response.body()).unwrap()
            )
        );

        assert_eq!(srv.block_on(response.body()).unwrap(), "Hello RustLab");
    }

    #[test]
    // #[ignore]
    fn static_content() {
        let mut srv = create_server();

        let req = srv.get("/static");
        let mut response = srv.block_on(req.send()).unwrap();
        assert!(
            response.status().is_success(),
            format!(
                "{} - {:?}",
                response.status(),
                srv.block_on(response.body()).unwrap()
            )
        );

        assert_eq!(
            srv.block_on(response.body()).unwrap(),
            include_str!("../static/index.html")
        );
    }

    #[test]
    // #[ignore]
    fn hello_params() {
        let mut srv = create_server();

        let req = srv.get("/hello/RustLab");
        let mut response = srv.block_on(req.send()).unwrap();
        assert!(
            response.status().is_success(),
            format!(
                "{} - {:?}",
                response.status(),
                srv.block_on(response.body()).unwrap()
            )
        );
        assert_eq!(response.body().wait().unwrap(), "Hello RustLab");
    }

    #[test]
    // #[ignore]
    fn hello_params_json() {
        let mut srv = create_server();

        let req = srv.get("/hello_json/Mark");
        let mut response = srv.block_on(req.send()).unwrap();
        assert!(
            response.status().is_success(),
            format!(
                "{} - {:?}",
                response.status(),
                srv.block_on(response.body()).unwrap()
            )
        );
        let expected = json!({
            "message":"Hello Mark"
        });
        assert_eq!(response.json::<Value>().wait().unwrap(), expected);
    }

    #[test]
    // #[ignore]
    fn json_body() {
        let mut srv = create_server();

        let req = srv.post("/json_body");
        let mut response = srv
            .block_on(req.send_json(&json!({"name" : "Mark"})))
            .unwrap();
        assert!(
            response.status().is_success(),
            format!(
                "{} - {:?}",
                response.status(),
                srv.block_on(response.body()).unwrap()
            )
        );
        let expected = json!({
            "message":"Hello Mark"
        });
        assert_eq!(response.json::<Value>().wait().unwrap(), expected);
    }

    #[test]
    // #[ignore]
    fn async_json_error() {
        let mut srv = create_server();

        let req = srv.get("/async_error");
        let mut response = srv.block_on(req.send()).unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let expected = json!({
            "msg":"some error",
            "status" : 400
        });
        assert_eq!(response.json::<Value>().wait().unwrap(), expected);
    }

    #[test]
    // #[ignore]
    fn middleware() {
        let mut srv = create_server_with_middleware();

        let mut rng = rand::thread_rng();

        let count = rng.gen_range(1, 10);
        for _ in 0..count {
            let req = srv.get("/");
            let mut response = srv.block_on(req.send()).unwrap();
            assert!(
                response.status().is_success(),
                format!(
                    "{} - {:?}",
                    response.status(),
                    srv.block_on(response.body()).unwrap()
                )
            );
        }

        let req = srv.get("/requests");
        let mut response = srv.block_on(req.send()).unwrap();
        assert!(
            response.status().is_success(),
            format!(
                "{} - {:?}",
                response.status(),
                srv.block_on(response.body()).unwrap()
            )
        );

        let expected = json!({ "count": count });
        assert_eq!(response.json::<Value>().wait().unwrap(), expected);
    }

}

#[cfg(test)]
mod contacts_book {

    use super::common::create_server;
    use actix_http::{http::Method, http::StatusCode};
    use actix_http_test::TestServerRuntime;
    use futures;
    use futures::future::Future;
    use serde_json::{json, Value};

    #[test]
    // #[ignore]
    fn create_contact() {
        let mut srv = create_server();
        let contact = create_contact_internal(&mut srv, new_contact("Mark", "mark@foo.com"));

        let id = contact["id"].as_i64().unwrap();

        assert_eq!(contact, new_contact_with_id(id, "Mark", "mark@foo.com"));
    }

    #[test]
    // #[ignore]
    fn get_contact() {
        let mut srv = create_server();

        let contact = create_contact_internal(&mut srv, new_contact("Mark", "mark@foo.com"));

        let id = contact["id"].as_i64().unwrap();

        let contact = get_contact_internal(&mut srv, id);

        assert_eq!(contact, new_contact_with_id(id, "Mark", "mark@foo.com"));

        let req = srv.get(format!("/api/contacts/{}", 120));

        let response = srv.block_on(req.send()).unwrap();
        assert_eq!(response.status(), StatusCode::NOT_FOUND)
    }

    #[test]
    // #[ignore]
    fn list_contact() {
        let mut srv = create_server();

        let contact = create_contact_internal(&mut srv, new_contact("Mark", "mark@foo.com"));

        let id = contact["id"].as_i64().unwrap();

        let req = srv.get("/api/contacts");
        let mut response = srv.block_on(req.send()).unwrap();
        assert!(
            response.status().is_success(),
            format!(
                "{} - {:?}",
                response.status(),
                srv.block_on(response.body()).unwrap()
            )
        );
        let expected = json!([new_contact_with_id(id, "Mark", "mark@foo.com")]);

        assert_eq!(response.json::<Value>().wait().unwrap(), expected);
    }

    #[test]
    // #[ignore]
    fn delete_contact() {
        let mut srv = create_server();

        let contact = create_contact_internal(&mut srv, new_contact("Mark", "mark@foo.com"));

        let id = contact["id"].as_i64().unwrap();

        let req = srv.request(Method::DELETE, srv.url(&format!("/api/contacts/{}", id)));
        let mut response = srv.block_on(req.send()).unwrap();
        assert!(
            response.status().is_success(),
            format!(
                "{} - {:?}",
                response.status(),
                srv.block_on(response.body()).unwrap()
            )
        );

        assert_eq!(response.status(), StatusCode::NO_CONTENT);

        let req = srv.get(format!("/api/contacts/{}", id));

        let response = srv.block_on(req.send()).unwrap();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    fn create_contact_internal(srv: &mut TestServerRuntime, contact: Value) -> Value {
        let req = srv.post("/api/contacts");
        let mut response = srv.block_on(req.send_json(&contact)).unwrap();

        assert!(
            response.status().is_success(),
            format!(
                "{} - {:?}",
                response.status(),
                srv.block_on(response.body()).unwrap()
            )
        );

        assert_eq!(response.status(), StatusCode::CREATED);

        srv.block_on(response.json::<Value>()).unwrap()
    }

    fn get_contact_internal(srv: &mut TestServerRuntime, contact_id: i64) -> Value {
        let req = srv.get(format!("/api/contacts/{}", contact_id));

        let mut response = srv.block_on(req.send()).unwrap();

        assert!(
            response.status().is_success(),
            format!(
                "{} - {:?}",
                response.status(),
                srv.block_on(response.body()).unwrap()
            )
        );

        assert_eq!(response.status(), StatusCode::OK);

        srv.block_on(response.json::<Value>()).unwrap()
    }
    fn new_contact(name: &str, email: &str) -> Value {
        json!({
            "name":name,
            "email": email
        })
    }

    fn new_contact_with_id(id: i64, name: &str, email: &str) -> Value {
        json!({
            "id" : id,
            "name":name,
            "email": email
        })
    }
}
