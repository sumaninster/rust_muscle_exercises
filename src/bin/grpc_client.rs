use muscle_exercises::data_client::DataClient;
use muscle_exercises::DataRequest;

pub mod muscle_exercises {
    tonic::include_proto!("muscle_exercises");
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = DataClient::connect(api::api::grpc_client_url()).await?;
    let request = tonic::Request::new(DataRequest {
        name: "muscle_groups".into(),
    });
    let mut stream = client
        .send_reply(request)
        .await?
        .into_inner();

    while let Some(feature) = stream.message().await? {
        println!("RESPONSE = {:?}", feature);
    }
    Ok(())
}
