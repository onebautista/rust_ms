use tonic::Request;
use crate::user;
use crate::user::api::UserApi;
use crate::user::entity::User;
use  crate::user::grpc_user::{
  UserByCredentialsRequest, 
  UserByCredentialsResponse,
  UserByIdRequest,
  UserByIdResponse,
  UpdateUserLoginSessionRequest,
  EmptyResponse
};

type Error = Box<dyn std::error::Error + Send + Sync>;

#[async_trait::async_trait]
pub trait UserRepository {    
  async fn get_user_by_id(&self, id_user: i32) -> Result<Option<User>, Error>;
  async fn get_user_by_credentials(&self, username_or_email: &str, user_password: &str) -> Result<Option<User>, Error>;
  //async fn get_user_by_username_or_email(&self, username_or_email: &str) -> Result<User, Error>;
  async fn update_user_login_session(&self, username_or_email: &str) -> Result<(), Error>;
}

pub struct UserRepositoryImpl {
  api: Box<dyn UserApi + Send + Sync>,
}

impl UserRepositoryImpl {
  pub fn new(api: Box<dyn UserApi + Send + Sync>) -> Box<dyn UserRepository + Send + Sync> {
      Box::new(UserRepositoryImpl { api })
  }
}

#[async_trait::async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn get_user_by_id(&self, id_user: i32) -> Result<Option<User>, Error> {
        let UserByIdResponse { user } = self.api.get_user_by_id(
            Request::new(
                UserByIdRequest { id_user }
            )
        ).await?.into_inner();
        Ok(user.map(|u| u.into()))
    }

    async fn get_user_by_credentials(&self, username_or_email: &str, user_password: &str) -> Result<Option<user::entity::User>, Error> {
      println!("repo: {:?}", username_or_email);
      println!("repo: {:?}", user_password);
      let UserByCredentialsResponse { user } = self.api.get_user_by_credentials(
          Request::new(
              UserByCredentialsRequest {
                username_or_email: String::from(username_or_email),
                user_password: String::from(user_password),
              }
          )
      ).await?.into_inner();
      Ok(user.map(|u| u.into()))
    }

    async fn update_user_login_session(&self, username_or_email: &str) -> Result<(), Error> { //Result<Option<User>, Error> {
      let EmptyResponse { } = self.api.update_user_login_session(
          Request::new(
            UpdateUserLoginSessionRequest { 
              username_or_email: String::from(username_or_email)
            }
          )
      ).await?.into_inner();
      Ok(()) //user_data.map(|u| u.into()))
    }

    
}