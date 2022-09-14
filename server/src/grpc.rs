use crate::{
    modelserver::{model_server_server::ModelServer, *},
    ModelHandle,
};
use futures::{StreamExt, TryStreamExt};
use tokio_stream::wrappers::BroadcastStream;
use std::sync::{Arc, RwLock};
use tonic::Response;
use tracing::info;
use types::utility::ReadIfSet;

pub struct ModelServerImpl {
    pub models: ModelHandle,
    pub visual: Arc<ReadIfSet<RGraph>>,
    pub model_broadcast_spawner:
        tokio::sync::broadcast::Sender<Arc<Model>>,
}

#[tonic::async_trait]
impl ModelServer for ModelServerImpl {
    async fn get_latest_model(
        &self,
        request: tonic::Request<crate::modelserver::LatestModelReq>,
    ) -> Result<
        tonic::Response<crate::modelserver::Model>,
        tonic::Status,
    > {
        info!("Got a request: {:?}", request);
        let models: &[Arc<Model>] = &self
            .models
            .read()
            .map_err(|e: _| tonic::Status::internal(e.to_string()))?;
        let last = models.last().ok_or_else(|| {
            tonic::Status::internal("No models found")
        })?;
        Ok(Response::new(Model::clone(last)))
    }

    async fn get_model(
        &self,
        request: tonic::Request<crate::modelserver::ModelReq>,
    ) -> Result<
        tonic::Response<crate::modelserver::Model>,
        tonic::Status,
    > {
        info!("Got a request: {:?}", request);
        let ModelReq { tick } = request.into_inner();
        let models: &[Model] = &self
            .models
            .read()
            .map_err(|e: _| tonic::Status::internal(e.to_string()))?;
        let model = models.get(tick as usize).ok_or_else(|| {
            tonic::Status::internal(format!(
                "Model {} not found",
                tick
            ))
        })?;
        Ok(Response::new(Model::clone(model)))
    }

    async fn get_visual(
        &self,
        request: tonic::Request<crate::modelserver::VisualReq>,
    ) -> Result<
        tonic::Response<crate::modelserver::RGraph>,
        tonic::Status,
    > {
        info!("Got a request: {:?}", request);
        let visual = self.visual.get().ok_or_else(|| {
            tonic::Status::internal("No visual set at this time")
        })?;
        Ok(Response::new(visual.clone()))
    }

    type SubModelsStream = futures::stream::Map<BroadcastStream<Arc<Model>>, impl Fn(Arc<Model>) -> Result<Model, tonic::Status>>;

    async fn sub_models(
        &self,
        request: tonic::Request<crate::modelserver::SubModelReq>,
    ) -> Result<tonic::Response<Self::SubModelsStream>, tonic::Status>
    {
        info!("Got a request: {:?}", request);
        let rx_stream: BroadcastStream<Arc<Model>> =
            BroadcastStream::new(
                self.model_broadcast_spawner.subscribe(),
            );
        let mapped: _ = rx_stream
            .map(|m| Ok(Model::clone(&m)))
            .map_err(|e| tonic::Status::internal(e.to_string()));
        Ok(Response::new(mapped))
    }
}