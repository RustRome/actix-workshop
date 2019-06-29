use actix_web::{middleware, web, App, HttpServer};
use workshop::{config_app, create_state, middleware::Counter};

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let state = create_state();
    let data = web::Data::new(state);

    HttpServer::new(move || {
        App::new()
            .register_data(data.clone())
            .wrap(Counter)
            .wrap(middleware::Logger::default())
            .configure(|builder| config_app(builder))
    })
    .bind("localhost:8080")?
    .run()
}
