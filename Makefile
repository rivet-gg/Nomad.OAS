# Command to run the `openapi-generator-cli` command
define GEN_CLI
echo Compiling $(pwd)
docker run \
    --rm \
    --mount "type=bind,src=$(CURDIR),dst=/local" \
    --user "$(shell id -u):$(shell id -g)" \
    openapitools/openapi-generator-cli:v5.1.1
endef

# Command to generate a client
define GEN_CLI_TEMPLATE
$(GEN_CLI) generate \
	--skip-validate-spec \
	--input-spec /local/dist/swagger.json \
	--output /local/$@
endef

SHELL=/bin/sh

targets = clients/rust
all: $(targets)

clients/rust: bundle
	$(GEN_CLI_TEMPLATE) --config /local/configs/rust.yaml

.PHONY: clients/rust/check
clients/rust/check: clients/rust
	cd clients/rust && cargo check

.PHONY: clients/rust/publish
clients/rust/publish: clients/rust/check
	cd clients/rust && cargo publish

.PHONY: bundle
bundle: validate dist/swagger.json

dist/swagger.json: $(shell find swagger/ -name '*.yaml')
	npx swagger-cli@4 bundle swagger/swagger.yaml -o dist/swagger.json

.PHONY: validate
validate:
	@echo Validation temporarily disabled
	# npx swagger-cli@4 validate swagger/swagger.yaml

.PHONY: clean
clean:
	rm -r clients/* dist/*

