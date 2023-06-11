use tonic::{transport::Server, Request, Response, Status};



pub mod zkp_auth {
    include!("./zkp_auth.rs");
}

use zkp_auth::auth_server::{Auth, AuthServer};
use zkp_auth::{
    RegisterRequest,
    RegisterResponse,
    AuthenticationChallengeRequest,
    AuthenticationChallengeResponse,
    AuthenticationAnswerRequest,
    AuthenticationAnswerResponse
};

#[derive(Debug, Default)]
pub struct AuthImpl {}

#[tonic::async_trait]
impl Auth for AuthImpl {
    async fn register(
        &self,
        request: Request<RegisterRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<RegisterResponse>, Status> { // Return an instance of type HelloReply
        println!("Got a request: {:?}", request);

        // let reply = hello_world::HelloReply {
        //     message: format!("Hello {}!", request.into_inner().name).into(), // We must use .into_inner() as the fields of gRPC requests and responses are private
        // };

        // Ok(Response::new(reply)) // Send back our formatted greeting
        todo!()
    }

    async fn authentication_challenge(
        &self,
        request: Request<AuthenticationChallengeRequest>,
    ) -> Result<Response<AuthenticationChallengeResponse>,Status> {
        println!("Got a request: {:?}", request);
        todo!()
    }

    async fn verify_authentication(
        &self,
        request: Request<AuthenticationAnswerRequest>,
    ) -> Result<Response<AuthenticationAnswerResponse>,Status> {
        println!("Got a request: {:?}", request);
        todo!()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    println!("Running server...");
    let addr = "127.0.0.1:50051".parse()?;
    let auth_impl = AuthImpl::default();

    Server::builder()
        .add_service(AuthServer::new(auth_impl))
        .serve(addr)
        .await?;

    Ok(())
}