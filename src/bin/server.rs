use tonic::transport::Server;

use coordinator::core::rpc::{core_grpc::core_grpc_server::CoreGrpcServer, CoreGrpcService};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:5555".parse().unwrap();

    let route_guide = CoreGrpcService {};

    let svc = CoreGrpcServer::new(route_guide);

    println!("Running on http://{}", addr);
    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}
