pub mod create;

use actix_web::web::{post, scope, ServiceConfig};

pub fn shader_factory(app: &mut ServiceConfig) {
    app.service(scope("/v1/shader").route("create", post().to(create::create)));
}
