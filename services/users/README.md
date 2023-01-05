### Environment variables

```shell
export DATABASE_URI=postgres://postgres:postgres@localhost:5002/users
```

### grpcurl

```shell
grpcurl -plaintext -import-path ../../proto -proto users.proto -d '{"id": 1}' '[::]:50000' users.Users/GetUser
grpcurl -plaintext -import-path ../../proto -proto users.proto -d '{"id": 1}' '[::]:50000' users.Users/GetUserCreditLimit
```
