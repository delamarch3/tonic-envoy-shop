### Generate descriptor sets for Envoy

```shell
protoc -Iproto -I. --include_imports --include_source_info --descriptor_set_out=envoy/proto/users.pb proto/users.proto
protoc -Iproto -I. --include_imports --include_source_info --descriptor_set_out=envoy/proto/orders.pb proto/orders.proto
```
