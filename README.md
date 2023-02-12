# Nomad.OAS
HashiCorp Nomad v0.11 OpenAPI specification.

## 🚨 Deprecated 🚨

HashiCorp published their official OpenAPI spec with generated Rust clients available [here](https://github.com/hashicorp/nomad-openapi/tree/main/clients/rust).

## Client libraries (generated)

### Rust

TODO: Link to package

### Adding your own client
In `Makefile`: Add your client to `target`. Then add a target for your client in the `clients` folder that calls `GEN_CLI_TEMPLATE`.

## Endpoints

| Api               | Implement | Comment        |
|-------------------|-----------|----------------|
| ACL Policies      |           |                |
| ACL Tokens        |           |                |
| Agent             | ✓         |                |
| Allocations       | ✓         |                |
| Client            | ✓         |                |
| Deployments       | ✓         |                |
| Evaluations       | ✓         |                |
| Jobs              | ✓         |                |
| Namespaces        |           | enterprise api |
| Nodes             | ✓         |                |
| Metrics           |           |                |
| Operator          |           |                |
| Plugins           |           |                |
| Quotas            |           |                |
| Regions           | ✓         |                |
| Scaling Policies  |           |                |
| Search            | ✓         |                |
| Sentinel Policies |           |                |
| Status            | ✓         |                |
| System            | ✓         |                |
| Validate          | ✓         |                |
| Volumes           | ✓         |                |
