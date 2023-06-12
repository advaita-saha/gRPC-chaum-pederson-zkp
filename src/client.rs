use chaum_pederson_zkp::{exponentiate, random_number, random_string, solve, G, H, P, Q};
use tonic::{transport::Server, Code, Request, Response, Status};

pub mod zkp_auth {
    include!("./zkp_auth.rs");
}

use zkp_auth::auth_client::AuthClient;
use zkp_auth::{
    AuthenticationAnswerRequest, AuthenticationAnswerResponse, AuthenticationChallengeRequest,
    AuthenticationChallengeResponse, RegisterRequest, RegisterResponse,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = AuthClient::connect("http://127.0.0.1:50051").await?;

    let user_id = "advaita".to_string();
    let x = 6u32;
    let y1 = exponentiate(G, x, P);
    let y2 = exponentiate(H, x, P);

    let register_request = tonic::Request::new(RegisterRequest {
        user: user_id.clone(),
        y1,
        y2,
    });

    let register_response = client.register(register_request).await?;
    println!("RESPONSE={:?}", register_response);

    let k = random_number() % 10; // TODO : improve to high precision
    let r1 = exponentiate(G, k, P);
    let r2 = exponentiate(H, k, P);
    let auth_challenge_request = tonic::Request::new(AuthenticationChallengeRequest {
        user: user_id.clone(),
        r1,
        r2,
    });

    let auth_challenge_response = client
        .authentication_challenge(auth_challenge_request)
        .await?;

    println!("RESPONSE={:?}", auth_challenge_response);

    let auth_challenge_response = auth_challenge_response.into_inner();
    let c = auth_challenge_response.c;
    let auth_id = auth_challenge_response.auth_id;

    let s = solve(x, k, c, Q);

    let auth_answer_request = tonic::Request::new(AuthenticationAnswerRequest {
        auth_id,
        s,
    });

    let auth_answer_response = client.verify_authentication(auth_answer_request).await?;
    println!("RESPONSE={:?}", auth_answer_response);


    Ok(())
}
