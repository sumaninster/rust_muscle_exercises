use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status, transport::Server};
use muscle_exercises_object::{DataRequest, DataEmpty, MuscleGroups, Exercises};
use muscle_exercises_object::data_server::{Data, DataServer};
use crate::psql_object_async::psql_object_async::{get_muscle_groups, get_exercises, get_exercises_for_muscle_group, get_muscle_groups_for_exercise};

mod psql_object_async;

pub mod muscle_exercises_object {
    tonic::include_proto!("muscle_exercises_object");
}

#[derive(Default)]
pub struct MyGrpc {}

#[tonic::async_trait]
impl Data for MyGrpc {
    type AllMuscleGroupsStream = ReceiverStream<Result<MuscleGroups, Status>>;
    type AllExercisesStream = ReceiverStream<Result<Exercises, Status>>;
    type ExercisesForMuscleGroupStream = ReceiverStream<Result<Exercises, Status>>;
    type MuscleGroupsForExerciseStream = ReceiverStream<Result<MuscleGroups, Status>>;

    async fn all_muscle_groups (
        &self,
        request: Request<DataEmpty>,
    ) -> Result<Response<Self::AllMuscleGroupsStream>, Status> {
        println!("Got a request from {:?} : {:?}", request.remote_addr(), request);
        let (tx, rx) = mpsc::channel(4);
        tokio::spawn(async move {
            let data = MuscleGroups {
                muscle_groups: get_muscle_groups().await.unwrap(),
            };
            tx.send(Ok(data)).await.unwrap();
        });
        return Ok(Response::new(ReceiverStream::new(rx)));
    }

    async fn all_exercises (
        &self,
        request: Request<DataEmpty>,
    ) -> Result<Response<Self::AllExercisesStream>, Status> {
        println!("Got a request from {:?} : {:?}", request.remote_addr(), request);
        let (tx, rx) = mpsc::channel(4);
        tokio::spawn(async move {
            let data = Exercises {
                exercises: get_exercises().await.unwrap(),
            };
            tx.send(Ok(data)).await.unwrap();
        });
        return Ok(Response::new(ReceiverStream::new(rx)));
    }

    async fn exercises_for_muscle_group (
        &self,
        request: Request<DataRequest>,
    ) -> Result<Response<Self::ExercisesForMuscleGroupStream>, Status> {
        println!("Got a request from {:?} : {:?}", request.remote_addr(), request);
        let (tx, rx) = mpsc::channel(4);
        tokio::spawn(async move {
            let DataRequest { id } = request.into_inner();
            let data = Exercises {
                exercises: get_exercises_for_muscle_group(&id).await.unwrap(),
            };
            tx.send(Ok(data)).await.unwrap();
        });
        return Ok(Response::new(ReceiverStream::new(rx)));
    }

    async fn muscle_groups_for_exercise (
        &self,
        request: Request<DataRequest>,
    ) -> Result<Response<Self::MuscleGroupsForExerciseStream>, Status> {
        println!("Got a request from {:?} : {:?}", request.remote_addr(), request);
        let (tx, rx) = mpsc::channel(4);
        tokio::spawn(async move {
            let DataRequest { id } = request.into_inner();
            let data = MuscleGroups {
                muscle_groups: get_muscle_groups_for_exercise(&id).await.unwrap(),
            };
            tx.send(Ok(data)).await.unwrap();
        });
        return Ok(Response::new(ReceiverStream::new(rx)));
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address = api::api::grpc_server_url().parse().unwrap();
    let my_grpc = MyGrpc::default();
    println!("DataServer listening on {}", address);
    Server::builder()
        .add_service(DataServer::new(my_grpc))
        .serve(address)
        .await?;
    Ok(())
}
