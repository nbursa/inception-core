#[cfg(feature = "grpc")]
use tonic::{transport::Server, Request, Response, Status};
#[cfg(feature = "grpc")]
use crate::core::memory::{MemorySystem};

#[cfg(feature = "grpc")]
pub mod proto {
    tonic::include_proto!("inception");
}

#[cfg(feature = "grpc")]
#[derive(Default)]
pub struct InceptionGrpc {
    pub memory: std::sync::Arc<MemorySystem>,
}

#[cfg(feature = "grpc")]
#[tonic::async_trait]
impl proto::inception_server::Inception for InceptionGrpc {
    async fn store_token(
        &self,
        request: Request<proto::TokenRequest>,
    ) -> Result<Response<proto::GenericReply>, Status> {
        Ok(Response::new(proto::GenericReply {
            message: "Stored".into(),
        }))
    }

    async fn recall(
        &self,
        request: Request<proto::RecallQuery>,
    ) -> Result<Response<proto::RecallReply>, Status> {
        Ok(Response::new(proto::RecallReply {
            results: vec![],
        }))
    }
}