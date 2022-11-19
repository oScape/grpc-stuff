# Build gRPC stuff

```
cargo build
```

```
grpcurl -plaintext -import-path ./proto -proto route.proto '[::]:50051' route.Route/GetMessage
grpcurl -plaintext -import-path ./proto -proto auth.proto -d '{"lastname": "THE GOOOOAT"}' '[::]:50052' auth.Auth/CreateUser
```

# Server

gRPC server

```
cd server
cargo run
```

# Auth microservice

gRPC auth server

```
cd auth
docker-compose up -d
sqlx migrate run
cargo run
```

# Front app

Yew app

```
cd app
trunk serve
```