# Rust REST API & GRPC - Muscle and Exercises
This API is built to demonstrate Rust REST API

## Project Setup

1. Create a project directory and navigate to the folder from your terminal.
2. git clone https://github.com/sumaninster/rust_muscle_exercises.git
3. Update your database username, password and database name in the config.rs file.

## How to import the db.sql file in your postgres database

Run the following commands from your terminal-

1. $ createdb -U postgres muscle_exercises
2. $ psql -U postgres muscle_exercises < <project_path>/db/db.sql

## How to run REST API from the terminal

1. Run $ cargo build
2. Run $ cargo run --bin http-server

## REST API: URL to test

Open following url from browser-

1. Homepage: http://localhost:7878/
2. Muscle Groups: http://localhost:7878/musclegroups
3. Exercises: http://localhost:7878/exercises
4. Exercises for a Muscle Group with id 1: http://localhost:7878/exercisesformusclegroup/1
5. Muscle Groups for an Exercise with id 1: http://localhost:7878/musclegroupsforexercise/1

## How to run grpc Server & Client from the terminal

1. Run $ cargo build

Run the below commands in two different terminal

2. Run $ cargo run --bin grpc-server
3. Run $ cargo run --bin grpc-client
4. Select an option from client terminal to receive data from grpc server

## How to run grpc Server & Client (json) from the terminal

1. Run $ cargo build

Run the below commands in two different terminal

2. Run $ cargo run --bin grpc-json-server
3. Run $ cargo run --bin grpc-json-client
4. Select an option from client terminal to receive data from grpc server