#[macro_export]
macro_rules! get_func_str {
    ($func: ident) => {{
        stringify!($func)
    }};
}

#[macro_use]
extern crate diesel;
extern crate dotenv;

#[macro_use]
use ai::helpers::command_line::get_response;

use actix_cors::Cors;
use actix_service::Service;
use actix_web;
use actix_web::{middleware::Logger, App, HttpResponse, HttpServer};
use futures::future::{ok, Either};

mod ai;
mod json_serialization;
mod models;
mod views;

use ai::ai_functions;
use ai::helpers;
use ai::models::manager::manager::Manager;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    const ALLOWED_VERSION: &'static str = include_str!("./v.txt");
    // let usr_request: String = get_response("Define your perfect shader: ");

    // let mut managing_agent: Manager = Manager::new(usr_request)
    //     .await
    //     .expect("Error creating the managing agent.");

    // managing_agent.execute_all().await;

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(|| {
        let cors: Cors = Cors::default()
            .allow_any_header()
            .allow_any_method()
            .allow_any_origin();

        let app = App::new()
            .wrap_fn(|req, srv| {
                let passed: bool;

                if *&req.path().contains(&format!("/{}/", ALLOWED_VERSION)) {
                    passed = true;
                } else {
                    passed = false;
                }

                let end_result = match passed {
                    true => Either::Left(srv.call(req)),
                    false => {
                        let resp = HttpResponse::NotImplemented()
                            .body(format!("Use {} API", ALLOWED_VERSION));

                        Either::Right(ok(req.into_response(resp).map_into_boxed_body()))
                    }
                };

                async move {
                    let result = end_result.await?;
                    Ok(result)
                }
            })
            .configure(views::factory)
            .wrap(cors)
            .wrap(Logger::new("%a %{User-Agent}i %r %s %D"));
        return app;
    })
    .bind("0.0.0.0:3002")?
    .run()
    .await
}
