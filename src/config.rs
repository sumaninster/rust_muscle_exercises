pub mod config {
    pub fn db_url() -> String {
        return "postgresql://postgres:suman123@localhost/muscle_exercises".to_string();
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
    pub fn help() {
        println!();
        println!("Rust REST API v1.0 - Running on port 7878");
        println!();
        println!("Homepage: http://localhost:7878/");
        println!("Muscle Groups: http://localhost:7878/musclegroups");
        println!("Exercises: http://localhost:7878/exercises");
        println!();
        println!("Exercises for a Muscle Group with id 1: http://localhost:7878/exercisesformusclegroup/1");
        println!("Muscle Groups for an Exercise with id 1: http://localhost:7878/musclegroupsforexercise/1");
    }
}