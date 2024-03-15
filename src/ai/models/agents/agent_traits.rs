use crate::ai::models::core_agent::core_agent::CoreAgent;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::marker::Send;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RouteObject {
    pub is_route_dynamic: String,
    pub method: String,
    pub request_body: String,
    pub response: serde_json::Value,
    pub route: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Scope {
    pub are_uniforms_required: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Shader {
    pub shader_description: String,
    pub scope: Option<Scope>,
    pub uniforms: Option<Vec<String>>,
    pub frag_shader: Option<String>,
}

#[async_trait]
pub trait AgentFunctions
where
    Self: Send,
    Self: Debug,
{
    fn get_attributes_from_agent(&self) -> &CoreAgent;

    async fn execute(&mut self, shader: &mut Shader) -> Result<(), Box<dyn std::error::Error>>;
}
