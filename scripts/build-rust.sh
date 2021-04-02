#!/bin/bash

# docker run --rm -v ${PWD}/..:/local swaggerapi/swagger-codegen-cli-v3 generate \
#     -i http://petstore.swagger.io/v2/swagger.json \
#     -l go \
#     -o /local/client/go

# docker run --rm -v ${PWD}:/local swaggerapi/swagger-codegen-cli-v3 generate \
#     -i https://raw.githubusercontent.com/swagger-api/swagger-codegen/3.0.0/modules/swagger-codegen/src/test/resources/2_0/petstore.yaml \
#     -l go \
#     -o /local/client/go

# docker run --rm -v ${PWD}:/local swaggerapi/swagger-codegen-cli-v3 generate \
#     -i /local/swagger/swagger.yaml \
#     -l go \
#     -o /local/client/go


swagger-cli bundle swagger/swagger.yaml -o dist/swagger.json

docker run --rm -v ${PWD}:/local swaggerapi/swagger-codegen-cli-v3 generate \
    -i /local/dist/swagger.json \
    -l go \
    -o /local/client/go

    # -i /local/swagger/swagger.yaml \
