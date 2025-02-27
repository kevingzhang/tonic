pub mod pb {
    tonic::include_proto!("/grpc.examples.echo");
}

use pb::{EchoRequest, EchoResponse};
use std::collections::VecDeque;
use tonic::{
    transport::{Identity, Server},
    Request, Response, Status, Streaming,
};

type EchoResult<T> = Result<Response<T>, Status>;
type Stream = VecDeque<Result<EchoResponse, Status>>;

#[derive(Default)]
pub struct EchoServer;

#[tonic::async_trait]
impl pb::server::Echo for EchoServer {
    async fn unary_echo(&self, request: Request<EchoRequest>) -> EchoResult<EchoResponse> {
        let message = request.into_inner().message;
        Ok(Response::new(EchoResponse { message }))
    }

    type ServerStreamingEchoStream = Stream;

    async fn server_streaming_echo(
        &self,
        _: Request<EchoRequest>,
    ) -> EchoResult<Self::ServerStreamingEchoStream> {
        Err(Status::unimplemented("not implemented"))
    }

    async fn client_streaming_echo(
        &self,
        _: Request<Streaming<EchoRequest>>,
    ) -> EchoResult<EchoResponse> {
        Err(Status::unimplemented("not implemented"))
    }

    type BidirectionalStreamingEchoStream = Stream;

    async fn bidirectional_streaming_echo(
        &self,
        _: Request<Streaming<EchoRequest>>,
    ) -> EchoResult<Self::BidirectionalStreamingEchoStream> {
        Err(Status::unimplemented("not implemented"))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cert = tokio::fs::read("tonic-examples/data/tls/server.pem").await?;
    let key = tokio::fs::read("tonic-examples/data/tls/server.key").await?;

    let identity = Identity::from_pem(cert, key);

    let addr = "[::1]:50051".parse().unwrap();
    let server = EchoServer::default();

    Server::builder()
        .rustls_tls(identity)
        .clone()
        .serve(addr, pb::server::EchoServer::new(server))
        .await?;

    Ok(())
}
