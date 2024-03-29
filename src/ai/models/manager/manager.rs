use crate::ai::models::agents::agent_traits::{AgentFunctions, Shader};
use crate::ai::models::core_agent::core_agent::{AgentState, CoreAgent};

use crate::ai::ai_functions::ai_manager::input_to_goal;
use crate::ai::helpers::local::task_request;
use crate::ai::models::agents::artist_agent::{self, ArtistAgent};
use crate::ai::models::general::llm::Message;

#[derive(Debug)]
pub struct Manager {
    attributes: CoreAgent,
    shader: Shader,
    agents: Vec<Box<dyn AgentFunctions>>,
}

impl Manager {
    pub async fn new(usr_req: String) -> Result<Self, Box<dyn std::error::Error>> {
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
        })
    }

    fn add_agent(&mut self, agent: Box<dyn AgentFunctions>) {
        self.agents.push(agent);
    }

    fn create_agents(&mut self) {
        self.add_agent(Box::new(ArtistAgent::new()));
    }

    pub async fn execute_all(&mut self) -> &Shader {
        self.create_agents();

        for agent in &mut self.agents {
            let agent_res: Result<(), Box<dyn std::error::Error>> =
                agent.execute(&mut self.shader).await;
        }

        return &self.shader;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn tests_manager() {
        let usr_req: &str = "create a shader that shows a palm tree in a sunset";

        let mut manager_ai: Manager = Manager::new(usr_req.to_string())
            .await
            .expect("Error creating the managing agent");

        manager_ai.execute_all().await;
        dbg!(manager_ai.shader);
    }
}
