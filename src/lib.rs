mod test{
    tonic::include_proto!("test");
}

pub use test::Req;
pub use test::Resp;

pub use test::test_service_client::TestServiceClient;

pub use test::test_service_server::TestService;
pub use test::test_service_server::TestServiceServer;
