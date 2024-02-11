#[macro_export]
macro_rules! get_func_str {
    ($func: ident) => {{
        stringify!($func)
    }};
}
#[macro_use]
use std::collections::HashMap;
use serde_json::json;
use std::rc::Rc;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

use futures_util::{SinkExt, StreamExt, TryFutureExt};
use tokio::sync::{mpsc, RwLock};
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::ws::{Message, WebSocket};
use warp::Filter;

use helpers::send_func::SendFn;
use models::manager::manager::Manager;

mod ai_functions;
mod helpers;
mod llm_api;
mod models;

static NEXT_USER_ID: AtomicUsize = AtomicUsize::new(1);

type Users = Arc<RwLock<HashMap<usize, mpsc::UnboundedSender<Message>>>>;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let users = Users::default();
    let users = warp::any().map(move || users.clone());

    let index = warp::path("agent")
        .and(warp::ws())
        .and(users)
        .map(|ws: warp::ws::Ws, users| ws.on_upgrade(move |socket| user_connected(socket, users)));

    let routes = index;

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

async fn user_connected(ws: WebSocket, users: Users) {
    let my_id = NEXT_USER_ID.fetch_add(1, Ordering::Relaxed);

    let (mut user_ws_tx, mut user_ws_rx) = ws.split();

    let (tx, rx) = mpsc::unbounded_channel();
    let mut rx = UnboundedReceiverStream::new(rx);

    tokio::task::spawn(async move {
        while let Some(message) = rx.next().await {
            user_ws_tx
                .send(message)
                .unwrap_or_else(|e| {
                    eprintln!("websocket send error: {}", e);
                })
                .await;
        }
    });

    users.write().await.insert(my_id, tx);

    while let Some(result) = user_ws_rx.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("websocket error(uid={}): {}", my_id, e);
                break;
            }
        };
        user_message(my_id, msg, &users).await;
    }

    user_disconnected(my_id, &users).await;
}

async fn user_message(my_id: usize, msg: Message, users: &Users) {
    let msg = if let Ok(s) = msg.to_str() {
        s
    } else {
        return;
    };

    for (&uid, tx) in users.read().await.iter() {
        let strong_tx = tx.downgrade().clone().upgrade();

        let send_msg: Arc<Box<dyn Fn(u8, Rc<String>) + Send + Sync>>;

        if let Some(custom_tx) = strong_tx {
            send_msg = Arc::new(Box::new(move |num: u8, agent_msg: Rc<String>| {
                let mut agent_msg_str: String = (*agent_msg).to_string();

                let value_json = json!({
                    "status": num,
                    "content": agent_msg_str,
                });

                let agent_str = value_json.to_string();

                if let Err(_disconnected) = custom_tx.send(Message::text(agent_str)) {
                    println!("Client unable to reach.");
                }
            }));
        } else {
            send_msg = Arc::new(Box::new(move |num: u8, agent_msg: Rc<String>| {
                let agent_str = (*agent_msg).to_string();

                println!("{}", agent_str);
            }));
        }

        let send_struct = Arc::new(SendFn::new(send_msg));

        if my_id == uid {
            let send_msg_to_thread = Arc::clone(&send_struct);
            let mut managing_agent: Manager = Manager::new(msg.to_string(), send_msg_to_thread)
                .await
                .expect("Error creating the managing agent.");

            managing_agent.execute_all().await;
        }
    }
}

async fn user_disconnected(my_id: usize, users: &Users) {
    eprintln!("good bye user: {}", my_id);

    users.write().await.remove(&my_id);
}
