use muscle_exercises::data_client::DataClient;
use muscle_exercises::DataRequest;

pub mod muscle_exercises {
    tonic::include_proto!("muscle_exercises");
}

async fn help() -> String {
    api::api::grpc_client_help();
    let mut select = String::new();
    std::io::stdin().read_line(&mut select).expect("Failed");
    let option= select.trim().parse().unwrap();
    println!("option: {}", option);
    let option = match option {
        1 => "muscle_groups".to_string(),
        2 => "exercises".to_string(),
        3 => "exercises_for_muscle_group".to_string(),
        4 => "muscle_groups_for_exercise".to_string(),
        _ => "".to_string(),
    };
    return option;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = DataClient::connect(api::api::grpc_client_url()).await?;
    let request = tonic::Request::new(DataRequest {
        name: help().await.into(),
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
