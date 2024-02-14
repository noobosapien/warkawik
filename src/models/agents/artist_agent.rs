use crate::ai_functions::ai_artist::{
    print_fixed_code, print_frag_shader_code, print_improved_frag_shader_code,
};
use crate::helpers::command_line::{decision_to_proceed, PrintCommand};
use crate::helpers::local::{check_status, read_template, save_frag_file};
use crate::helpers::local::{sanitize_frag, task_request};
use crate::models::core_agent::core_agent::{AgentState, CoreAgent};

use crate::models::agents::agent_traits::{AgentFunctions, RouteObject, Shader};
use crate::models::manager::manager::Manager;

use async_trait::async_trait;

#[derive(Debug)]

pub struct ArtistAgent {
    attributes: CoreAgent,
    bug_errors: Option<String>,
    bug_count: u8,
    manager: *const Manager,
}

impl ArtistAgent {
    pub fn new(manager: *const Manager) -> Self {
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
            manager,
        }
    }

    async fn call_initial_shader_code(&mut self, shader: &mut Shader) {
        let code_template_str: String = read_template();

        let msg_context: String = format!(
            "CODE_TEMPLATE: {:?} \n SHADER_DESCRIPTION: {:?}\n",
            code_template_str, shader
        );

        let manager: &Manager = unsafe { self.manager.as_ref().unwrap() };
        manager.send_msg(0, "Creating the shader.".to_string());

        let ai_response: String = task_request(
            msg_context,
            &self.attributes.position,
            get_func_str!(print_frag_shader_code),
            print_frag_shader_code,
        )
        .await;
        let sanitized = sanitize_frag(ai_response);

        // save_frag_file(&ai_response);
        shader.frag_shader = Some(sanitized);
    }

    async fn call_improved_shader_code(&mut self, shader: &mut Shader) {
        let code_template_str: String = read_template();

        let msg_context: String = format!(
            "CODE_TEMPLATE: {:?} \n SHADER_DESCRIPTION: {:?}\n",
            shader.frag_shader, shader
        );

        let manager: &Manager = unsafe { self.manager.as_ref().unwrap() };
        manager.send_msg(0, "Improving the shader.".to_string());

        let ai_response: String = task_request(
            msg_context,
            &self.attributes.position,
            get_func_str!(print_improved_frag_shader_code),
            print_improved_frag_shader_code,
        )
        .await;

        let sanitized = sanitize_frag(ai_response);

        let manager: &Manager = unsafe { self.manager.as_ref().unwrap() };
        manager.send_msg(1, sanitized.clone());

        save_frag_file(&sanitized);
        shader.frag_shader = Some(sanitized);
    }

    async fn call_fix_shader_code(&mut self, shader: &mut Shader) {
        let code_template_str: String = read_template();

        let msg_context: String = format!(
            "BROKEN_CODE: {:?} \n ERROR_BUGS: {:?}\n
            THIS FUNCTION ONLY OUTPUTS THE CODE. JUST THE WORKING CODE AND NOTHING ELSE",
            shader.frag_shader, self.bug_errors
        );

        let manager: &Manager = unsafe { self.manager.as_ref().unwrap() };
        manager.send_msg(0, "Fixing shader bugs.".to_string());

        let ai_response: String = task_request(
            msg_context,
            &self.attributes.position,
            get_func_str!(print_improved_frag_shader_code),
            print_improved_frag_shader_code,
        )
        .await;

        let sanitized = sanitize_frag(ai_response);

        save_frag_file(&sanitized);
        shader.frag_shader = Some(sanitized);
    }
}

unsafe impl Send for ArtistAgent {}

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

                AgentState::Finished => {
                    println!("Done");
                    break;
                }
                _ => {
                    self.attributes.state = AgentState::Finished;
                    break;
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::helpers::send_func::SendFn;
    use crate::models::manager::manager::Manager;
    use std::rc::Rc;
    use std::sync::Arc;

    use super::*;

    #[tokio::test]
    async fn tests_shader_artist() {
        let send_msg: Arc<Box<dyn Fn(u8, Rc<String>) + Send + Sync>> =
            Arc::new(Box::new(move |num: u8, agent_msg: Rc<String>| {
                let agent_str = (*agent_msg).to_string();

                println!("{}", agent_str);
            }));

        let send_struct = Arc::new(SendFn::new(send_msg));

        let manager = Manager::new("".to_string(), send_struct)
            .await
            .expect("Unable to create a manager");

        let mut agent = ArtistAgent::new(&manager);

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
            .expect("Failed to execute Shader.")
    }
}
