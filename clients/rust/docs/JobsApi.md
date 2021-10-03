# \JobsApi

All URIs are relative to *http://localhost:4646/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**dispatch_job**](JobsApi.md#dispatch_job) | **post** /job/{job_id}/dispatch | dispatches a new instance of a parameterized job
[**evaluate_job**](JobsApi.md#evaluate_job) | **post** /job/{job_id}/evaluate | creates a new evaluation for the given job. This can be used to force run the scheduling logic if necessary
[**force_new_periodic_instance**](JobsApi.md#force_new_periodic_instance) | **post** /job/{job_id}/periodic/force | forces a new instance of the periodic job. A new instance will be created even if it violates the job's prohibit_overlap settings. As such, this should be only used to immediately run a periodic job
[**get_job**](JobsApi.md#get_job) | **get** /job/{job_id} | reads information about a single job for its specification and status
[**get_job_allocations**](JobsApi.md#get_job_allocations) | **get** /job/{job_id}/allocations | reads information about a single job's allocations
[**get_job_deployments**](JobsApi.md#get_job_deployments) | **get** /job/{job_id}/deployments | lists a single job's deployments
[**get_job_evaluations**](JobsApi.md#get_job_evaluations) | **get** /job/{job_id}/evaluations | reads information about a single job's evaluations
[**get_job_latest_deployment**](JobsApi.md#get_job_latest_deployment) | **get** /job/{job_id}/deployment | get a single job's most recent deployment
[**get_job_scale_status**](JobsApi.md#get_job_scale_status) | **get** /job/{job_id}/scale | reads scale information about a job
[**get_job_summary**](JobsApi.md#get_job_summary) | **get** /job/{job_id}/summary | reads summary information about a job
[**get_job_versions**](JobsApi.md#get_job_versions) | **get** /job/{job_id}/versions | reads information about all versions of a job
[**get_jobs**](JobsApi.md#get_jobs) | **get** /jobs | lists all known jobs in the system registered with Nomad
[**parse_job_hcl**](JobsApi.md#parse_job_hcl) | **post** /jobs/parse | parse a HCL jobspec and produce the equivalent JSON encoded job
[**plan_job**](JobsApi.md#plan_job) | **post** /job/{job_id}/plan | invokes a dry-run of the scheduler for the job
[**register_job**](JobsApi.md#register_job) | **post** /jobs | creates (aka \"registers\") a new job in the system
[**revert_job**](JobsApi.md#revert_job) | **post** /job/{job_id}/revert | reverts the job to an older version
[**scale_task_group**](JobsApi.md#scale_task_group) | **post** /job/{job_id}/scale | performs a scaling action against a job. Currently, this endpoint supports scaling the count for a task group
[**set_job_stability**](JobsApi.md#set_job_stability) | **post** /job/{job_id}/stable | sets the job's stability
[**stop_job**](JobsApi.md#stop_job) | **delete** /job/{job_id} | deregisters a job, and stops all allocations part of it
[**update_job**](JobsApi.md#update_job) | **post** /job/{job_id} | registers a new job or updates an existing job
[**validate_job**](JobsApi.md#validate_job) | **post** /validate/job | validate object structs, fields, and types



## dispatch_job

> crate::models::JobDispatchResponse dispatch_job(job_id, namespace, region, index, wait, job_dispatch_request)
dispatches a new instance of a parameterized job

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | job id | [required] |
**namespace** | Option<**String**> |  |  |
**region** | Option<**String**> | Make a request across regions to the given region |  |
**index** | Option<**i64**> | index used for blocking requests |  |
**wait** | Option<**String**> | wait time used for blocking requests |  |
**job_dispatch_request** | Option<[**JobDispatchRequest**](JobDispatchRequest.md)> |  |  |

### Return type

[**crate::models::JobDispatchResponse**](JobDispatchResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## evaluate_job

> crate::models::JobRegisterResponse evaluate_job(job_id, namespace, region, index, wait, job_evaluate_request)
creates a new evaluation for the given job. This can be used to force run the scheduling logic if necessary

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | job id | [required] |
**namespace** | Option<**String**> |  |  |
**region** | Option<**String**> | Make a request across regions to the given region |  |
**index** | Option<**i64**> | index used for blocking requests |  |
**wait** | Option<**String**> | wait time used for blocking requests |  |
**job_evaluate_request** | Option<[**JobEvaluateRequest**](JobEvaluateRequest.md)> |  |  |

### Return type

[**crate::models::JobRegisterResponse**](JobRegisterResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## force_new_periodic_instance

> crate::models::PeriodicForceResponse force_new_periodic_instance(job_id, namespace, region, index, wait)
forces a new instance of the periodic job. A new instance will be created even if it violates the job's prohibit_overlap settings. As such, this should be only used to immediately run a periodic job

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | job id | [required] |
**namespace** | Option<**String**> |  |  |
**region** | Option<**String**> | Make a request across regions to the given region |  |
**index** | Option<**i64**> | index used for blocking requests |  |
**wait** | Option<**String**> | wait time used for blocking requests |  |

### Return type

[**crate::models::PeriodicForceResponse**](PeriodicForceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_job

> crate::models::Job get_job(job_id, namespace, region, index, wait)
reads information about a single job for its specification and status

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | job id | [required] |
**namespace** | Option<**String**> |  |  |
**region** | Option<**String**> | Make a request across regions to the given region |  |
**index** | Option<**i64**> | index used for blocking requests |  |
**wait** | Option<**String**> | wait time used for blocking requests |  |

### Return type

[**crate::models::Job**](Job.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_job_allocations

> Vec<crate::models::AllocationListStub> get_job_allocations(job_id, all)
reads information about a single job's allocations

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | job id | [required] |
**all** | Option<**bool**> | Specifies whether should include * from a previously registered job with the same ID. This is possible if the job is deregistered and reregistered. |  |

### Return type

[**Vec<crate::models::AllocationListStub>**](AllocationListStub.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_job_deployments

> Vec<crate::models::Deployment> get_job_deployments(job_id, namespace, region, index, wait, all)
lists a single job's deployments

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | job id | [required] |
**namespace** | Option<**String**> |  |  |
**region** | Option<**String**> | Make a request across regions to the given region |  |
**index** | Option<**i64**> | index used for blocking requests |  |
**wait** | Option<**String**> | wait time used for blocking requests |  |
**all** | Option<**bool**> | Specifies whether should include * from a previously registered job with the same ID. This is possible if the job is deregistered and reregistered. |  |

### Return type

[**Vec<crate::models::Deployment>**](Deployment.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_job_evaluations

> Vec<crate::models::Evaluation> get_job_evaluations(job_id, namespace, region, index, wait)
reads information about a single job's evaluations

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | job id | [required] |
**namespace** | Option<**String**> |  |  |
**region** | Option<**String**> | Make a request across regions to the given region |  |
**index** | Option<**i64**> | index used for blocking requests |  |
**wait** | Option<**String**> | wait time used for blocking requests |  |

### Return type

[**Vec<crate::models::Evaluation>**](Evaluation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_job_latest_deployment

> crate::models::Deployment get_job_latest_deployment(job_id, namespace, region, index, wait)
get a single job's most recent deployment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | job id | [required] |
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


## get_job_scale_status

> crate::models::JobScaleStatusResponse get_job_scale_status(job_id, namespace, region, index, wait)
reads scale information about a job

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | job id | [required] |
**namespace** | Option<**String**> |  |  |
**region** | Option<**String**> | Make a request across regions to the given region |  |
**index** | Option<**i64**> | index used for blocking requests |  |
**wait** | Option<**String**> | wait time used for blocking requests |  |

### Return type

[**crate::models::JobScaleStatusResponse**](JobScaleStatusResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_job_summary

> crate::models::JobSummary get_job_summary(job_id, namespace, region, index, wait)
reads summary information about a job

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | job id | [required] |
**namespace** | Option<**String**> |  |  |
**region** | Option<**String**> | Make a request across regions to the given region |  |
**index** | Option<**i64**> | index used for blocking requests |  |
**wait** | Option<**String**> | wait time used for blocking requests |  |

### Return type

[**crate::models::JobSummary**](JobSummary.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_job_versions

> crate::models::JobVersionsResponse get_job_versions(job_id, namespace, region, index, wait)
reads information about all versions of a job

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | job id | [required] |
**namespace** | Option<**String**> |  |  |
**region** | Option<**String**> | Make a request across regions to the given region |  |
**index** | Option<**i64**> | index used for blocking requests |  |
**wait** | Option<**String**> | wait time used for blocking requests |  |

### Return type

[**crate::models::JobVersionsResponse**](JobVersionsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_jobs

> Vec<crate::models::JobListStub> get_jobs(namespace, region, index, wait, prefix)
lists all known jobs in the system registered with Nomad

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**namespace** | Option<**String**> |  |  |
**region** | Option<**String**> | Make a request across regions to the given region |  |
**index** | Option<**i64**> | index used for blocking requests |  |
**wait** | Option<**String**> | wait time used for blocking requests |  |
**prefix** | Option<**String**> | Specifies a string to filter jobs on based on an index prefix. This is specified as a query string parameter |  |

### Return type

[**Vec<crate::models::JobListStub>**](JobListStub.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## parse_job_hcl

> crate::models::Job parse_job_hcl(namespace, region, index, wait, jobs_parse_request)
parse a HCL jobspec and produce the equivalent JSON encoded job

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**namespace** | Option<**String**> |  |  |
**region** | Option<**String**> | Make a request across regions to the given region |  |
**index** | Option<**i64**> | index used for blocking requests |  |
**wait** | Option<**String**> | wait time used for blocking requests |  |
**jobs_parse_request** | Option<[**JobsParseRequest**](JobsParseRequest.md)> |  |  |

### Return type

[**crate::models::Job**](Job.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## plan_job

> crate::models::JobPlanResponse plan_job(job_id, namespace, region, index, wait, job_plan_request)
invokes a dry-run of the scheduler for the job

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | job id | [required] |
**namespace** | Option<**String**> |  |  |
**region** | Option<**String**> | Make a request across regions to the given region |  |
**index** | Option<**i64**> | index used for blocking requests |  |
**wait** | Option<**String**> | wait time used for blocking requests |  |
**job_plan_request** | Option<[**JobPlanRequest**](JobPlanRequest.md)> |  |  |

### Return type

[**crate::models::JobPlanResponse**](JobPlanResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## register_job

> crate::models::JobRegisterResponse register_job(namespace, region, index, wait, register_job_request)
creates (aka \"registers\") a new job in the system

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**namespace** | Option<**String**> |  |  |
**region** | Option<**String**> | Make a request across regions to the given region |  |
**index** | Option<**i64**> | index used for blocking requests |  |
**wait** | Option<**String**> | wait time used for blocking requests |  |
**register_job_request** | Option<[**RegisterJobRequest**](RegisterJobRequest.md)> |  |  |

### Return type

[**crate::models::JobRegisterResponse**](JobRegisterResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## revert_job

> crate::models::JobRegisterResponse revert_job(job_id, namespace, region, index, wait, job_revert_request)
reverts the job to an older version

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | job id | [required] |
**namespace** | Option<**String**> |  |  |
**region** | Option<**String**> | Make a request across regions to the given region |  |
**index** | Option<**i64**> | index used for blocking requests |  |
**wait** | Option<**String**> | wait time used for blocking requests |  |
**job_revert_request** | Option<[**JobRevertRequest**](JobRevertRequest.md)> |  |  |

### Return type

[**crate::models::JobRegisterResponse**](JobRegisterResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## scale_task_group

> crate::models::JobRegisterResponse scale_task_group(job_id, namespace, region, index, wait, scaling_request)
performs a scaling action against a job. Currently, this endpoint supports scaling the count for a task group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | job id | [required] |
**namespace** | Option<**String**> |  |  |
**region** | Option<**String**> | Make a request across regions to the given region |  |
**index** | Option<**i64**> | index used for blocking requests |  |
**wait** | Option<**String**> | wait time used for blocking requests |  |
**scaling_request** | Option<[**ScalingRequest**](ScalingRequest.md)> |  |  |

### Return type

[**crate::models::JobRegisterResponse**](JobRegisterResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_job_stability

> crate::models::JobStabilityResponse set_job_stability(job_id, namespace, region, index, wait, job_stability_request)
sets the job's stability

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | job id | [required] |
**namespace** | Option<**String**> |  |  |
**region** | Option<**String**> | Make a request across regions to the given region |  |
**index** | Option<**i64**> | index used for blocking requests |  |
**wait** | Option<**String**> | wait time used for blocking requests |  |
**job_stability_request** | Option<[**JobStabilityRequest**](JobStabilityRequest.md)> |  |  |

### Return type

[**crate::models::JobStabilityResponse**](JobStabilityResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stop_job

> crate::models::JobDeregisterResponse stop_job(job_id, namespace, region, index, wait, purge)
deregisters a job, and stops all allocations part of it

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | job id | [required] |
**namespace** | Option<**String**> |  |  |
**region** | Option<**String**> | Make a request across regions to the given region |  |
**index** | Option<**i64**> | index used for blocking requests |  |
**wait** | Option<**String**> | wait time used for blocking requests |  |
**purge** | Option<**bool**> | Specifies that the job should stopped and purged immediately. This means the job will not be queryable after being stopped. If not set, the job will be purged by the garbage collector |  |

### Return type

[**crate::models::JobDeregisterResponse**](JobDeregisterResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_job

> crate::models::JobRegisterResponse update_job(job_id, namespace, region, index, wait, register_job_request)
registers a new job or updates an existing job

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | job id | [required] |
**namespace** | Option<**String**> |  |  |
**region** | Option<**String**> | Make a request across regions to the given region |  |
**index** | Option<**i64**> | index used for blocking requests |  |
**wait** | Option<**String**> | wait time used for blocking requests |  |
**register_job_request** | Option<[**RegisterJobRequest**](RegisterJobRequest.md)> |  |  |

### Return type

[**crate::models::JobRegisterResponse**](JobRegisterResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## validate_job

> crate::models::JobValidateResponse validate_job(namespace, region, index, wait, job_validate_request)
validate object structs, fields, and types

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**namespace** | Option<**String**> |  |  |
**region** | Option<**String**> | Make a request across regions to the given region |  |
**index** | Option<**i64**> | index used for blocking requests |  |
**wait** | Option<**String**> | wait time used for blocking requests |  |
**job_validate_request** | Option<[**JobValidateRequest**](JobValidateRequest.md)> |  |  |

### Return type

[**crate::models::JobValidateResponse**](JobValidateResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

