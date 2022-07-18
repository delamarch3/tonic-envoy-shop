#!/bin/bash

# Update descriptor sets for envoy:
protoc -Iproto -I. --include_imports --include_source_info --descriptor_set_out=envoy/proto/users.pb proto/users.proto
protoc -Iproto -I. --include_imports --include_source_info --descriptor_set_out=envoy/proto/orders.pb proto/orders.proto

for dir in $(ls -d services/*);
do 
    cd $dir;
    cp -r ../../proto .;
    cd ../..;
done
