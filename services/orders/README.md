### Environment variables

```shell
export DATABASE_URI=postgres://postgres:postgres@localhost:5001/orders
export USER_SERVICE_ADDR=http://0.0.0.0:50000
```

### grpcurl

```shell
grpcurl -plaintext -import-path ../../proto -proto orders.proto -d '{"userid": 1}' '[::]:50001' orders.Orders/GetUserOrders
grpcurl -plaintext -import-path ../../proto -proto orders.proto -d '{"userid": 1, "product": "Microwave", "total": 59.99}' -emit-defaults '[::]:50001' orders.Orders/PlaceOrder
```
