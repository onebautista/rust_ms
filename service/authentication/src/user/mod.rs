pub mod entity;
pub mod mapper;
pub mod repository;
pub mod api;

pub mod grpc_user {
    tonic::include_proto!("user");
}