pub mod grpc;
pub mod modelserver;
pub mod web;

use futures::future::{AbortHandle, Abortable};
use grpc::ModelServerImpl;
use modelserver::{
    model_server_server::ModelServerServer, Model, SaveFormat,
};
use prost::Message;
use rouille::{Request, Response};
use std::{path, sync::RwLock};
use tokio::sync::{broadcast, mpsc};
use tonic::transport::Server;
use tracing::info;
use types::{prelude::*, utility, State};

type Handle<T> = Arc<RwLock<Vec<T>>>;
type ModelHandle = Handle<Arc<Model>>;

const SAVES_DIR: &str = "saves/";
const SAVE_FILENAME: &str = "savefile";

// pub fn static_server(path_to_save_file: path::PathBuf) -> Result<()> {
//     let save: SaveFormat =
//         serde_json::from_slice(&fs::read(path_to_save_file)?)?;

//     let vec = save.models.iter().map(|(_, m)| m.clone()).collect();
//     let model_handle = Arc::new(RwLock::new(vec));
//     let read_if_set = utility::ReadIfSet::default();
//     read_if_set.set(save.visual)?;

//     server(model_handle, Arc::new(read_if_set));
//     Ok(())
// }

pub fn spawn(
    state_recv: mpsc::UnboundedReceiver<State>,
    save_signal: mpsc::UnboundedReceiver<Option<String>>,
) {
    std::thread::spawn(move || {
        let model_list =
            Arc::new(RwLock::new(Vec::with_capacity(100)));
        let visual_read_if_set =
            Arc::new(utility::ReadIfSet::default());
        std::fs::create_dir_all(SAVES_DIR)
            .expect("Couldn't create saves directory");

        let (model_broadcast, _recv) = tokio::sync::broadcast::channel(10);
        drop(_recv);

        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                let grpc_impl = ModelServerImpl {
                    models: model_list.clone(),
                    visual: visual_read_if_set.clone(),
                    model_broadcast_spawner: model_broadcast.clone()
                };
                let grpc_handle = tokio::task::spawn(async {
                    let greeter = ModelServerServer::new(grpc_impl);
                    Server::builder()
                        .accept_http1(true)
                        .add_service(tonic_web::enable(greeter))
                        .serve("0.0.0.0:50051".parse().unwrap())
                        .await
                        .unwrap();
                });
                let model_worker_handle = start_model_worker(
                    state_recv,
                    model_list.clone(),
                    model_broadcast.clone(),
                    visual_read_if_set.clone(),
                );
                let save_on_signal_worker_handle =
                    start_save_on_signal_worker(
                        save_signal,
                        model_list.clone(),
                        visual_read_if_set.clone(),
                    );
                grpc_handle.await.unwrap();
                model_worker_handle.abort();
                save_on_signal_worker_handle.abort();
            });
    });
}

fn start_save_on_signal_worker(
    mut signal: mpsc::UnboundedReceiver<Option<String>>,
    model_list: ModelHandle,
    visual_reader: Arc<utility::ReadIfSet<modelserver::RGraph>>,
) -> AbortHandle {
    let worker = async move {
        while let Some(file_name) = signal.recv().await {
            let file_name: &str =
                file_name.as_ref().map(|s| s.as_str()).unwrap_or("");
            if let Err(e) = save_state_to_file(
                file_name,
                model_list.clone(),
                visual_reader.clone(),
            ) {
                error!(
                    "Save signal worker failed, e= {}",
                    e.to_string()
                );
            }
        }
    };
    info!("Starting save signal worker");
    let (handle, registration) = AbortHandle::new_pair();
    tokio::task::spawn(Abortable::new(worker, registration));
    handle
}

// pub fn server(
//     model_list: ModelHandle,
//     visual_reader: Arc<utility::ReadIfSet<modelserver::RGraph>>,
// ) {
//     let model_list = model_list.clone();
//     rouille::Server::new("0.0.0.0:3030", move |request| {
//         debug!("Received request, url={}", request.url());
//         let response = rouille::router!(request,
//             (GET) (/state) => {
//                 model_handler(request, model_list.read().unwrap().len()-1, model_list.clone())
//             },
//             (GET) (/state/{tick: usize}) => {
//                 model_handler(request, tick, model_list.clone())
//             },
//             (GET) (/rgraph) => {
//                 match visual_reader.get() {
//                     Some(rgraph) => Response::json(&rgraph),
//                     None => {
//                         error!("Expected rgraph to be initialized");
//                         Response::text("Expected rgraph to be initialized").with_status_code(500)
//                     }
//                 }
//             },
//             (POST) (/save) => { save_handler("", model_list.clone(), visual_reader.clone()) },
//             (POST) (/save/{path: String}) => { save_handler(&path, model_list.clone(), visual_reader.clone()) },
//             _ => {
//                 warn!("Unrecognized path {}", request.url());
//                 Response::empty_404()
//         });
//         response
//             .with_additional_header("Access-Control-Allow-Origin", "*")
//             .with_additional_header("Access-Control-Allow-Headers", "*")
//     }).unwrap().run();
// }

fn save_state_to_file(
    file_name: &str,
    model_list: ModelHandle,
    visual_reader: Arc<utility::ReadIfSet<modelserver::RGraph>>,
) -> Result<modelserver::SaveFormat> {
    let visual = visual_reader
        .get()
        .ok_or(anyhow::Error::msg("Visual not set yet"))?;
    let model_list = model_list
        .read()
        .map_err(|e| anyhow::Error::msg(e.to_string()))?;
    let models: HashMap<_, _> = model_list
        .iter()
        .cloned()
        .map(|m| (m.tick.clone(), m))
        .collect();

    let save = modelserver::SaveFormat {
        models,
        visual: Some(visual.clone()),
    };

    let dir = path::Path::new(SAVES_DIR);
    let mut path = if !file_name.is_empty() {
        dir.join(file_name)
    } else {
        let num = fs::read_dir(SAVES_DIR)
            .context("Reading saves directory")?
            .into_iter()
            .filter_map(|e| e.ok())
            .filter_map(|entry| {
                let path = entry.path();
                let mut parts =
                    path.file_stem()?.to_str()?.split("--");
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
    fs::write(path, save.encode_to_vec())?;
    Ok(save)
}

// fn save_handler(
//     file_name: &str,
//     model_list: ModelHandle,
//     visual_reader: Arc<utility::ReadIfSet<modelserver::RGraph>>,
// ) -> Response {
//     match save_state_to_file(file_name, model_list, visual_reader) {
//         Ok(r) => Response::json(&r),
//         Err(e) => Response::text(format!(
//             "Error saving file to desired path. Error= {}",
//             e
//         )),
//     }
// }

// fn model_handler(
//     _req: &Request,
//     tick: usize,
//     model_map: ModelHandle,
// ) -> Response {
//     match &model_map
//         .read()
//         .expect("couldn't get model map lock")
//         .get(tick)
//     {
//         Some(model) => {
//             info!("Sending model at tick {}", tick);
//             Response::json(model)
//         }
//         None => {
//             warn!("Model at tick {} not found", tick);
//             Response::empty_204()
//         }
//     }
// }

pub fn start_model_worker(
    mut state: mpsc::UnboundedReceiver<State>,
    model_list: ModelHandle,
    model_stream_sender: broadcast::Sender<Arc<Model>>,
    rgraph: Arc<types::utility::ReadIfSet<modelserver::RGraph>>,
) -> AbortHandle {
    let (abort_handle, abort_registration) = AbortHandle::new_pair();
    let model_worker = Abortable::new(
        async move {
            while let Some(val) = state.recv().await {
                info!("Updating model at tick {}", &val.tick.0);
                rgraph
                    .set_with_if_unset(|| web::state_to_rgraph(&val));

                let model = Arc::new(web::state_to_model(&val));
                let mut write = model_list
                    .write()
                    .expect("couldn't get model map lock");
                if write.len() != model.tick as usize {
                    error!(
                        "dropped a model! len {}, tick {}",
                        write.len(),
                        model.tick
                    );
                }
                write.push(model.clone());
                model_stream_sender.send(model);
            }
        },
        abort_registration,
    );
    info!("Starting model worker task");
    tokio::task::spawn(model_worker);
    abort_handle
}
