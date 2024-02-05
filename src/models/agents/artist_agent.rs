use crate::ai_functions::ai_artist::{print_fixed_code, print_frag_shader_code};
use crate::helpers::command_line::{decision_to_proceed, PrintCommand};
use crate::helpers::local::task_request;
use crate::helpers::local::{
    check_status, read_frag_shader, read_template, save_frag_file, STATIC_PATH,
};
use crate::models::core_agent::core_agent::{AgentState, CoreAgent};

use crate::models::agents::agent_traits::{AgentFunctions, RouteObject, Shader};

use async_trait::async_trait;
use core::panic;
use reqwest::Client;
use std::process::{Command, Stdio};
use std::time::Duration;
use tokio::time;
