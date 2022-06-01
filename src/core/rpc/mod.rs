use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

pub mod core_grpc {
    tonic::include_proto!("core_grpc");
}

use core_grpc::core_grpc_server::CoreGrpc;
use core_grpc::{join_response, JoinRequest, JoinResponse, SubscribeRequest, SubscribeResponse};

#[derive(Debug)]
pub struct CoreGrpcService;

#[tonic::async_trait]
impl CoreGrpc for CoreGrpcService {
    async fn join(&self, request: Request<JoinRequest>) -> Result<Response<JoinResponse>, Status> {
        println!("Join request: {:?}", request);
        Ok(Response::new(JoinResponse {
            result: join_response::Result::Success as i32,
            position: request.get_ref().position.clone(),
        }))
    }

    type SubscribeStream = ReceiverStream<Result<SubscribeResponse, Status>>;

    async fn subscribe(
        &self,
        request: Request<SubscribeRequest>,
    ) -> Result<Response<Self::SubscribeStream>, Status> {
        let (tx, rx) = mpsc::channel(4);

        println!("Subscribe request: {:?}", request);

        tokio::spawn(async move {
            for _ in 0..100 {
                tx.send(Ok(SubscribeResponse {
                    ..Default::default()
                }))
                .await
                .unwrap();
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}
