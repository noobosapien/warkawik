use serde::Deserialize;

#[derive(Deserialize)]
pub struct ShaderReq {
    pub content: String,
}
