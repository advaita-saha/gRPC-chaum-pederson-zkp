use chaum_pederson_zkp::{deserialize, exponentiate, random_number, serialize, solve, G, H, P, Q};
use num_bigint::BigUint;
use std::io::stdin;
// use tonic::{transport::Server, Code, Request, Response, Status};

pub mod zkp_auth {
    include!("./zkp_auth.rs");
}

use zkp_auth::auth_client::AuthClient;
use zkp_auth::{AuthenticationAnswerRequest, AuthenticationChallengeRequest, RegisterRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = AuthClient::connect("http://127.0.0.1:50051").await?;

    let mut buffer = String::new();

    println!("Enter user id : ");
    stdin()
        .read_line(&mut buffer)
        .expect("Expected an input for user id");

    let user_id = buffer.trim().to_string();

    println!("Enter the password x (should be big number) :");
    buffer = String::new();
    stdin()
        .read_line(&mut buffer)
        .expect("Expected an input for password x");

    let x = buffer
        .trim()
        .parse::<BigUint>()
        .expect("Expected a valid number");
    println!("x = {:?}", x);
    let y1 = exponentiate(&deserialize(G), &x, &deserialize(P));
    let y2 = exponentiate(&deserialize(H), &x, &deserialize(P));

    let register_request = tonic::Request::new(RegisterRequest {
        user: user_id.clone(),
        y1: serialize(&y1),
        y2: serialize(&y2),
    });

    let _register_response = client.register(register_request).await?;

    let k = random_number(); // TODO : improve to high precision
    let r1 = exponentiate(&deserialize(G), &k, &deserialize(P));
    let r2 = exponentiate(&deserialize(H), &k, &deserialize(P));
    let auth_challenge_request = tonic::Request::new(AuthenticationChallengeRequest {
        user: user_id.clone(),
        r1: serialize(&r1),
        r2: serialize(&r2),
    });

    let auth_challenge_response = client
        .authentication_challenge(auth_challenge_request)
        .await?;

    let auth_challenge_response = auth_challenge_response.into_inner();
    let c = deserialize(&auth_challenge_response.c);
    let auth_id = auth_challenge_response.auth_id;

    let s = solve(&x, &k, &c, &deserialize(Q));

    let auth_answer_request = tonic::Request::new(AuthenticationAnswerRequest {
        auth_id,
        s: serialize(&s),
    });

    let auth_answer_response = client.verify_authentication(auth_answer_request).await?;
    println!(
        "SessionID={:?}",
        auth_answer_response.into_inner().session_id
    );

    Ok(())
}
