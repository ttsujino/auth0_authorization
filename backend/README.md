# Backend of muscleshare app

## Getting started

1. Make sure that the docker daemon is runnning properly
2. Build a docker image
```
make build
```
3. Start the db container
```
make db
```
4. Create the database, run the migrations, and start the server
```
make dev
```

## How to use
1. Add data to the database
```bash
curl -X POST http://localhost:3000/post/new/1 \
     -H "Content-Type: application/json" \
     -d '{"title": "New Post", "content": "The content of the new post."}'
```

2. Retrieve all data from the database
```bash
curl -X GET http://localhost:3000/posts
```