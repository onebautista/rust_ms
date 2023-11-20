use serde::{Deserialize, Serialize};

use crate::account::entity::Role;

type Error = Box<dyn std::error::Error + Send + Sync>;

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenPayload {
    pub account_id: String, 
    pub role: Role,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct TokenOnePayload {
    pub user_id: String, 
} 

impl TokenOnePayload {
    pub fn new(id: &str) -> Self {
        Self { user_id: String::from(id) }
    }

    pub fn from_str(s: &str) -> Result<TokenOnePayload, Error> {
        serde_json::from_str(s).map_err(|e| Box::new(e) as Error)
    }

    pub fn to_string(&self) -> Result<String, Error> {
        serde_json::to_string(self).map_err(|e| Box::new(e) as Error)
    }
}



impl TokenPayload {
    pub fn new(id: &str, role: Role) -> Self {
        Self { account_id: String::from(id), role }
    }

    pub fn from_str(s: &str) -> Result<TokenPayload, Error> {
        serde_json::from_str(s).map_err(|e| Box::new(e) as Error)
    }

    pub fn to_string(&self) -> Result<String, Error> {
        serde_json::to_string(self).map_err(|e| Box::new(e) as Error)
    }
}

pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
}

pub struct TokenOnePair {
    pub access_token: String,
}
impl TokenOnePair {
    pub fn new(access_token: &str) -> Self {
        Self {
            access_token: String::from(access_token)
        }        
    }
}



impl TokenPair {
    pub fn new(access_token: &str, refresh_token: &str) -> Self {
        Self {
            access_token: String::from(access_token),
            refresh_token: String::from(refresh_token),
        }
    }
}