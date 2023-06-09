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

fn main(){
    println!("I am the server");
}