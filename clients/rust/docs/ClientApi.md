# \ClientApi

All URIs are relative to *http://localhost:4646/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**garbage_collect_allocation**](ClientApi.md#garbage_collect_allocation) | **get** /client/allocation/{alloc_id}/gc | forces a garbage collection of a particular, stopped allocation on a node
[**garbage_collect_allocation_0**](ClientApi.md#garbage_collect_allocation_0) | **get** /client/gc | forces a garbage collection of all stopped allocations on a node
[**get_client_allocation_stats**](ClientApi.md#get_client_allocation_stats) | **get** /client/allocation/{alloc_id}/stats | query the actual resources consumed by an allocation
[**get_client_file**](ClientApi.md#get_client_file) | **get** /client/fs/cat/{alloc_id} | reads the contents of a file in an allocation directory
[**get_client_file_at_offest**](ClientApi.md#get_client_file_at_offest) | **get** /client/fs/readat/{alloc_id} | reads the contents of a file in an allocation directory at a particular offset and limit
[**get_client_stats**](ClientApi.md#get_client_stats) | **get** /client/stats | queries the actual resources consumed on a node. The API endpoint is hosted by the Nomad client and requests have to be made to the nomad client whose resource usage metrics are of interest
[**list_client_files**](ClientApi.md#list_client_files) | **get** /client/fs/ls/{alloc_id} | lists files in an allocation directory
[**stat_client_file**](ClientApi.md#stat_client_file) | **get** /client/fs/stat/{alloc_id} | stats a file in an allocation
[**stream_client_file**](ClientApi.md#stream_client_file) | **get** /client/fs/stream/{alloc_id} | streams the contents of a file in an allocation directory
[**stream_client_logs**](ClientApi.md#stream_client_logs) | **get** /client/fs/logs/{alloc_id} | streams a task's stderr/stdout logs



## garbage_collect_allocation

> garbage_collect_allocation(alloc_id)
forces a garbage collection of a particular, stopped allocation on a node

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alloc_id** | **String** | Specifies the UUID of the allocation. This must be the full UUID, not the short 8-character one | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## garbage_collect_allocation_0

> garbage_collect_allocation_0(node_id)
forces a garbage collection of all stopped allocations on a node

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**node_id** | Option<**String**> | node id |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_client_allocation_stats

> crate::models::AllocResourceUsage get_client_allocation_stats(alloc_id)
query the actual resources consumed by an allocation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alloc_id** | **String** | Specifies the UUID of the allocation. This must be the full UUID, not the short 8-character one | [required] |

### Return type

[**crate::models::AllocResourceUsage**](AllocResourceUsage.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_client_file

> String get_client_file(alloc_id, path)
reads the contents of a file in an allocation directory

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alloc_id** | **String** | Specifies the UUID of the allocation. This must be the full UUID, not the short 8-character one | [required] |
**path** | Option<**String**> | Specifies the path of the file to read, relative to the root of the allocation directory |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_client_file_at_offest

> String get_client_file_at_offest(alloc_id, offset, limit, path)
reads the contents of a file in an allocation directory at a particular offset and limit

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alloc_id** | **String** | Specifies the UUID of the allocation. This must be the full UUID, not the short 8-character one | [required] |
**offset** | **i64** | Specifies the byte offset from where content will be read | [required] |
**limit** | **i64** | Specifies the number of bytes to read from the offset | [required] |
**path** | Option<**String**> | Specifies the path of the file to read, relative to the root of the allocation directory |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_client_stats

> Vec<crate::models::HostStats> get_client_stats(node_id)
queries the actual resources consumed on a node. The API endpoint is hosted by the Nomad client and requests have to be made to the nomad client whose resource usage metrics are of interest

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**node_id** | Option<**String**> | node id |  |

### Return type

[**Vec<crate::models::HostStats>**](HostStats.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_client_files

> Vec<crate::models::AllocFileInfo> list_client_files(alloc_id, path)
lists files in an allocation directory

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alloc_id** | **String** | Specifies the UUID of the allocation. This must be the full UUID, not the short 8-character one | [required] |
**path** | Option<**String**> | Specifies the path of the file to read, relative to the root of the allocation directory |  |

### Return type

[**Vec<crate::models::AllocFileInfo>**](AllocFileInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stat_client_file

> crate::models::AllocFileInfo stat_client_file(alloc_id, path)
stats a file in an allocation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alloc_id** | **String** | Specifies the UUID of the allocation. This must be the full UUID, not the short 8-character one | [required] |
**path** | Option<**String**> | Specifies the path of the file to read, relative to the root of the allocation directory |  |

### Return type

[**crate::models::AllocFileInfo**](AllocFileInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stream_client_file

> String stream_client_file(alloc_id, offset, path, follow, origin)
streams the contents of a file in an allocation directory

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alloc_id** | **String** | Specifies the UUID of the allocation. This must be the full UUID, not the short 8-character one | [required] |
**offset** | **i64** | Specifies the byte offset from where content will be read | [required] |
**path** | Option<**String**> | Specifies the path of the file to read, relative to the root of the allocation directory |  |
**follow** | Option<**bool**> | Specifies whether to tail the file |  |[default to true]
**origin** | Option<**String**> | Applies the relative offset to either the start or end of the file |  |[default to start]

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stream_client_logs

> String stream_client_logs(alloc_id, task, offset, follow, _type, origin, plain)
streams a task's stderr/stdout logs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alloc_id** | **String** | Specifies the UUID of the allocation. This must be the full UUID, not the short 8-character one | [required] |
**task** | **String** | Specifies the name of the task inside the allocation to stream logs from | [required] |
**offset** | **i64** | Specifies the byte offset from where content will be read | [required] |
**follow** | Option<**bool**> | Specifies whether to tail the file |  |[default to true]
**_type** | Option<**String**> | Specifies the stream to stream |  |
**origin** | Option<**String**> | Applies the relative offset to either the start or end of the file |  |[default to start]
**plain** | Option<**bool**> | is plain text format |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

