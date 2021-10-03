# \RegionsApi

All URIs are relative to *http://localhost:4646/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_regions**](RegionsApi.md#get_regions) | **get** /regions | list all known regions



## get_regions

> Vec<String> get_regions(namespace, region, index, wait)
list all known regions

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

