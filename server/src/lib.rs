pub mod web;

use crate::web::Model;
use futures::future::{AbortHandle, Abortable};
use rouille::{router, Request, Response};
use std::sync::RwLock;
use tokio::sync::mpsc;
use tracing::info;
use types::prelude::*;
use types::State;

type Handle<T> = Arc<RwLock<Vec<T>>>;
type ModelHandle = Handle<Model>;

const SAVES_DIR: &str = "saves/";
const SAVE_FILENAME: &str = "savefile";

pub fn spawn(state_rx: mpsc::UnboundedReceiver<State>) {
    std::thread::spawn(move || {
        match tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(server(state_rx)) {
            Ok(_) => {}
            Err(e) => {
                error!("Error encountered in server. {}", e);
            }
        }
    });
}

pub async fn server(state: mpsc::UnboundedReceiver<State>) -> anyhow::Result<()> {
    let model_list = Arc::new(RwLock::new(Vec::with_capacity(100)));
    let state_list = Arc::new(RwLock::new(Vec::with_capacity(100)));
    let model_worker_handle = start_model_worker(state, model_list.clone(), state_list.clone());

    std::fs::create_dir_all(SAVES_DIR)?;

    let server_handle = tokio::task::spawn_blocking(move || {
        let state_list = state_list.clone();
        let model_list = model_list.clone();
        rouille::Server::new("0.0.0.0:3030", move |request| {
            info!("Received request: {:?}", &request);
            let response = rouille::router!(request,
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
                (POST) (/save) => {
                    match save_handler("", model_list.clone(), state_list.clone()) {
                        Ok(r) => r,
                        Err(e) => Response::text(format!("Error saving file to desired path. Error={}", e))
                    }
                },
                (POST) (/save/{path: String}) => {
                    match save_handler(&path, model_list.clone(), state_list.clone()) {
                        Ok(r) => r,
                        Err(e) => Response::text(format!("Error saving file to desired path. Path={}, Error={}", path, e))
                    }
                },
                _ => {
                    info!("Unrecognized path {}", request.url());
                    Response::empty_404()
            });
            response
                .with_additional_header("Access-Control-Allow-Origin", "*")
                .with_additional_header("Access-Control-Allow-Headers", "*")
        })
            .unwrap().run();
    });
    server_handle.await?;
    model_worker_handle.abort();
    Ok(())
}

fn save_handler(
    fname: &str,
    model_list: ModelHandle,
    state_list: Handle<State>,
) -> Result<Response> {
    let model_list = model_list.read().unwrap();
    let mut models = HashMap::with_capacity(model_list.len());
    for m in model_list.iter() {
        models.insert(m.tick, m);
    }
    let save = web::SaveFormat {
        models,
        visual: &web::state_to_rgraph(state_list.read().unwrap().get(0).unwrap()),
    };
    let dir = std::path::Path::new(SAVES_DIR);
    let mut path = if !fname.is_empty() {
        dir.join(fname)
    } else {
        let num = fs::read_dir(SAVES_DIR).context("Reading saves directory")?
            .into_iter()
            .filter_map(|e| e.ok())
            .filter_map(|entry| {
                let path = entry.path();
                let mut parts = path.file_stem()?.to_str()?.split("--");
                if parts.next() != Some(SAVE_FILENAME) {
                    return None;
                }
                parts.next().and_then(|num| num.parse::<u16>().ok())
            })
            .max()
            .unwrap_or(0);
        dir.join(format!("{}--{}", SAVE_FILENAME, num + 1))
    };
    path.set_extension("json");

    debug!("writing file: {:?}", &path);
    fs::write(path, serde_json::to_vec(&save)?)?;
    Ok(Response::json(&save))
}

fn model_handler(_req: &Request, tick: usize, model_map: ModelHandle) -> Response {
    match &model_map
        .read()
        .expect("couldn't get model map lock")
        .get(tick)
    {
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

pub fn start_model_worker(
    mut state: mpsc::UnboundedReceiver<State>,
    model_list: ModelHandle,
    state_list: Arc<RwLock<Vec<State>>>,
) -> AbortHandle {
    let (abort_handle, abort_registration) = AbortHandle::new_pair();
    let model_worker = Abortable::new(
        async move {
            while let Some(val) = state.recv().await {
                let model = web::state_to_model(&val);
                state_list.write().unwrap().push(val);
                let mut write = model_list.write().expect("couldn't get model map lock");
                if write.len() != model.tick as usize {
                    error!("dropped a model! len {}, tick {}", write.len(), model.tick);
                }
                write.push(model);
            }
        },
        abort_registration,
    );
    info!("Starting model worker task");
    tokio::task::spawn(model_worker);
    abort_handle
}
