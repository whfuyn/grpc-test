use grpc_test::*;

use tonic::Status;
use tonic::Request;
use tonic::Response;


struct Service;

#[tonic::async_trait]
impl TestService for Service {
    async fn call(&self, _req: Request<Req>) -> Result<Response<Resp>, Status> {
        Ok(Response::new(Resp{ payload: vec![] }))
    }
}

#[tokio::main]
async fn main() {
    let hostname = "0.0.0.0";
    let port = 6666;

    let addr = format!("{}:{}", hostname, port).parse().unwrap();
    tonic::transport::Server::builder()
        .add_service(TestServiceServer::new(Service))
        .serve(addr)
        .await
        .unwrap();
}
