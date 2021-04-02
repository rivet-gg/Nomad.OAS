#!/bin/bash

npx swagger-cli bundle swagger/swagger.yaml -o dist/swagger.json

docker run \
    --rm \
    --mount "type=bind,src=$(pwd),dst=/local" \
    --user "$(id -u):$(id -g)" \
    openapitools/openapi-generator-cli generate \
        --skip-validate-spec \
        --input-spec /local/dist/swagger.json \
        --generator-name rust \
        --output /local/client/rust \
        --additional-properties packageName=nomad-client
