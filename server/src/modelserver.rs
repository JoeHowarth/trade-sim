#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubModelReq {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VisualReq {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LatestModelReq {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModelReq {
    #[prost(uint64, tag = "1")]
    pub tick: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Model {
    #[prost(uint64, tag = "1")]
    pub tick: u64,
    #[prost(map = "string, message", tag = "2")]
    pub nodes: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        Node,
    >,
    #[prost(message, repeated, tag = "3")]
    pub edges: ::prost::alloc::vec::Vec<Edge>,
    #[prost(map = "string, message", tag = "4")]
    pub agents: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        Agent,
    >,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Node {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(map = "string, message", tag = "2")]
    pub markets: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        MarketInfo,
    >,
    #[prost(string, repeated, tag = "3")]
    pub links:
        ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Agent {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub cargo: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub location: ::prost::alloc::string::String,
    #[prost(double, tag = "4")]
    pub money: f64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Edge {
    #[prost(string, tag = "1")]
    pub from: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub to: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketInfo {
    #[prost(double, tag = "1")]
    pub supply: f64,
    #[prost(double, tag = "2")]
    pub consumption: f64,
    #[prost(double, tag = "3")]
    pub production: f64,
    #[prost(double, tag = "4")]
    pub price: f64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RGraph {
    #[prost(map = "string, message", tag = "1")]
    pub nodes: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        RNode,
    >,
    #[prost(message, repeated, tag = "2")]
    pub edges: ::prost::alloc::vec::Vec<Edge>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RNode {
    #[prost(int32, tag = "1")]
    pub x: i32,
    #[prost(int32, tag = "2")]
    pub y: i32,
    #[prost(string, tag = "3")]
    pub id: ::prost::alloc::string::String,
    #[prost(float, tag = "4")]
    pub radius: f32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SaveFormat {
    #[prost(map = "uint64, message", tag = "1")]
    pub models: ::std::collections::HashMap<u64, Model>,
    #[prost(message, optional, tag = "2")]
    pub visual: ::core::option::Option<RGraph>,
}
/// Generated client implementations.
pub mod model_server_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::let_unit_value
    )]
    use tonic::codegen::{http::Uri, *};
    #[derive(Debug, Clone)]
    pub struct ModelServerClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ModelServerClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(
            dst: D,
        ) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?
                .connect()
                .await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> ModelServerClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner =
                tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> ModelServerClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<
                        tonic::body::BoxBody,
                    >>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            ModelServerClient::new(InterceptedService::new(
                inner,
                interceptor,
            ))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(
            mut self,
            encoding: CompressionEncoding,
        ) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(
            mut self,
            encoding: CompressionEncoding,
        ) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        pub async fn get_latest_model(
            &mut self,
            request: impl tonic::IntoRequest<super::LatestModelReq>,
        ) -> Result<tonic::Response<super::Model>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/modelserver.ModelServer/GetLatestModel",
            );
            self.inner
                .unary(request.into_request(), path, codec)
                .await
        }
        pub async fn get_model(
            &mut self,
            request: impl tonic::IntoRequest<super::ModelReq>,
        ) -> Result<tonic::Response<super::Model>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/modelserver.ModelServer/GetModel",
            );
            self.inner
                .unary(request.into_request(), path, codec)
                .await
        }
        pub async fn get_visual(
            &mut self,
            request: impl tonic::IntoRequest<super::VisualReq>,
        ) -> Result<tonic::Response<super::RGraph>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/modelserver.ModelServer/GetVisual",
            );
            self.inner
                .unary(request.into_request(), path, codec)
                .await
        }
        pub async fn sub_models(
            &mut self,
            request: impl tonic::IntoRequest<super::SubModelReq>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::Model>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/modelserver.ModelServer/SubModels",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
    }
}
/// Generated server implementations.
pub mod model_server_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::let_unit_value
    )]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with ModelServerServer.
    #[async_trait]
    pub trait ModelServer: Send + Sync + 'static {
        async fn get_latest_model(
            &self,
            request: tonic::Request<super::LatestModelReq>,
        ) -> Result<tonic::Response<super::Model>, tonic::Status>;
        async fn get_model(
            &self,
            request: tonic::Request<super::ModelReq>,
        ) -> Result<tonic::Response<super::Model>, tonic::Status>;
        async fn get_visual(
            &self,
            request: tonic::Request<super::VisualReq>,
        ) -> Result<tonic::Response<super::RGraph>, tonic::Status>;
        ///Server streaming response type for the SubModels method.
        type SubModelsStream: futures_core::Stream<
                Item = Result<super::Model, tonic::Status>,
            > + Send
            + 'static;
        async fn sub_models(
            &self,
            request: tonic::Request<super::SubModelReq>,
        ) -> Result<
            tonic::Response<Self::SubModelsStream>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct ModelServerServer<T: ModelServer> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: ModelServer> ModelServerServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(
            mut self,
            encoding: CompressionEncoding,
        ) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(
            mut self,
            encoding: CompressionEncoding,
        ) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>>
        for ModelServerServer<T>
    where
        T: ModelServer,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/modelserver.ModelServer/GetLatestModel" => {
                    #[allow(non_camel_case_types)]
                    struct GetLatestModelSvc<T: ModelServer>(
                        pub Arc<T>,
                    );
                    impl<T: ModelServer>
                        tonic::server::UnaryService<
                            super::LatestModelReq,
                        > for GetLatestModelSvc<T>
                    {
                        type Response = super::Model;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::LatestModelReq,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner)
                                    .get_latest_model(request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings =
                        self.accept_compression_encodings;
                    let send_compression_encodings =
                        self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetLatestModelSvc(inner);
                        let codec =
                            tonic::codec::ProstCodec::default();
                        let mut grpc =
                            tonic::server::Grpc::new(codec)
                                .apply_compression_config(
                                    accept_compression_encodings,
                                    send_compression_encodings,
                                );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/modelserver.ModelServer/GetModel" => {
                    #[allow(non_camel_case_types)]
                    struct GetModelSvc<T: ModelServer>(pub Arc<T>);
                    impl<T: ModelServer>
                        tonic::server::UnaryService<super::ModelReq>
                        for GetModelSvc<T>
                    {
                        type Response = super::Model;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ModelReq>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_model(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings =
                        self.accept_compression_encodings;
                    let send_compression_encodings =
                        self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetModelSvc(inner);
                        let codec =
                            tonic::codec::ProstCodec::default();
                        let mut grpc =
                            tonic::server::Grpc::new(codec)
                                .apply_compression_config(
                                    accept_compression_encodings,
                                    send_compression_encodings,
                                );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/modelserver.ModelServer/GetVisual" => {
                    #[allow(non_camel_case_types)]
                    struct GetVisualSvc<T: ModelServer>(pub Arc<T>);
                    impl<T: ModelServer>
                        tonic::server::UnaryService<super::VisualReq>
                        for GetVisualSvc<T>
                    {
                        type Response = super::RGraph;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::VisualReq>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_visual(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings =
                        self.accept_compression_encodings;
                    let send_compression_encodings =
                        self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetVisualSvc(inner);
                        let codec =
                            tonic::codec::ProstCodec::default();
                        let mut grpc =
                            tonic::server::Grpc::new(codec)
                                .apply_compression_config(
                                    accept_compression_encodings,
                                    send_compression_encodings,
                                );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/modelserver.ModelServer/SubModels" => {
                    #[allow(non_camel_case_types)]
                    struct SubModelsSvc<T: ModelServer>(pub Arc<T>);
                    impl<T: ModelServer>
                        tonic::server::ServerStreamingService<
                            super::SubModelReq,
                        > for SubModelsSvc<T>
                    {
                        type Response = super::Model;
                        type ResponseStream = T::SubModelsStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::SubModelReq,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).sub_models(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings =
                        self.accept_compression_encodings;
                    let send_compression_encodings =
                        self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubModelsSvc(inner);
                        let codec =
                            tonic::codec::ProstCodec::default();
                        let mut grpc =
                            tonic::server::Grpc::new(codec)
                                .apply_compression_config(
                                    accept_compression_encodings,
                                    send_compression_encodings,
                                );
                        let res =
                            grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: ModelServer> Clone for ModelServerServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self
                    .accept_compression_encodings,
                send_compression_encodings: self
                    .send_compression_encodings,
            }
        }
    }
    impl<T: ModelServer> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(
            &self,
            f: &mut std::fmt::Formatter<'_>,
        ) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ModelServer> tonic::server::NamedService
        for ModelServerServer<T>
    {
        const NAME: &'static str = "modelserver.ModelServer";
    }
}
