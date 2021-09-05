pub mod web;

use types::prelude::*;
use std::sync::RwLock;
use types::State;
use tracing::{info};
use futures::future::{AbortHandle, Abortable};
use rouille::{Response, Request, router};
use tokio::sync::{mpsc};
use crate::web::Model;

type ModelHandle = Arc<RwLock<Vec<Model>>>;

pub fn start_model_worker(
    mut state: mpsc::UnboundedReceiver<State>,
    model_list: ModelHandle,
    state_list: Arc<RwLock<Vec<State>>>,
) -> AbortHandle {
    let (abort_handle, abort_registration) = AbortHandle::new_pair();
    let model_worker = Abortable::new(async move {
        while let Some(val) = state.recv().await {
            let model = web::state_to_model(&val);
            state_list.write().unwrap().push(val);
            let mut write = model_list.write().expect("couldn't get model map lock");
            if write.len() != model.tick as usize {
                error!("dropped a model! len {}, tick {}", write.len(), model.tick);
            }
            write.push(model);
        }
    }, abort_registration);
    info!("Starting model worker task");
    tokio::task::spawn(model_worker);
    abort_handle
}

pub async fn server(state: mpsc::UnboundedReceiver<State>) -> anyhow::Result<()> {
    let model_list = Arc::new(RwLock::new(Vec::with_capacity(100)));
    let state_list = Arc::new(RwLock::new(Vec::with_capacity(100)));
    let model_worker_handle = start_model_worker(state, model_list.clone(), state_list.clone());

    let server_handle = tokio::task::spawn_blocking(move || {
        let state_list = state_list.clone();
        let model_list = model_list.clone();
        rouille::Server::new("0.0.0.0:3030", move |request| {
            info!("Received request: {:?}", &request);
            rouille::router!(request,
                (GET) (/state) => {
                    model_handler(request, model_list.read().unwrap().len()-1, model_list.clone())
                },
                (GET) (/state/{tick: usize}) => {
                    model_handler(request, tick, model_list.clone())
                },
                (GET) (/rgraph) => {
                    let state_list = state_list.read().unwrap();
                    match state_list.last() {
                        Some(state) => Response::json(&web::state_to_rgraph(state)),
                        None => {
                            error!("Expected non-empty state_list");
                            Response::text("Expected non-empty state list")
                            .with_status_code(500)
                        }
                    }
                },
                _ => {
                    info!("Unrecognized path {}", request.url());
                    Response::empty_404()
            })
                .with_additional_header("Access-Control-Allow-Origin", "*")
                .with_additional_header("Access-Control-Allow-Headers", "*")
        })
            .unwrap().run();
    });
    server_handle.await?;
    model_worker_handle.abort();
    Ok(())
}

fn model_handler(_req: &Request, tick: usize, model_map: ModelHandle) -> Response {
    match &model_map.read().expect("couldn't get model map lock").get(tick) {
        Some(model) => {
            info!("Sending model at tick {}", tick);
            Response::json(model)
        }
        None => {
            warn!("Model at tick {} not found", tick);
            Response::empty_404()
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
