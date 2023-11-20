use crate::user::entity::{User, LoginDTO};
use crate::user::repository::UserRepository;
use crate::password::hasher::Hasher;

type Error = Box<dyn std::error::Error + Send + Sync>;
use error::make_error;

#[tonic::async_trait]
 pub trait UserInteractor {
  async fn get_user_by_id(&self, i: i32) -> Result<User, Error>;
  async fn get_user_by_credentials(&self, username_or_email: &str, password: &str) -> Result<User, Error>;
  async fn get_user_by_username(&self, username_or_email: &str) -> Result<User, Error>;
  async fn update_login_session_to_db(&self, username_or_email: &str, ) -> Result<(), Error>;
}

pub struct UserInteractorImpl {
  password_hasher: Box<dyn Hasher + Send + Sync>,
  repository: Box<dyn UserRepository + Send + Sync>,
}

impl UserInteractorImpl {
  pub fn new(password_hasher: Box<dyn Hasher + Send + Sync>, repository: Box<dyn UserRepository + Send + Sync>) -> Box<dyn UserInteractor + Send + Sync> {
    Box::new(UserInteractorImpl { password_hasher, repository })
  }
}

#[tonic::async_trait]
impl UserInteractor for UserInteractorImpl {
    async fn get_user_by_id(&self, i: i32)-> Result<User, Error> {
        self.repository.get_user_by_id(i).await
    }

    async fn get_user_by_credentials(&self, username_or_email: &str, password: &str)-> Result<User, Error> {
        let user_data = self.repository.get_user_by_username_or_email(username_or_email).await?;
        
        if let Ok(_) = self.password_hasher.verify_password(password, &user_data.password) {
          Ok(user_data) 
        } else {
          Err(make_error!("invalid password"))
        }
    }

    async fn get_user_by_username(&self, username_or_email: &str)-> Result<User, Error> {
        self.repository.get_user_by_username_or_email(username_or_email).await
    }

    async fn update_login_session_to_db(&self, username_or_email: &str) -> Result<(), Error> {
        let user_login_session = self.repository.generate_login_session().await?;
        println!("user login sesion: {:?}", user_login_session);
        //let user_data = self.repository.get_user_by_username_or_email(un).await?;
        //if let Ok(user) = repository.get_user_by_username_or_email(un).await? {
            self.repository.update_login_session_to_db(username_or_email, &user_login_session).await?;
            Ok(())
        //}
        


    }

    

    
}