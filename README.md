# Build gRPC stuff

```
cargo build
```

```
grpcurl -plaintext -import-path ./proto -proto route.proto '[::]:50051' route.Route/GetMessage
grpcurl -plaintext -import-path ./proto -proto auth.proto -d '{"lastname": "THE GOOOOAT"}' '[::]:50052' auth.Auth/CreateUser
```

# Cache redis