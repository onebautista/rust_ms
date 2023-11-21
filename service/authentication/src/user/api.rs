use tonic::{Request, Response, Status};
use tonic::transport::Channel;

use crate::user::grpc_user::{
  UserByCredentialsRequest, 
  UserByCredentialsResponse,
  UserByIdRequest,
  UserByIdResponse,
  UpdateUserLoginSessionRequest,
  EmptyResponse
};
use crate::user::grpc_user::user_service_client::UserServiceClient;

#[tonic::async_trait]
pub trait UserApi {
  async fn get_user_by_id(&self, request: Request<UserByIdRequest>) -> Result<Response<UserByIdResponse>, Status>;
  async fn get_user_by_credentials(&self, request: Request<UserByCredentialsRequest>) -> Result<Response<UserByCredentialsResponse>, Status>;
  async fn update_user_login_session(&self, request: Request<UpdateUserLoginSessionRequest>) -> Result<Response<EmptyResponse>, Status>;
}

pub struct  UserApiImpl {
  client: UserServiceClient<Channel>, 
}

impl UserApiImpl { 
  pub fn new(client: UserServiceClient<Channel>) -> Box<dyn UserApi + Send + Sync> {
    Box::new(UserApiImpl { client })
  }
}

#[tonic::async_trait]
impl UserApi for UserApiImpl {
  async fn get_user_by_id(&self, request: Request<UserByIdRequest>) -> Result<Response<UserByIdResponse>, Status> {
    let UserByIdRequest { id_user } = request.into_inner();
    if !id_user.is_positive() {
        return status::Status::invalid_arguments(vec!["id_user"]);
    }

    self.client
        .clone()
        .get_user_by_id(
            Request::new(
                UserByIdRequest { id_user }
            )
        ).await
  }

  async fn get_user_by_credentials(&self, request: Request<UserByCredentialsRequest>) -> Result<Response<UserByCredentialsResponse>, Status> {
    println!("user api ms {:?}", request);
       
    let UserByCredentialsRequest { username_or_email, user_password } = request.into_inner();
    if username_or_email.is_empty() || user_password.is_empty() {
        return status::Status::invalid_arguments(vec!["username_or_email....", "user_password"]);
    }

    self.client
        .clone()
        .get_user_by_credentials(
            Request::new(
                UserByCredentialsRequest {
                    username_or_email,
                    user_password,
                }
            )
        ).await
  }

  async fn update_user_login_session(&self, request: Request<UpdateUserLoginSessionRequest>) -> Result<Response<EmptyResponse>, Status> {
    let UpdateUserLoginSessionRequest { username_or_email } = request.into_inner();
    if username_or_email.is_empty() {
      return status::Status::invalid_arguments(vec!["username_or_email"]);
  }

  self.client
      .clone()
      .update_user_login_session(
          Request::new(
            UpdateUserLoginSessionRequest { username_or_email }
          )
      ).await
  }

}
