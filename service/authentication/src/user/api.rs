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
  async fn get_user_by_id(&self, request: Request<UserByIdRequest>) -> Result<Response<UserByIdResponse>, Error>;
  async fn get_user_by_credentials(&self, request: Request<UserByCredentialsRequest>) -> Result<Response<UserByCredentialsResponse>, Error>;
  async fn update_login_session_to_db(&self, request: Request<UpdateUserLoginSessionRequest>) -> Result<Response<EmptyResponse>, Error>;
}

pub struct  UserApiImpl {
  client: UserServiceClient<Channel>, 
}

impl UserApiImpl { 
  fn new(client: UserServiceClient<Channel>) -> Box<dyn UserApi + Send + Sync> {
    Box::new(UserApiImpl { client })
  }
}

#[tonic::async_trait]
impl UserApi for UserApiImpl {
  async fn get_user_by_id(&self, request: Request<UserByIdRequest>) -> Result<Response<UserByIdResponse>, Status> {
    let UserByIdRequest { id_user } = request.into_inner();
    if id_user.is_empty() {
        return status::Status::invalid_arguments(vec!["id"]);
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
    let UserByCredentialsRequest { username_or_email, user_password } = request.into_inner();
    if username_or_email.is_empty() || user_password.is_empty() {
        return status::Status::invalid_arguments(vec!["username_or_email", "user_password"]);
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

  async fn update_login_session_to_db(&self, request: Request<UpdateUserLoginSessionRequest>) -> Result<Response<EmptyResponse>, Status> {
    let UpdateUserLoginSessionRequest { username_or_email } = request.into_inner();
    if username_or_email.is_empty() {
      return status::Status::invalid_arguments(vec!["username_or_email"]);
  }

  self.client
      .clone()
      .update_login_session_to_db(
          Request::new(
            EmptyResponse {
                  user_name_or_email
              }
          )
      ).await
  }
}
