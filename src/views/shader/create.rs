use crate::diesel;
use diesel::prelude::*;

use actix_web::{web, HttpResponse};

use crate::ai::models::agents::agent_traits::Shader;
use crate::ai::models::manager::manager::Manager;
use crate::json_serialization::shader_req::ShaderReq;

pub async fn create(shader_req: web::Json<ShaderReq>) -> HttpResponse {
    println!("{}", shader_req.content);

    let mut managing_agent: Manager = Manager::new(shader_req.content.clone())
        .await
        .expect("Error creating the managing agent.");

    let sh: &Shader = managing_agent.execute_all().await;
    let frag_shader = sh.frag_shader.as_ref().unwrap().clone();

    HttpResponse::Ok().json(frag_shader)
}
