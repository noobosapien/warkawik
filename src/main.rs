#[macro_export]
macro_rules! get_func_str {
    ($func: ident) => {{
        stringify!($func)
    }};
}
#[macro_use]
use helpers::command_line::get_response;

mod ai_functions;
mod helpers;
mod llm_api;
mod models;

fn main() {
    let usr_request: String = get_response("Define your perfect shader: ");
    dbg!(usr_request);
}
