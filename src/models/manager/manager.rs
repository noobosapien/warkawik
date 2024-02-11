use crate::models::agents::agent_traits::{AgentFunctions, Shader};
use crate::models::core_agent::core_agent::{AgentState, CoreAgent};

use crate::ai_functions::ai_manager::input_to_goal;
use crate::helpers::local::task_request;
use crate::helpers::send_func::SendFn;
use crate::models::agents::artist_agent::{self, ArtistAgent};
use crate::models::general::llm::Message;

use std::marker::Send;
use std::rc::Rc;
use std::sync::Arc;

#[derive(Debug)]
pub struct Manager {
    attributes: CoreAgent,
    shader: Shader,
    agents: Vec<Box<dyn AgentFunctions>>,
    pub send_func: Arc<SendFn>,
}

unsafe impl Send for Manager {}

impl Manager {
    pub async fn new(
        usr_req: String,
        send_func: Arc<SendFn>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let position: String = "Manager".to_string();

        let attributes: CoreAgent = CoreAgent {
            objective: "Managing the components of the fragment shader".to_string(),
            position: position.clone(),
            state: AgentState::Discover,
            memory: vec![],
        };

        let shader_description: String = task_request(
            usr_req,
            &position,
            get_func_str!(input_to_goal),
            input_to_goal,
        )
        .await;

        let agents: Vec<Box<dyn AgentFunctions>> = vec![];

        let shader: Shader = Shader {
            shader_description,
            scope: None,
            uniforms: None,
            frag_shader: None,
        };

        Ok(Self {
            attributes,
            agents,
            shader,
            send_func,
        })
    }

    fn add_agent(&mut self, agent: Box<dyn AgentFunctions>) {
        self.agents.push(agent);
    }

    fn create_agents(&mut self) {
        self.add_agent(Box::new(ArtistAgent::new(self as *const Manager)));
    }

    pub fn send_msg(&self, num: u8, msg: String) {
        let arc_func: Arc<Box<dyn Fn(u8, Rc<String>) + Send + Sync>> = self.send_func.get();

        arc_func(num, Rc::new(msg));
    }

    pub async fn execute_all(&mut self) {
        self.create_agents();

        for agent in &mut self.agents {
            let agent_res: Result<(), Box<dyn std::error::Error>> =
                agent.execute(&mut self.shader).await;

            match agent_res {
                Ok(()) => println!("Done creating the shader"),
                Err(_) => println!("Error creating the shader"),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn tests_manager() {
        let usr_req: &str = "create a shader that shows a palm tree in a sunset";

        let send_msg: Arc<Box<dyn Fn(u8, Rc<String>) + Send + Sync>> =
            Arc::new(Box::new(|num: u8, agent_msg: Rc<String>| {}));

        let send_struct = Arc::new(SendFn::new(send_msg));

        let mut manager_ai: Manager = Manager::new(usr_req.to_string(), send_struct)
            .await
            .expect("Error creating the managing agent");

        manager_ai.execute_all().await;
        dbg!(manager_ai.shader);
    }
}
