use grpc_test::*;

use tonic::transport::Endpoint;

#[tokio::main]
async fn main() {
    let hostname = "localhost";
    let port = 6666;

    let client = {
        let uri = format!("http://{}:{}", hostname, port);
        let channel = Endpoint::from_shared(uri)
            .unwrap()
            .connect_lazy()
            .unwrap();
        TestServiceClient::new(channel)
    };

    let hs: Vec<tokio::task::JoinHandle<()>> = (0..200_000).map(|_| {
        let mut client = client.clone();
        tokio::spawn(async move {
            client.call(Req{ payload: vec![6; 1000]}).await.unwrap();
        })
    })
    .collect();

    for h in hs {
        h.await.unwrap();
    }
}
