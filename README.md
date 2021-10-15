# grpc-test

An example to reproduce the memory usage issue in `tonic`.

`tonic` takes a lot of memory under heavy load, and still occupies those memory even after the client-side stoped.

I have reproduced this issue in two different servers running Ubuntu. 

And it didn't occur on Win10 (memory < 3MB).

Maybe related: https://github.com/hyperium/tonic/issues/726

## server
First, start a grpc server.

```
cargo run --release --bin grpc-server
```

## client
Then, run the client. You may need to run it multiple times(up to 16 in my test) to see this.

In the begining, there may only have a small memory usage increase.
But after some rounds, the memory usage suddenly goes up.

```
cargo run --release --bin grpc-client
```

## check memory usage

```
top -p `pgrep -d ',' 'grpc-'`

PID     USER  PR  NI    VIRT    RES    SHR S  %CPU  %MEM     TIME+ COMMAND
_       _     20   0    1.5g   1.1g   0.0g S   0.0   3.5   1:46.44 grpc-server
```


## code
```protobuf
syntax = "proto3";

package test;


message Req {
    bytes payload = 1;
}

message Resp {
    bytes payload = 1;
}

service TestService {
    rpc call(Req) returns (Resp);
}
```


```rust
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
```

```rust
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
```

