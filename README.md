# gRPC client/server for zero-knowledge proof authentication
Chaum Pederson Zero-Knowledge Proof in Rust. Chaum Pederson is a zero-knowledge proof protocol that allows a prover to prove to a verifier that they know a discrete logarithm without revealing the discrete logarithm. This is useful for authentication, where the prover can prove to the verifier that they know a secret without revealing the secret. 
</br>
In zero-knowledge proof authentication, the prover and verifier are both clients of a server. The server has a public key and a private key. The prover knows the private key and the verifier knows the public key. The prover wants to prove to the verifier that they know the private key without revealing the private key. The verifier wants to verify that the prover knows the private key without learning the private key.

## Usage
You can run the program with Docker. First build the containers:

```
$ docker-compose build zkpserver
```

Run the container:

```
$ docker-compose run --rm zkpserver
```

In the remote terminal that appears run the server:

```
root@e84736012f9a:/zkp-server# cargo run --bin server --release
```

Open a new terminal on your machine and connect to the container:

```
$ docker container ls
CONTAINER ID   IMAGE                  COMMAND   CREATED          STATUS          PORTS     NAMES
e84736012f9a   zkp-course-zkpserver   "bash"    20 minutes ago   Up 20 minutes             zkp-course_zkpserver_run_b1f3fa2cd94a

$ docker exec -it e84736012f9a /bin/bash
```

Run the client:

```
root@e84736012f9a:/zkp-server# cargo run --bin client --release
```
