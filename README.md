# Rust REST API - Muscle and Exercises
This API is built to demonstrate Rust REST API

## How to run the go codes from the terminal

1. Create a project directory and navigate to the folder from your terminal.
2. git clone <project_git_url>
3. Update your database username and password in the config.rs file.
4. Run $ cargo build
5. Run $ cargo run

## How to import the db.sql file in your postgres database

Run the following commands from your terminal-

1. $ createdb -U postgres tonal
2. $ psql -U postgres tonal < <project_path>/db.sql


## URL to test

Open following url from browser-

1. Homepage: http://localhost:7878/
2. Muscle Groups: http://localhost:7878/musclegroups
3. Exercises: http://localhost:7878/exercises
4. Exercises for a Muscle Group with id 1: http://localhost:7878/exercisesformusclegroup/1
5. Muscle Groups for an Exercise with id 1: http://localhost:7878/musclegroupsforexercise/1