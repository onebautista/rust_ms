use tonic::{Request, Response, Status};
use tonic::transport::Channel;

use crate::authentication::pb::{
    ChangePasswordRequest, ChangePasswordResponse, RefreshTokenRequest, RefreshTokenResponse, 
    SignInRequest, SignInResponse, SignInOneRequest, SignInOneResponse,
    SignOutRequest, SignOutResponse, SignUpRequest, SignUpResponse, 
    ValidateTokenRequest, ValidateTokenResponse};
use crate::authentication::pb::authentication_service_client::AuthenticationServiceClient;
use crate::authentication::pb::authentication_service_server::AuthenticationService;

pub struct AuthenticationServiceImpl {  //represent the implementation of your grpc service
    client: AuthenticationServiceClient<Channel>,  //is the client used to make gRPC calls to another service. 
}

/*
creates a new instance of AuthenticationServiceImpl and returns it as an implementation of the AuthenticationService trait. 
 It takes the AuthenticationServiceClient<Channel> as a parameter, 
 typically set up to connect to the actual gRPC service you want to interact with.
*/
impl AuthenticationServiceImpl {
    pub fn new(client: AuthenticationServiceClient<Channel>) -> impl AuthenticationService {
        AuthenticationServiceImpl { client }
    }
}

#[tonic::async_trait]
impl AuthenticationService for AuthenticationServiceImpl {
    async fn sign_up(&self, request: Request<SignUpRequest>) -> Result<Response<SignUpResponse>, Status> {
        self.client.clone().sign_up(request).await
    }

    async fn sign_in(&self, request: Request<SignInRequest>) -> Result<Response<SignInResponse>, Status> {
        println!("Got a request: {:?}", request);
        self.client.clone().sign_in(request).await
    }

    async fn sign_in_one(&self, request: Request<SignInOneRequest>) -> Result<Response<SignInOneResponse>, Status> {
        println!("request gateway: {:?}", request);
        self.client.clone().sign_in_one(request).await
    }

    async fn sign_out(&self, request: Request<SignOutRequest>) -> Result<Response<SignOutResponse>, Status> {
        self.client.clone().sign_out(request).await
    }

    async fn change_password(&self, request: Request<ChangePasswordRequest>) -> Result<Response<ChangePasswordResponse>, Status> {
        self.client.clone().change_password(request).await
    }

    async fn refresh_token(&self, request: Request<RefreshTokenRequest>) -> Result<Response<RefreshTokenResponse>, Status> {
        self.client.clone().refresh_token(request).await
    }

    async fn validate_token(&self, request: Request<ValidateTokenRequest>) -> Result<Response<ValidateTokenResponse>, Status> {
        self.client.clone().validate_token(request).await
    }
}