use super::command_line::PrintCommand;
use crate::{llm_api::calls::call_gpt, models::general::llm::Message};
use reqwest::Client;
use serde::de::DeserializeOwned;
use serde_json;
use std::{fmt::format, fs};

pub const FRAG_SHADER_TEMPLATE: &str = "../../static/template.frag";
pub const FRAG_SHADER_MAIN: &str = "../../static/shader.frag";
pub const STATIC_PATH: &str = "../../static/";

//Extends the given functions output by adding more context to what is actually needed from the LLM.
pub fn extend_function(func: fn(&str) -> &'static str, input: &str) -> Message {
    let func_str: &str = func(input);

    let msg: String = format!(
        "FUNCTION: {}
    INSTRUCTION: You ONLY print the function. You only print the results of the functions.
    NOTHING ELSE. NO COMMENTARY OR WHAT TYPE OF CODE IT IS. Just print the function. This is the input of the function: {}.
    Print what the function will return.
    ",
        func_str, input
    );

    Message {
        role: String::from("system"),
        content: msg,
    }
}

//Agents call this function which extends the function to get only the code first then calls the LLM.
pub async fn task_request(
    msg_context: String,
    agent_position: &str,
    agent_operation: &str,
    function_pass: for<'a> fn(&'a str) -> &'static str,
) -> String {
    let extended_msg: Message = extend_function(function_pass, msg_context.as_str());

    PrintCommand::Primary.print_msg(agent_position, agent_operation);

    let llm_res: Result<String, Box<dyn std::error::Error + Send>> =
        call_gpt(vec![extended_msg.clone()]).await;

    match llm_res {
        Ok(llm_r) => llm_r,
        Err(_) => call_gpt(vec![extended_msg.clone()])
            .await
            .expect("Failed to call OpenAI."),
    }
}

//Decode what is returned by the task_request function.
pub async fn task_request_decoded<T: DeserializeOwned>(
    msg_context: String,
    agent_position: &str,
    agent_operation: &str,
    function_pass: for<'a> fn(&'a str) -> &'static str,
) -> T {
    let llm_res: String =
        task_request(msg_context, agent_position, agent_operation, function_pass).await;

    let decoded: T =
        serde_json::from_str(&llm_res.as_str()).expect("Failed to decode the response by OpenAI.");

    decoded
}

pub async fn check_status(client: &Client, url: &str) -> Result<u16, reqwest::Error> {
    let res: reqwest::Response = client.get(url).send().await?;
    Ok(res.status().as_u16())
}

pub fn read_template() -> String {
    let path: String = FRAG_SHADER_TEMPLATE.to_string();
    fs::read_to_string(path).expect("Couldn't read the Fragment shader template file.")
}

pub fn read_frag_shader() -> String {
    let path: String = FRAG_SHADER_MAIN.to_string();
    fs::read_to_string(path).expect("Couldn't read the Fragment shader main file.")
}

pub fn save_frag_file(frag: &String) {
    let path: String = STATIC_PATH.to_string();
    fs::write(path, frag).expect("Couldn't write the Fragment shader file.")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai_functions::ai_manager::input_to_goal;

    #[test]
    fn tests_extend_function() {
        let res = extend_function(input_to_goal, "Nothing specific");
        assert_eq!(res.role, "system".to_string());
    }

    #[tokio::test]
    async fn tests_task_request() {
        let param: String =
            "Need to create an energy shield which changes color with time.".to_string();

        let res = task_request(param, "Manager", "Getting user input", input_to_goal).await;

        dbg!(&res);
        assert!(res.len() > 10);
    }
}
