use crate::models::core_agent::core_traits::CoreTraits;
use crate::models::general::llm::Message;

#[derive(Debug, PartialEq)]
pub enum AgentState {
    Discover,
    Working,
    Testing,
    Finished,
}

#[derive(Debug)]
pub struct CoreAgent {
    pub objective: String,
    pub position: String,
    pub state: AgentState,
    pub memory: Vec<Message>,
}

impl CoreTraits for CoreAgent {
    fn new(objective: String, position: String) -> Self {
        CoreAgent {
            objective,
            position,
            state: AgentState::Discover,
            memory: vec![],
        }
    }

    fn update_state(&mut self, new_state: AgentState) {
        self.state = new_state;
    }

    fn get_objective(&self) -> &String {
        &self.objective
    }

    fn get_position(&self) -> &String {
        &self.position
    }

    fn get_state(&self) -> &AgentState {
        &self.state
    }

    fn get_memory(&self) -> &Vec<Message> {
        &self.memory
    }
}
