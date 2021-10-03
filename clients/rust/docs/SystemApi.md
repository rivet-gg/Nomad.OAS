# \SystemApi

All URIs are relative to *http://localhost:4646/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**force_gc**](SystemApi.md#force_gc) | **put** /system/gc | initializes a garbage collection of jobs, evaluations, allocations, and nodes. This is an asynchronous operation
[**reconcile_summary**](SystemApi.md#reconcile_summary) | **put** /system/reconcile/summaries | reconciles the summaries of all registered jobs



## force_gc

> force_gc(namespace, region, index, wait)
initializes a garbage collection of jobs, evaluations, allocations, and nodes. This is an asynchronous operation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**namespace** | Option<**String**> |  |  |
**region** | Option<**String**> | Make a request across regions to the given region |  |
**index** | Option<**i64**> | index used for blocking requests |  |
**wait** | Option<**String**> | wait time used for blocking requests |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reconcile_summary

> reconcile_summary(namespace, region, index, wait)
reconciles the summaries of all registered jobs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**namespace** | Option<**String**> |  |  |
**region** | Option<**String**> | Make a request across regions to the given region |  |
**index** | Option<**i64**> | index used for blocking requests |  |
**wait** | Option<**String**> | wait time used for blocking requests |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

