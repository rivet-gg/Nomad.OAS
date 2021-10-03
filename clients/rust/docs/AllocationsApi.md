# \AllocationsApi

All URIs are relative to *http://localhost:4646/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_allocation**](AllocationsApi.md#get_allocation) | **get** /allocation/{alloc_id} | reads information about a specific allocation
[**get_allocations**](AllocationsApi.md#get_allocations) | **get** /allocations | query for and interact with allocations
[**restart_allocation**](AllocationsApi.md#restart_allocation) | **post** /allocation/{alloc_id}/restart | restarts an allocation or task in-place
[**signal_allocation**](AllocationsApi.md#signal_allocation) | **post** /allocation/{alloc_id}/signal | sends a signal to an allocation or task
[**stop_allocation**](AllocationsApi.md#stop_allocation) | **post** /allocation/{alloc_id}/stop | stops and reschedules a specific allocation



## get_allocation

> crate::models::Allocation get_allocation(alloc_id, namespace, region, index, wait)
reads information about a specific allocation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alloc_id** | **String** | Specifies the UUID of the allocation. This must be the full UUID, not the short 8-character one | [required] |
**namespace** | Option<**String**> |  |  |
**region** | Option<**String**> | Make a request across regions to the given region |  |
**index** | Option<**i64**> | index used for blocking requests |  |
**wait** | Option<**String**> | wait time used for blocking requests |  |

### Return type

[**crate::models::Allocation**](Allocation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_allocations

> Vec<crate::models::AllocationListStub> get_allocations(namespace, region, index, wait, prefix)
query for and interact with allocations

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**namespace** | Option<**String**> |  |  |
**region** | Option<**String**> | Make a request across regions to the given region |  |
**index** | Option<**i64**> | index used for blocking requests |  |
**wait** | Option<**String**> | wait time used for blocking requests |  |
**prefix** | Option<**String**> | Specifies a string to filter jobs on based on an index prefix. This is specified as a query string parameter |  |

### Return type

[**Vec<crate::models::AllocationListStub>**](AllocationListStub.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## restart_allocation

> restart_allocation(alloc_id, namespace, region, index, wait, allocation_restart_request)
restarts an allocation or task in-place

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alloc_id** | **String** | Specifies the UUID of the allocation. This must be the full UUID, not the short 8-character one | [required] |
**namespace** | Option<**String**> |  |  |
**region** | Option<**String**> | Make a request across regions to the given region |  |
**index** | Option<**i64**> | index used for blocking requests |  |
**wait** | Option<**String**> | wait time used for blocking requests |  |
**allocation_restart_request** | Option<[**AllocationRestartRequest**](AllocationRestartRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## signal_allocation

> signal_allocation(alloc_id, namespace, region, index, wait, alloc_signal_request)
sends a signal to an allocation or task

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alloc_id** | **String** | Specifies the UUID of the allocation. This must be the full UUID, not the short 8-character one | [required] |
**namespace** | Option<**String**> |  |  |
**region** | Option<**String**> | Make a request across regions to the given region |  |
**index** | Option<**i64**> | index used for blocking requests |  |
**wait** | Option<**String**> | wait time used for blocking requests |  |
**alloc_signal_request** | Option<[**AllocSignalRequest**](AllocSignalRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stop_allocation

> crate::models::AllocStopResponse stop_allocation(alloc_id, namespace, region, index, wait)
stops and reschedules a specific allocation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alloc_id** | **String** | Specifies the UUID of the allocation. This must be the full UUID, not the short 8-character one | [required] |
**namespace** | Option<**String**> |  |  |
**region** | Option<**String**> | Make a request across regions to the given region |  |
**index** | Option<**i64**> | index used for blocking requests |  |
**wait** | Option<**String**> | wait time used for blocking requests |  |

### Return type

[**crate::models::AllocStopResponse**](AllocStopResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

