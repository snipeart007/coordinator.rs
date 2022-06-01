use tonic::Request;

pub mod core_grpc {
    tonic::include_proto!("core_grpc");
}

use core_grpc::{core_grpc_client::CoreGrpcClient, Coordinate, JoinRequest};

async fn client() -> Result<(), Box<dyn std::error::Error>> {
    println!("Connecting to server...");
    let mut client = CoreGrpcClient::connect("http://[::1]:5555").await?;

    println!("Joining...");
    let response = client
        .join(Request::new(JoinRequest {
            id: "test".to_string(),
            position: Some(Coordinate {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }),
        }))
        .await?;

    println!("{:#?}", response);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    client().await?;

    Ok(())
}
