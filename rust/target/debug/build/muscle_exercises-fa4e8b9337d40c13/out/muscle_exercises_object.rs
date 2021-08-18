#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataEmpty {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataRequest {
    #[prost(int64, tag = "1")]
    pub id: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MuscleGroup {
    #[prost(int64, tag = "1")]
    pub id: i64,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Exercise {
    #[prost(int64, tag = "1")]
    pub id: i64,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MuscleGroups {
    #[prost(message, repeated, tag = "1")]
    pub muscle_groups: ::prost::alloc::vec::Vec<MuscleGroup>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Exercises {
    #[prost(message, repeated, tag = "1")]
    pub exercises: ::prost::alloc::vec::Vec<Exercise>,
}
#[doc = r" Generated client implementations."]
pub mod data_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct DataClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DataClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> DataClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + Sync + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> DataClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            DataClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        pub async fn all_muscle_groups(
            &mut self,
            request: impl tonic::IntoRequest<super::DataEmpty>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::MuscleGroups>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/muscle_exercises_object.Data/AllMuscleGroups",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        pub async fn all_exercises(
            &mut self,
            request: impl tonic::IntoRequest<super::DataEmpty>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::Exercises>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/muscle_exercises_object.Data/AllExercises");
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        pub async fn exercises_for_muscle_group(
            &mut self,
            request: impl tonic::IntoRequest<super::DataRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::Exercises>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/muscle_exercises_object.Data/ExercisesForMuscleGroup",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        pub async fn muscle_groups_for_exercise(
            &mut self,
            request: impl tonic::IntoRequest<super::DataRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::MuscleGroups>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/muscle_exercises_object.Data/MuscleGroupsForExercise",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod data_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with DataServer."]
    #[async_trait]
    pub trait Data: Send + Sync + 'static {
        #[doc = "Server streaming response type for the AllMuscleGroups method."]
        type AllMuscleGroupsStream: futures_core::Stream<Item = Result<super::MuscleGroups, tonic::Status>>
            + Send
            + Sync
            + 'static;
        async fn all_muscle_groups(
            &self,
            request: tonic::Request<super::DataEmpty>,
        ) -> Result<tonic::Response<Self::AllMuscleGroupsStream>, tonic::Status>;
        #[doc = "Server streaming response type for the AllExercises method."]
        type AllExercisesStream: futures_core::Stream<Item = Result<super::Exercises, tonic::Status>>
            + Send
            + Sync
            + 'static;
        async fn all_exercises(
            &self,
            request: tonic::Request<super::DataEmpty>,
        ) -> Result<tonic::Response<Self::AllExercisesStream>, tonic::Status>;
        #[doc = "Server streaming response type for the ExercisesForMuscleGroup method."]
        type ExercisesForMuscleGroupStream: futures_core::Stream<Item = Result<super::Exercises, tonic::Status>>
            + Send
            + Sync
            + 'static;
        async fn exercises_for_muscle_group(
            &self,
            request: tonic::Request<super::DataRequest>,
        ) -> Result<tonic::Response<Self::ExercisesForMuscleGroupStream>, tonic::Status>;
        #[doc = "Server streaming response type for the MuscleGroupsForExercise method."]
        type MuscleGroupsForExerciseStream: futures_core::Stream<Item = Result<super::MuscleGroups, tonic::Status>>
            + Send
            + Sync
            + 'static;
        async fn muscle_groups_for_exercise(
            &self,
            request: tonic::Request<super::DataRequest>,
        ) -> Result<tonic::Response<Self::MuscleGroupsForExerciseStream>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct DataServer<T: Data> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Data> DataServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for DataServer<T>
    where
        T: Data,
        B: Body + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/muscle_exercises_object.Data/AllMuscleGroups" => {
                    #[allow(non_camel_case_types)]
                    struct AllMuscleGroupsSvc<T: Data>(pub Arc<T>);
                    impl<T: Data> tonic::server::ServerStreamingService<super::DataEmpty> for AllMuscleGroupsSvc<T> {
                        type Response = super::MuscleGroups;
                        type ResponseStream = T::AllMuscleGroupsStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DataEmpty>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).all_muscle_groups(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AllMuscleGroupsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/muscle_exercises_object.Data/AllExercises" => {
                    #[allow(non_camel_case_types)]
                    struct AllExercisesSvc<T: Data>(pub Arc<T>);
                    impl<T: Data> tonic::server::ServerStreamingService<super::DataEmpty> for AllExercisesSvc<T> {
                        type Response = super::Exercises;
                        type ResponseStream = T::AllExercisesStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DataEmpty>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).all_exercises(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AllExercisesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/muscle_exercises_object.Data/ExercisesForMuscleGroup" => {
                    #[allow(non_camel_case_types)]
                    struct ExercisesForMuscleGroupSvc<T: Data>(pub Arc<T>);
                    impl<T: Data> tonic::server::ServerStreamingService<super::DataRequest>
                        for ExercisesForMuscleGroupSvc<T>
                    {
                        type Response = super::Exercises;
                        type ResponseStream = T::ExercisesForMuscleGroupStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DataRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).exercises_for_muscle_group(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ExercisesForMuscleGroupSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/muscle_exercises_object.Data/MuscleGroupsForExercise" => {
                    #[allow(non_camel_case_types)]
                    struct MuscleGroupsForExerciseSvc<T: Data>(pub Arc<T>);
                    impl<T: Data> tonic::server::ServerStreamingService<super::DataRequest>
                        for MuscleGroupsForExerciseSvc<T>
                    {
                        type Response = super::MuscleGroups;
                        type ResponseStream = T::MuscleGroupsForExerciseStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DataRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).muscle_groups_for_exercise(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MuscleGroupsForExerciseSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.server_streaming(method, req).await;
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
    impl<T: Data> Clone for DataServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: Data> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Data> tonic::transport::NamedService for DataServer<T> {
        const NAME: &'static str = "muscle_exercises_object.Data";
    }
}
