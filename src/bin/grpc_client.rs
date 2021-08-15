use muscle_exercises::data_client::DataClient;
use muscle_exercises::DataRequest;

pub mod muscle_exercises {
    tonic::include_proto!("muscle_exercises");
}

fn get_id(option: &str) -> i64 {
    println!("Provide {} Id:", option);
    let mut select = String::new();
    std::io::stdin().read_line(&mut select).expect("Failed");
    let option: i64 = select.trim().parse().unwrap();
    return option;
}

async fn help() -> (String, i64) {
    api::api::grpc_client_help();
    let mut select = String::new();
    std::io::stdin().read_line(&mut select).expect("Failed");
    let option= select.trim().parse().unwrap();
    println!("option: {}", option);
    let (option, id) = match option {
        1 => ("muscle_groups".to_string(), 0_i64),
        2 => ("exercises".to_string(), 0_i64),
        3 => ("exercises_for_muscle_group".to_string(), get_id("Muscle Group")),
        4 => ("muscle_groups_for_exercise".to_string(), get_id("Exercise")),
        _ => ("".to_string(), 0_i64),
    };
    return (option, id);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = DataClient::connect(api::api::grpc_client_url()).await?;
    let (option, id) = help().await;
    let request = tonic::Request::new(DataRequest {
        name: option.into(),
        id: id.into(),
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
