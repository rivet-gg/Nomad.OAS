# \DeploymentsApi

All URIs are relative to *http://localhost:4646/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**fail_deployment**](DeploymentsApi.md#fail_deployment) | **post** /deployment/fail/{deployment_id} | mark a deployment as failed. This should be done to force the scheduler to stop creating allocations as part of the deployment or to cause a rollback to a previous job version. This endpoint only triggers a rollback if the most recent stable version of the job has a different specification than the job being reverted
[**get_allocations_for_deployment**](DeploymentsApi.md#get_allocations_for_deployment) | **get** /deployment/allocations/{deployment_id} | lists the allocations created or modified for the given deployment
[**get_deployment**](DeploymentsApi.md#get_deployment) | **get** /deployment/{deployment_id} | reads information about a specific deployment by ID
[**get_deployments**](DeploymentsApi.md#get_deployments) | **get** /deployments | lists all deployments
[**pause_deployment**](DeploymentsApi.md#pause_deployment) | **post** /deployment/pause/{deployment_id} | pause or unpause a deployment. This is done to pause a rolling upgrade or resume it
[**promote_deployment**](DeploymentsApi.md#promote_deployment) | **post** /deployment/promote/{deployment_id} | promote task groups that have canaries for a deployment. This should be done when the placed canaries are healthy and the rolling upgrade of the remaining allocations should begin
[**set_allocation_health_in_deployment**](DeploymentsApi.md#set_allocation_health_in_deployment) | **post** /deployment/allocation-health/{deployment_id} | set the health of an allocation that is in the deployment manually



## fail_deployment

> crate::models::DeploymentUpdateResponse fail_deployment(deployment_id, namespace, region, index, wait)
mark a deployment as failed. This should be done to force the scheduler to stop creating allocations as part of the deployment or to cause a rollback to a previous job version. This endpoint only triggers a rollback if the most recent stable version of the job has a different specification than the job being reverted

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**deployment_id** | **String** | Specifies the UUID of the deployment. This must be the full UUID, not the short 8-character one. This is specified as part of the path | [required] |
**namespace** | Option<**String**> |  |  |
**region** | Option<**String**> | Make a request across regions to the given region |  |
**index** | Option<**i64**> | index used for blocking requests |  |
**wait** | Option<**String**> | wait time used for blocking requests |  |

### Return type

[**crate::models::DeploymentUpdateResponse**](DeploymentUpdateResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_allocations_for_deployment

> Vec<crate::models::AllocationListStub> get_allocations_for_deployment(deployment_id, namespace, region, index, wait)
lists the allocations created or modified for the given deployment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**deployment_id** | **String** | Specifies the UUID of the deployment. This must be the full UUID, not the short 8-character one. This is specified as part of the path | [required] |
**namespace** | Option<**String**> |  |  |
**region** | Option<**String**> | Make a request across regions to the given region |  |
**index** | Option<**i64**> | index used for blocking requests |  |
**wait** | Option<**String**> | wait time used for blocking requests |  |

### Return type

[**Vec<crate::models::AllocationListStub>**](AllocationListStub.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_deployment

> crate::models::Deployment get_deployment(deployment_id, namespace, region, index, wait)
reads information about a specific deployment by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**deployment_id** | **String** | Specifies the UUID of the deployment. This must be the full UUID, not the short 8-character one. This is specified as part of the path | [required] |
**namespace** | Option<**String**> |  |  |
**region** | Option<**String**> | Make a request across regions to the given region |  |
**index** | Option<**i64**> | index used for blocking requests |  |
**wait** | Option<**String**> | wait time used for blocking requests |  |

### Return type

[**crate::models::Deployment**](Deployment.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_deployments

> Vec<crate::models::Deployment> get_deployments(namespace, region, index, wait, prefix)
lists all deployments

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**namespace** | Option<**String**> |  |  |
**region** | Option<**String**> | Make a request across regions to the given region |  |
**index** | Option<**i64**> | index used for blocking requests |  |
**wait** | Option<**String**> | wait time used for blocking requests |  |
**prefix** | Option<**String**> | Specifies a string to filter jobs on based on an index prefix. This is specified as a query string parameter |  |

### Return type

[**Vec<crate::models::Deployment>**](Deployment.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pause_deployment

> crate::models::DeploymentUpdateResponse pause_deployment(deployment_id, namespace, region, index, wait, deployment_pause_request)
pause or unpause a deployment. This is done to pause a rolling upgrade or resume it

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**deployment_id** | **String** | Specifies the UUID of the deployment. This must be the full UUID, not the short 8-character one. This is specified as part of the path | [required] |
**namespace** | Option<**String**> |  |  |
**region** | Option<**String**> | Make a request across regions to the given region |  |
**index** | Option<**i64**> | index used for blocking requests |  |
**wait** | Option<**String**> | wait time used for blocking requests |  |
**deployment_pause_request** | Option<[**DeploymentPauseRequest**](DeploymentPauseRequest.md)> |  |  |

### Return type

[**crate::models::DeploymentUpdateResponse**](DeploymentUpdateResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## promote_deployment

> crate::models::DeploymentUpdateResponse promote_deployment(deployment_id, namespace, region, index, wait, deployment_promote_request)
promote task groups that have canaries for a deployment. This should be done when the placed canaries are healthy and the rolling upgrade of the remaining allocations should begin

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**deployment_id** | **String** | Specifies the UUID of the deployment. This must be the full UUID, not the short 8-character one. This is specified as part of the path | [required] |
**namespace** | Option<**String**> |  |  |
**region** | Option<**String**> | Make a request across regions to the given region |  |
**index** | Option<**i64**> | index used for blocking requests |  |
**wait** | Option<**String**> | wait time used for blocking requests |  |
**deployment_promote_request** | Option<[**DeploymentPromoteRequest**](DeploymentPromoteRequest.md)> |  |  |

### Return type

[**crate::models::DeploymentUpdateResponse**](DeploymentUpdateResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_allocation_health_in_deployment

> crate::models::DeploymentUpdateResponse set_allocation_health_in_deployment(deployment_id, namespace, region, index, wait, deployment_alloc_health_request)
set the health of an allocation that is in the deployment manually

In some use cases, automatic detection of allocation health may not be desired. As such those task groups can be marked with an upgrade policy that uses health_check = \"manual\". Those allocations must have their health marked manually using this endpoint. Marking an allocation as healthy will allow the rolling upgrade to proceed. Marking it as failed will cause the deployment to fail. This endpoint only triggers a rollback if the most recent stable version of the job has a different specification than the job being reverted

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**deployment_id** | **String** | Specifies the UUID of the deployment. This must be the full UUID, not the short 8-character one. This is specified as part of the path | [required] |
**namespace** | Option<**String**> |  |  |
**region** | Option<**String**> | Make a request across regions to the given region |  |
**index** | Option<**i64**> | index used for blocking requests |  |
**wait** | Option<**String**> | wait time used for blocking requests |  |
**deployment_alloc_health_request** | Option<[**DeploymentAllocHealthRequest**](DeploymentAllocHealthRequest.md)> |  |  |

### Return type

[**crate::models::DeploymentUpdateResponse**](DeploymentUpdateResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

