# Command to run the `openapi-generator-cli` command
define GEN_CLI
echo Compiling $(pwd)
docker run \
    --rm \
    --mount "type=bind,src=$(CURDIR),dst=/local" \
    --user "$(shell id -u):$(shell id -g)" \
    openapitools/openapi-generator-cli
endef

# Command to generate a client
define GEN_CLI_TEMPLATE
$(GEN_CLI) generate \
	--skip-validate-spec \
	--input-spec /local/dist/swagger.json \
	--output /local/$@
endef

targets = clients/rust
all: $(targets)

clients/rust: bundle
	$(GEN_CLI_TEMPLATE) \
		--template-dir /local/templates/rust \
		--generator-name rust \
		--additional-properties packageName=nomad-client \
		--additional-properties packageVersion=0.0.1 \
		--additional-properties "packageDescription=A Rust client for HashiCorp Nomad's API." \
		--additional-properties packageHomepage=https://github.com/rivet-gg/Nomad.OAS/tree/master/client/rust \
		--additional-properties packageRepository=https://github.com/rivet-gg/Nomad.OAS \
		--additional-properties 'packageKeywords="nomad", "hashicorp", "api", "client", "openapi", "swagger"' \
		--additional-properties 'packageCategories="api-bindings"' \
		--additional-properties 'packageAuthors="FuturistiCoder <FuturistiCoder@gmail.com>", "Nathan Flurry <developer@nathanflurry.com>"' \
		--additional-properties packageLicense=MIT \
		--additional-properties supportAsync=true

.PHONY: bundle
bundle: validate dist/swagger.json

dist/swagger.json: $(shell find swagger/ -name *.yaml)
	npx swagger-cli@4 bundle swagger/swagger.yaml -o dist/swagger.json

.PHONY: validate
validate:
	@echo Validation temporarily disabled
	# npx swagger-cli@4 validate swagger/swagger.yaml

.PHONY: clean
clean:
	rm -r clients/* dist/*

