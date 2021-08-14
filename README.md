# Tonal - API Coding Challenge
This API is built as a coding challenge for Tonal

## How to run the go codes from the terminal

1. Move the unzipped tonal_api folder to your $GOPATH/src
2. Navigate to the unzipped folder from your terminal.
3. Run $ go get
4. Update your database username and password in the config.go file.
5. Run go run *.go

## How to import the db.sql file in your postgres database

Run the following commands from your terminal-

1. $ createdb -U postgres tonal
2. $ psql -U postgres tonal < $GOPATH/src/tonal_api/db.sql