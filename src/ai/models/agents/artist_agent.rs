use crate::ai::models::core_agent::core_agent::{AgentState, CoreAgent};
use crate::ai_functions::ai_artist::{print_frag_shader_code, print_improved_frag_shader_code};
use crate::helpers::local::task_request;
use crate::helpers::local::{read_template, save_frag_file};

use crate::ai::models::agents::agent_traits::{AgentFunctions, Shader};

use async_trait::async_trait;

#[derive(Debug)]

pub struct ArtistAgent {
    attributes: CoreAgent,
    bug_errors: Option<String>,
    bug_count: u8,
}

impl ArtistAgent {
    pub fn new() -> Self {
        let attributes: CoreAgent = CoreAgent {
            objective: "Develops code for the fragment shader".to_string(),
            position: "Artist Agent".to_string(),
            state: AgentState::Discover,
            memory: vec![],
        };

        Self {
            attributes,
            bug_errors: None,
            bug_count: 0,
        }
    }

    async fn call_initial_shader_code(&mut self, shader: &mut Shader) {
        let msg_context: String = format!(
            "CODE_TEMPLATE: {:?} \n SHADER_DESCRIPTION: {:?}\n",
            shader.frag_shader, shader
        );

        let ai_response: String = task_request(
            msg_context,
            &self.attributes.position,
            get_func_str!(print_frag_shader_code),
            print_frag_shader_code,
        )
        .await;

        save_frag_file(&ai_response);
        shader.frag_shader = Some(ai_response);
    }

    async fn call_improved_shader_code(&mut self, shader: &mut Shader) {
        let msg_context: String = format!(
            "CODE_TEMPLATE: {:?} \n SHADER_DESCRIPTION: {:?}\n",
            shader.frag_shader, shader
        );

        let ai_response: String = task_request(
            msg_context,
            &self.attributes.position,
            get_func_str!(print_improved_frag_shader_code),
            print_improved_frag_shader_code,
        )
        .await;

        save_frag_file(&ai_response);
        shader.frag_shader = Some(ai_response);
    }

    async fn call_fix_shader_code(&mut self, shader: &mut Shader) {
        let msg_context: String = format!(
            "BROKEN_CODE: {:?} \n ERROR_BUGS: {:?}\n
            THIS FUNCTION ONLY OUTPUTS THE CODE. JUST THE WORKING CODE AND NOTHING ELSE",
            shader.frag_shader, self.bug_errors
        );

        let ai_response: String = task_request(
            msg_context,
            &self.attributes.position,
            get_func_str!(print_improved_frag_shader_code),
            print_improved_frag_shader_code,
        )
        .await;

        save_frag_file(&ai_response);
        shader.frag_shader = Some(ai_response);
    }
}

#[async_trait]
impl AgentFunctions for ArtistAgent {
    fn get_attributes_from_agent(&self) -> &CoreAgent {
        &self.attributes
    }

    async fn execute(&mut self, shader: &mut Shader) -> Result<(), Box<dyn std::error::Error>> {
        while self.attributes.state != AgentState::Finished {
            match &self.attributes.state {
                AgentState::Discover => {
                    self.call_initial_shader_code(shader).await;
                    self.attributes.state = AgentState::Working;
                    continue;
                }

                AgentState::Working => {
                    if self.bug_count == 0 {
                        self.call_improved_shader_code(shader).await;
                    } else {
                        self.call_fix_shader_code(shader).await;
                    }

                    self.attributes.state = AgentState::Testing;
                    continue;
                }

                AgentState::Testing => {
                    // Implement later
                    self.attributes.state = AgentState::Finished;
                }
                _ => self.attributes.state = AgentState::Finished,
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn tests_shader_artist() {
        let mut agent = ArtistAgent::new();

        let shader_str: &str = r#"
        {
            "shader_description": "build a fragment shader which changes color with time.",
            "scope":  {
                    "are_uniforms_required": true
                },
            
            "uniforms": ["u_time"],
            
            "frag_shader": null
        }
        "#;

        let mut shader: Shader = serde_json::from_str(shader_str).unwrap();

        agent.attributes.state = AgentState::Testing;

        agent
            .execute(&mut shader)
            .await
            .expect("Failed to execute Shader.");
        ()
    }
}
