#!/usr/bin/env bash

set -eou pipefail

cleanup() {
	docker rm -f flipt-rust-integration
	rm -rf tmp/flipt-container
}
trap 'cleanup' SIGTERM EXIT

rm -rf tmp/flipt-container
mkdir -p tmp/flipt-container

docker run -d \
	--name flipt-rust-integration \
	-p 8080:8080 \
	-p 9000:9000 \
	-e FLIPT_AUTHENTICATION_METHODS_TOKEN_ENABLED=1 \
	-e FLIPT_AUTHENTICATION_REQUIRED=1 \
	-e FLIPT_LOG_ENCODING=json \
	-v "$(pwd)/tmp/flipt-container:/var/opt/flipt" \
	flipt/flipt:latest

# Wait for the service to wake up.
while ! nc -z localhost 8080; do   
	sleep 0.1
done

# Wait for access token to be generated and printed.
while ! docker logs flipt-rust-integration | grep -q "access token";
do
    sleep 1
done

TOKEN=$(docker logs flipt-rust-integration | jq -rR 'fromjson? | select(.M=="access token created") | .client_token')

FLIPT_AUTH_TOKEN=$TOKEN FLIPT_ENDPOINT=http://localhost:8080 cargo test --features flipt_integration --test "integration*"
