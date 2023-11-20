
use tonic::{transport::Server, Request, Response, Status};
//use std::env;
use std::net::SocketAddr;
extern crate uuid;

mod user;
mod password;

pub mod grpc_movie {
    tonic::include_proto!("movie");
}
use grpc_movie::movie_server::{Movie, MovieServer};
use grpc_movie::{MovieRequest, MovieResponse};
#[derive(Debug, Default)]
pub struct MovieService {}

#[tonic::async_trait]
impl Movie for MovieService {
    async fn get_movies(
        &self,
        request: Request<MovieRequest>,
    ) -> Result<Response<MovieResponse>, Status> {
        println!("Got a request: {:?}", request);

        let mut movies = Vec::new();
        movies.push(grpc_movie::MovieItem {
            id: 1,
            title: "Matrix".to_string(),
            genre: "Sci-Fi".to_string(),
            
        });

        let reply = grpc_movie::MovieResponse { movies };

        Ok(Response::new(reply))
    }
}

const SERVICE_NAME: &str = "user";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = configuration::Config::new("service/.env", SERVICE_NAME)?;
    println!("{:?}", cfg.service_port);
    let conn = db::db_connection();
    
    let movie = MovieService::default();
    let movie = MovieServer::new(movie);
    let movie = tonic_web::enable(movie);
    
    let hasher = password::hasher::DefaultHasher::new();
    let repository = user::repository::UserRepositoryImpl::new(conn);
    let interactor = user::interactor::UserInteractorImpl::new(hasher, repository);
    let service = user::service::UserServiceImpl::new(interactor);
    
    
    let server_addr = SocketAddr::new(cfg.service_hostname.unwrap().parse().unwrap(), cfg.service_port.unwrap().parse().unwrap());
    Server::builder()
        .add_service(user::grpc_user::user_service_server::UserServiceServer::new(service))
        .add_service(movie)
        .serve(server_addr)
        .await?;
    
    Ok(())
}

