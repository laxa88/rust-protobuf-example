use payments::bitcoin_service_client::BitcoinServiceClient;
use payments::SendPaymentRequest;

pub mod payments {
    tonic::include_proto!("payments");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = BitcoinServiceClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(SendPaymentRequest {
        from_addr: "123456".to_owned(),
        to_addr: "654321".to_owned(),
        amount: 123,
    });

    let response = client.send_payment(request).await?;

    println!("RESPONSE = {:#?}", response);

    Ok(())
}
