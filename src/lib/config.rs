pub mod config {
    pub fn db_url() -> String {
        let user = "postgres";
        let password = "<password>";
        let host = "localhost";
        let database = "muscle_exercises";
        return format!("postgresql://{}:{}@{}/{}", user, password, host, database);
    }

    pub fn http_server_url() -> String {
        return "127.0.0.1:7878".to_string();
    }

    pub fn grpc_server_url() -> String {
        return "[::1]:50051".to_string();
    }

    pub fn grpc_client_url() -> String {
        return "http://[::1]:50051".to_string();
    }

    // Function to print all URLs for HTTP requests
    pub fn api_help() {
        println!();
        println!("Rust REST API v1.0 - Running on port 7878");
        println!();
        println!("Homepage: http://localhost:7878/");
        println!("Muscle Groups: http://localhost:7878/musclegroups");
        println!("Exercises: http://localhost:7878/exercises");
        println!("Exercises for a Muscle Group with id 1: http://localhost:7878/exercisesformusclegroup/1");
        println!("Muscle Groups for an Exercise with id 1: http://localhost:7878/musclegroupsforexercise/1");
    }

    pub fn grpc_client_help() {
        println!();
        println!("grpc Client v1.0 - Connecting to [::1]:50051");
        println!();
        println!("Select option number");
        println!("1) Muscle Groups");
        println!("2) Exercises");
        println!("3) Exercises for a Muscle Group with id 1");
        println!("4) Muscle Groups for an Exercise with id 1");
        println!("5) Exit");
    }
}