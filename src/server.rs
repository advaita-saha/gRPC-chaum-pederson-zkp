use std::collections::HashMap;
use std::sync::Mutex;
use tonic::{transport::Server, Request, Response, Status};

pub mod zkp_auth {
    include!("./zkp_auth.rs");
}

use zkp_auth::auth_server::{Auth, AuthServer};
use zkp_auth::{
    AuthenticationAnswerRequest, AuthenticationAnswerResponse, AuthenticationChallengeRequest,
    AuthenticationChallengeResponse, RegisterRequest, RegisterResponse,
};

#[derive(Debug, Default)]
pub struct UserInfo {
    pub y1: u32,
    pub y2: u32,
    pub r1: u32,
    pub r2: u32,
    pub c: u32,
    pub session_id: String,
}

#[derive(Debug, Default)]
pub struct AuthImpl {
    pub user_info: Mutex<HashMap<String, UserInfo>>,
    pub auth_info: Mutex<HashMap<String, String>>,
}

#[tonic::async_trait]
impl Auth for AuthImpl {
    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<RegisterResponse>, Status> {
        println!("Got a register request: ");

        let request = request.into_inner();
        let user_id = request.user;

        let user_info = UserInfo {
            y1: request.y1,
            y2: request.y2,
            r1: 0,
            r2: 0,
            c: 0,
            session_id: String::new(),
        };

        let user_info_hashmap = &mut self.user_info.lock().unwrap(); // TODO : improve
        user_info_hashmap.insert(user_id, user_info);

        println!("{:?}", user_info_hashmap);

        Ok(Response::new(RegisterResponse {}))
    }

    async fn authentication_challenge(
        &self,
        request: Request<AuthenticationChallengeRequest>,
    ) -> Result<Response<AuthenticationChallengeResponse>, Status> {
        println!("Got an Authentication Challenge request: ");

        let request = request.into_inner();
        let user_id = request.user;

        let user_info_hashmap = &mut self.user_info.lock().unwrap(); // TODO : improve
        if let Some(user_info) = user_info_hashmap.get(&user_id) {

            let auth_id = String::new();    // TODO : generate auth_id
            let c = 0u32;                      // TODO : generate c

            let user_info = UserInfo {
                y1: user_info.y1,
                y2: user_info.y2,
                r1: request.r1,
                r2: request.r2,
                c,
                session_id: String::new(),
            };

            user_info_hashmap.insert(user_id, user_info);

            Ok(Response::new(AuthenticationChallengeResponse {
                auth_id,
                c,
            }))
        } else {
            todo!()
        }

        

       
    }

    async fn verify_authentication(
        &self,
        request: Request<AuthenticationAnswerRequest>,
    ) -> Result<Response<AuthenticationAnswerResponse>, Status> {
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
