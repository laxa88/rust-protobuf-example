use tonic::{transport::Server, Request, Response, Status};

use payments::bitcoin_service_server::{BitcoinService, BitcoinServiceServer};
use payments::{SendPaymentRequest, SendPaymentResponse};

pub mod payments {
    tonic::include_proto!("payments"); // The string specified here must match the proto package name
}

#[derive(Debug, Default)]
pub struct MyBitcoinService {}

#[tonic::async_trait]
impl BitcoinService for MyBitcoinService {
    async fn send_payment(
        &self,
        request: Request<SendPaymentRequest>,
    ) -> Result<Response<SendPaymentResponse>, Status> {
        println!("Got a request: {:#?}", request);

        let req = request.into_inner();

        let reply = SendPaymentResponse {
            successful: true,
            message: format!("Sent {}BTC to {}.", req.amount, req.to_addr).into(),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let btc_service = MyBitcoinService::default();

    Server::builder()
        .add_service(BitcoinServiceServer::new(btc_service))
        .serve(addr)
        .await?;

    Ok(())
}
