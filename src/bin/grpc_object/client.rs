use muscle_exercises_object::data_client::DataClient;
use muscle_exercises_object::{DataRequest, DataEmpty};

pub mod muscle_exercises_object {
    tonic::include_proto!("muscle_exercises_object");
}

fn get_id(option: &str) -> i64 {
    println!("Provide {} Id:", option);
    let mut select = String::new();
    std::io::stdin().read_line(&mut select).expect("Failed");
    let option: i64 = select.trim().parse().unwrap();
    return option;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    api::api::grpc_client_help();
    let mut select = String::new();
    std::io::stdin().read_line(&mut select).expect("Failed");
    let option= select.trim().parse().unwrap();
    println!("option: {}", option);
    let mut client = DataClient::connect(api::api::grpc_client_url()).await?;
    match option {
        1 => {
            let mut stream = client.all_muscle_groups(tonic::Request::new(DataEmpty{})).await?.into_inner();
            while let Some(object) = stream.message().await? {
                println!("RESPONSE = {:?}", object);
            }
        },
        2 => {
            let mut stream = client.all_exercises(tonic::Request::new(DataEmpty{})).await?.into_inner();
            while let Some(object) = stream.message().await? {
                println!("RESPONSE = {:?}", object);
            }
        },
        3 => {
            let mut stream = client.exercises_for_muscle_group(tonic::Request::new(DataRequest {id: get_id("Muscle Group").into()})).await?.into_inner();
            while let Some(object) = stream.message().await? {
                println!("RESPONSE = {:?}", object);
            }
        },
        4 => {
            let mut stream = client.muscle_groups_for_exercise(tonic::Request::new(DataRequest {id: get_id("Exercise").into()})).await?.into_inner();
            while let Some(object) = stream.message().await? {
                println!("RESPONSE = {:?}", object);
            }
        },
        _ => {},
    };
    Ok(())
}
