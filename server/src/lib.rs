pub mod web;

use crate::web::Model;
use futures::future::{AbortHandle, Abortable};
use rouille::{router, Request, Response};
use std::{
    sync::RwLock,
    path,
    cell,
};
use tokio::sync::mpsc;
use tracing::info;
use types::prelude::*;
use types::{
    State,
    Read
};
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::task::JoinHandle;
use std::sync::atomic::Ordering;

type Handle<T> = Arc<RwLock<Vec<T>>>;
type ModelHandle = Handle<Model>;

const SAVES_DIR: &str = "saves/";
const SAVE_FILENAME: &str = "savefile";

pub fn static_server(path_to_save_file: path::PathBuf) -> Result<()> {
    let save: web::SaveFormat = serde_json::from_slice(&fs::read(path_to_save_file)?)?;

    Ok(())
}

pub fn spawn(state_rx: mpsc::UnboundedReceiver<State>) {
    std::thread::spawn(move || {
        let model_list = Arc::new(RwLock::new(Vec::with_capacity(100)));
        // let state_list = Arc::new(RwLock::new(Vec::with_capacity(100)));
        let rgraph = ReadIfSet::default();
        std::fs::create_dir_all(SAVES_DIR)?;

        match tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                let model_worker_handle = start_model_worker(state_rx, model_list.clone(), &rgraph);
                let server_handle = server(model_list, &rgraph);
            }) {
            Ok(_) => {}
            Err(e) => {
                error!("Error encountered in server. {}", e);
            }
        }
    });
}


pub async fn server(model_list: ModelHandle, rgraph: ReadIfSet<web::RGraph>) -> JoinHandle<()> {
    let server_handle = tokio::task::spawn_blocking(move || {
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
                    match rgraph.get() {
                        Some(rgraph) => Response::json(rgraph),
                        None => {
                            error!("Expected rgraph to be initialized");
                            Response::text("Expected rgraph to be initialized").with_status_code(500)
                        }
                    }
                },
                (POST) (/save) => {
                    match save_handler("", model_list.clone(), &rgraph) {
                        Ok(r) => r,
                        Err(e) => Response::text(format!("Error saving file to desired path. Error={}", e))
                    }
                },
                (POST) (/save/{path: String}) => {
                    match save_handler(&path, model_list.clone(), &rgraph) {
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
    server_handle
}

fn save_handler(
    fname: &str,
    model_list: ModelHandle,
    visual: &web::RGraph,
) -> Result<Response> {
    let model_list = model_list.read().unwrap();
    let mut models = HashMap::with_capacity(model_list.len());
    for m in model_list.iter() {
        models.insert(m.tick, m);
    }
    let save = web::SaveFormat {
        models,
        visual,
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
    rgraph: ReadIfSet<web::RGraph>,
) -> AbortHandle {
    let (abort_handle, abort_registration) = AbortHandle::new_pair();
    let model_worker = Abortable::new(
        async move {
            while let Some(val) = state.recv().await {
                rgraph.set_with()

                let model = web::state_to_model(&val);
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
