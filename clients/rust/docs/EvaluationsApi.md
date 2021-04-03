# \EvaluationsApi

All URIs are relative to *http://localhost:4646/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_allocations_for_evaluation**](EvaluationsApi.md#get_allocations_for_evaluation) | **get** /evaluation/{eval_id}/allocations | lists the allocations created or modified for the given evaluation
[**get_evaluation**](EvaluationsApi.md#get_evaluation) | **get** /evaluation/{eval_id} | reads information about a specific evaluation by ID
[**get_evaluations**](EvaluationsApi.md#get_evaluations) | **get** /evaluations | lists all evaluations



## get_allocations_for_evaluation

> Vec<crate::models::AllocationListStub> get_allocations_for_evaluation(eval_id)
lists the allocations created or modified for the given evaluation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**eval_id** | **String** | Specifies the UUID of the evaluation. This must be the full UUID, not the short 8-character one. This is specified as part of the path | [required] |

### Return type

[**Vec<crate::models::AllocationListStub>**](AllocationListStub.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_evaluation

> crate::models::Evaluation get_evaluation(eval_id)
reads information about a specific evaluation by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**eval_id** | **String** | Specifies the UUID of the evaluation. This must be the full UUID, not the short 8-character one. This is specified as part of the path | [required] |

### Return type

[**crate::models::Evaluation**](Evaluation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_evaluations

> Vec<crate::models::Evaluation> get_evaluations(prefix)
lists all evaluations

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**prefix** | Option<**String**> | Specifies a string to filter jobs on based on an index prefix. This is specified as a query string parameter |  |

### Return type

[**Vec<crate::models::Evaluation>**](Evaluation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

