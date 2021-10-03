# \StatusApi

All URIs are relative to *http://localhost:4646/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_leader**](StatusApi.md#get_leader) | **get** /status/leader | returns the address of the current leader in the region
[**get_peers**](StatusApi.md#get_peers) | **get** /status/peers | returns the set of raft peers in the region



## get_leader

> String get_leader(namespace, region, index, wait)
returns the address of the current leader in the region

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**namespace** | Option<**String**> |  |  |
**region** | Option<**String**> | Make a request across regions to the given region |  |
**index** | Option<**i64**> | index used for blocking requests |  |
**wait** | Option<**String**> | wait time used for blocking requests |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_peers

> Vec<String> get_peers(namespace, region, index, wait)
returns the set of raft peers in the region

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**namespace** | Option<**String**> |  |  |
**region** | Option<**String**> | Make a request across regions to the given region |  |
**index** | Option<**i64**> | index used for blocking requests |  |
**wait** | Option<**String**> | wait time used for blocking requests |  |

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

