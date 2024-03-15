use actix_web::web::ServiceConfig;

pub mod shader;

use shader::shader_factory;

pub fn factory(app: &mut ServiceConfig) {
    shader_factory(app);
}
