mod psql;
mod config;

pub mod api {
    use std::io::prelude::*;
    use std::net::TcpListener;
    use std::net::TcpStream;
    use std::{fs, thread};
    use std::time::Duration;
    use threadpool::ThreadPool;
    use crate::psql::psql::Db;
    use crate::config::config;
    use crate::psql::psql_async;

    pub fn api() {
        config::api_help();
        let n_workers = 4;
        let pool = ThreadPool::new(n_workers);
        let listener = TcpListener::bind(config::http_server_url()).unwrap();
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            pool.execute(move || {
                handle_connection(stream);
            });
        }
    }

    fn handle_connection(mut stream: TcpStream) {
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        //println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
        let get = b"GET / HTTP/1.1\r\n";
        let sleep = b"GET /sleep HTTP/1.1\r\n";
        let muscle_groups = b"GET /musclegroups HTTP/1.1\r\n";
        let exercises = b"GET /exercises HTTP/1.1\r\n";
        let exercises_for_muscle_group = b"GET /exercisesformusclegroup HTTP/1.1\r\n";
        let muscle_groups_for_exercise = b"GET /musclegroupsforexercise HTTP/1.1\r\n";
        let (status_line, contents) = if buffer.starts_with(get) {
            ("HTTP/1.1 200 OK", file_contents("html/welcome.html"))
        } else if buffer.starts_with(sleep) {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", file_contents("html/welcome.html"))
        } else if buffer.starts_with(muscle_groups) {
            ("HTTP/1.1 200 OK", get_data("muscle_groups"))
        } else if buffer.starts_with(exercises) {
            ("HTTP/1.1 200 OK", get_data("exercises"))
        } else if buffer.starts_with(exercises_for_muscle_group) {
            ("HTTP/1.1 200 OK", get_data("exercises_for_muscle_group"))
        } else if buffer.starts_with(muscle_groups_for_exercise) {
            ("HTTP/1.1 200 OK", get_data("muscle_groups_for_exercise"))
        } else {
            ("HTTP/1.1 404 NOT FOUND", file_contents("html/404.html"))
        };
        let response = format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            status_line,
            contents.len(),
            contents
        );
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }

    fn file_contents(filename: &str) -> String {
        return fs::read_to_string(format!("{}", filename)).unwrap();
    }

    fn get_data(link: &str) -> String {
        let db = Db{};
        return db.get_data(link);
    }

    pub async fn get_data_async(link: &str) -> String {
        return psql_async::get_data(link).await;
    }

    pub fn grpc_server_url() -> String {
        return config::grpc_server_url();
    }

    pub fn grpc_client_url() -> String {
        return config::grpc_client_url();
    }

    pub fn grpc_client_help() {
        config::grpc_client_help();
    }
}