# \AgentApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**force_leave**](AgentApi.md#force_leave) | **post** /agent/force-leave | Forces a member of the gossip pool from the \"failed\" state into the \"left\" state.
[**get_health**](AgentApi.md#get_health) | **get** /agent/health | Performs a basic healthcheck
[**get_members**](AgentApi.md#get_members) | **get** /agent/members | Queries for the known peers in the gossip pool
[**get_self**](AgentApi.md#get_self) | **get** /agent/self | Queries for information about the agent we are connected to
[**get_servers**](AgentApi.md#get_servers) | **get** /agent/servers | Queries an agent in client mode for its list of known servers
[**join**](AgentApi.md#join) | **post** /agent/join | Causes the agent to join a cluster by joining the gossip pool at one of the given addresses
[**stream_logs**](AgentApi.md#stream_logs) | **get** /agent/monitor | Streams logs from a specific Nomad server node
[**update_servers**](AgentApi.md#update_servers) | **post** /agent/servers | Updates the list of known servers to the given addresses, replacing all previous addresses



## force_leave

> force_leave(node)
Forces a member of the gossip pool from the \"failed\" state into the \"left\" state.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**node** | **String** | the name of the node | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_health

> crate::models::AgentHealthResponse get_health()
Performs a basic healthcheck

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::AgentHealthResponse**](AgentHealthResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_members

> crate::models::ServerMembers get_members()
Queries for the known peers in the gossip pool

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ServerMembers**](ServerMembers.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_self

> crate::models::AgentSelf get_self()
Queries for information about the agent we are connected to

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::AgentSelf**](AgentSelf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_servers

> Vec<String> get_servers()
Queries an agent in client mode for its list of known servers

### Parameters

This endpoint does not need any parameter.

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## join

> crate::models::JoinResponse join(address)
Causes the agent to join a cluster by joining the gossip pool at one of the given addresses

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | [**Vec<String>**](String.md) | server address (ip:port) | [required] |

### Return type

[**crate::models::JoinResponse**](JoinResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stream_logs

> crate::models::StreamFrame stream_logs(log_level, node_id, server_id, json, plain)
Streams logs from a specific Nomad server node

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**log_level** | Option<**String**> | log level |  |
**node_id** | Option<**String**> | node id |  |
**server_id** | Option<**String**> | server id |  |
**json** | Option<**bool**> | is json format |  |
**plain** | Option<**bool**> | is plain text format |  |

### Return type

[**crate::models::StreamFrame**](StreamFrame.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_servers

> Vec<String> update_servers(address)
Updates the list of known servers to the given addresses, replacing all previous addresses

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | [**Vec<String>**](String.md) | server address (ip:port) | [required] |

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
