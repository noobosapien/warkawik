#[macro_export]
macro_rules! get_func_str {
    ($func: ident) => {{
        stringify!($func)
    }};
}
#[macro_use]
use helpers::command_line::get_response;
use models::manager::manager::Manager;

mod ai_functions;
mod helpers;
mod llm_api;
mod models;

static NEXT_USER_ID: AtomicUsize = AtomicUsize::new(1);

type Users = Arc<RwLock<HashMap<usize, mpsc::UnboundedSender<Message>>>>;

#[tokio::main]
async fn main() {
    let usr_request: String = get_response("Define your perfect shader: ");

    let mut managing_agent: Manager = Manager::new(usr_request)
        .await
        .expect("Error creating the managing agent.");

    managing_agent.execute_all().await;
}
