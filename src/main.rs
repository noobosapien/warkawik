use helpers::command_line::get_response;

mod ai_functions;
mod api;
mod helpers;
mod models;

fn main() {
    let usr_request: String = get_response("Define your perfect shader: ");
    dbg!(usr_request);
}
