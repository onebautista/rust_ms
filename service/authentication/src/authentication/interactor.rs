use error::make_error;

use crate::account::entity::{Account, Role};
use crate::user::entity::{User};
use crate::account::repository::AccountRepository;
use crate::authentication::entity::{TokenPair, TokenPayload, TokenOnePair, TokenOnePayload};
use crate::token::repository::TokenRepository;
use crate::user::repository::UserRepository;

type Error = Box<dyn std::error::Error + Send + Sync>;

#[async_trait::async_trait]
pub trait AuthenticationInteractor {
    async fn sign_up(
        &self,
        email: &str,
        password: &str,
        role: Role,
    ) -> Result<TokenPair, Error>;
    async fn sign_in(&self, email: &str, password: &str) -> Result<TokenPair, Error>;
    async fn sign_out(&self, access_token: &str, refresh_token: &str) -> Result<bool, Error>;
    async fn change_password(
        &self,
        access_token: &str,
        refresh_token: &str,
        new_password: &str,
    ) -> Result<TokenPair, Error>;
    async fn refresh_token(&self, refresh_token: &str) -> Result<TokenPair, Error>;
    async fn validate_token(&self, access_token: &str) -> Result<String, Error>;

    async fn sign_in_one(&self, username_or_email: &str, user_password: &str) -> Result<TokenOnePair, Error>;

}

pub struct AuthenticationInteractorImpl {
    account_repository: Box<dyn AccountRepository + Send + Sync>,
    token_repository:   Box<dyn TokenRepository + Send + Sync>,
    user_repository:    Box<dyn UserRepository + Send + Sync>,

}

impl AuthenticationInteractorImpl {
    async fn create_tokens(&self, account: Account) -> Result<TokenPair, Error> {
        let payload = TokenPayload::new(&account.id, account.role);
        dbg!(&payload);
        let payload_str = payload.to_string()?;
        println!("{:?}", payload_str); 
        let access_token = self.token_repository.generate_access_token(&payload_str).await?;
        let refresh_token = self.token_repository.generate_refresh_token(&payload_str).await?;
        Ok(TokenPair::new(&access_token, &refresh_token))
    }

    async fn create_tokens_one(&self, user: User) -> Result<TokenOnePair, Error> {
        let payload = TokenOnePayload::new(&user.username);
        dbg!(&payload);
        let payload_str = payload.to_string()?;
        println!("{:?}", payload_str); 
        let access_token = self.token_repository.generate_access_token(&payload_str).await?;
        //let refresh_token = self.token_repository.generate_refresh_token(&payload_str).await?;
        Ok(TokenOnePair::new(&access_token))
    }

    async fn invalidate_tokens(&self, access_token: &str, refresh_token: &str) -> Result<(), Error> {
        let _ = self.token_repository.invalidate_token(access_token).await;
        let _ = self.token_repository.invalidate_token(refresh_token).await;
        Ok(())
    }

    pub fn new(
        account_repository: Box<dyn AccountRepository + Send + Sync>,
        token_repository:   Box<dyn TokenRepository + Send + Sync>,
        user_repository:    Box<dyn UserRepository + Send + Sync>,
    ) -> Box<dyn AuthenticationInteractor + Send + Sync> {
        Box::new(AuthenticationInteractorImpl { account_repository, token_repository, user_repository })
    }
}

#[async_trait::async_trait]
impl AuthenticationInteractor for AuthenticationInteractorImpl {
    async fn sign_up(&self, email: &str, password: &str, role: Role) -> Result<TokenPair, Error> {
        let id = self.account_repository.create_account(email, password, role).await?;
        if let Some(account) = self.account_repository.get_account_by_id(&id).await? {
            return self.create_tokens(account).await;
        }
        Err(make_error!("unable to sign up"))
    }

    async fn sign_in(&self, email: &str, password: &str) -> Result<TokenPair, Error> {
        if let Some(account) = self.account_repository.get_account_by_credentials(email, password).await? {
            return self.create_tokens(account).await;
        }
        Err(make_error!("unable to sign in"))
    }

    async fn sign_in_one(&self, username_or_email: &str, user_password: &str) -> Result<TokenOnePair, Error> {
        if let Some(user) = self.user_repository.get_user_by_credentials(username_or_email, user_password).await? {
            return self.create_tokens_one(user).await;
        }
        Err(make_error!("unable to sign in"))
    }

    async fn sign_out(&self, access_token: &str, refresh_token: &str) -> Result<bool, Error> {
        if let Ok(_) = self.token_repository.validate_token(access_token).await {
            let _ = self.invalidate_tokens(access_token, refresh_token).await;
            return Ok(true);
        }
        Ok(false)
    }

    async fn change_password(&self, access_token: &str, refresh_token: &str, new_password: &str) -> Result<TokenPair, Error> {
        let payload_str = self.token_repository.validate_token(access_token).await?;
        if let Ok(TokenPayload { account_id, role }) = TokenPayload::from_str(&payload_str) {
            if let Some(_) = self.account_repository.update_account(&account_id, None, Some(String::from(new_password)), None).await? {
                let _ = self.invalidate_tokens(access_token, refresh_token).await;
                let updated_payload = TokenPayload::new(&account_id, role);
                let updated_payload_str = updated_payload.to_string()?;
                let access_token = self.token_repository.generate_access_token(&updated_payload_str).await?;
                let refresh_token = self.token_repository.generate_refresh_token(&updated_payload_str).await?;
                return Ok(TokenPair::new(&access_token, &refresh_token));
            }
        }
        Err(make_error!("unable to change password"))
    }

    async fn refresh_token(&self, refresh_token: &str) -> Result<TokenPair, Error> {
        let payload = self.token_repository.validate_token(refresh_token).await?;
        let access_token = self.token_repository.generate_access_token(&payload).await?;
        let refresh_token = self.token_repository.generate_refresh_token(&payload).await?;
        Ok(TokenPair::new(&access_token, &refresh_token))
    }

    async fn validate_token(&self, access_token: &str) -> Result<String, Error> {
        self.token_repository.validate_token(access_token).await
    }
}