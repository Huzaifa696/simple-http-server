# simple-http-server
simplest http server for a note taking application

## Build
```cargo build --release```

## Test
To test the CRUD operations run the following command:

```cargo test -- --test-threads=1```

## Manual Testing

Run the following command to start the server:

```cargo run --release```

And in a separate terminal send the http requests using curl.

### Create a note

```curl --header "Content-Type: application/json"   --request POST   --data '{"title":"note","description":"xyz"}'   http://127.0.0.1:8080/create```

This will create a note in the notes directory.

### Update a note

```curl --header "Content-Type: application/json"   --request POST   --data '{"title":"note","description":"some_updates"}'   http://127.0.0.1:8080/update```

This will update the note by appending updates from a newline.

### Read a note

```curl --header "Content-Type: application/json"   --request GET   --data '{"title":"note"}'   http://127.0.0.1:8080/read```

This will read the note and returns its description.

### Read a note

```curl --header "Content-Type: application/json"   --request DELETE   --data '{"title":"note"}'   http://127.0.0.1:8080/delete```

This will delete the note.