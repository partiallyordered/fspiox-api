# \PartiesApi

All URIs are relative to *http://localhost/fsp*

Method | HTTP request | Description
------------- | ------------- | -------------
[**parties_by_type_and_id**](PartiesApi.md#parties_by_type_and_id) | **Get** /parties/{Type}/{ID} | PartiesByTypeAndID
[**parties_by_type_and_id2**](PartiesApi.md#parties_by_type_and_id2) | **Put** /parties/{Type}/{ID} | PartiesByTypeAndID2
[**parties_error_by_type_and_id**](PartiesApi.md#parties_error_by_type_and_id) | **Put** /parties/{Type}/{ID}/error | PartiesErrorByTypeAndID
[**parties_sub_id_by_type_and_id**](PartiesApi.md#parties_sub_id_by_type_and_id) | **Get** /parties/{Type}/{ID}/{SubId} | PartiesSubIdByTypeAndID
[**parties_sub_id_by_type_and_id_put**](PartiesApi.md#parties_sub_id_by_type_and_id_put) | **Put** /parties/{Type}/{ID}/{SubId} | PartiesSubIdByTypeAndID
[**parties_sub_id_error_by_type_and_id**](PartiesApi.md#parties_sub_id_error_by_type_and_id) | **Put** /parties/{Type}/{ID}/{SubId}/error | PartiesSubIdErrorByTypeAndID


# **parties_by_type_and_id**
> parties_by_type_and_id(_type, ID, content_type, date, fspiop_source, accept, optional)
PartiesByTypeAndID

The HTTP request GET /parties/<Type>/<ID> (or GET /parties/<Type>/<ID>/<SubId>) is used to lookup information regarding the requested Party, defined by <Type>, <ID> and optionally <SubId> (for example, GET /parties/MSISDN/123456789, or GET /parties/BUSINESS/shoecompany/employee1).

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **_type** | **String**|  | 
  **ID** | **String**|  | 
  **content_type** | **String**| The Content-Type header indicates the specific version of the API used to send the payload body. | 
  **date** | **String**| The Date header field indicates the date when the request was sent. | 
  **fspiop_source** | **String**| The FSPIOP-Source header field is a non-HTTP standard field used by the API for identifying the sender of the HTTP request. The field should be set by the original sender of the request. Required for routing and signature verification (see header field FSPIOP-Signature). | 
  **accept** | **String**| The Accept header field indicates the version of the API the client would like the server to use. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **_type** | **String**|  | 
 **ID** | **String**|  | 
 **content_type** | **String**| The Content-Type header indicates the specific version of the API used to send the payload body. | 
 **date** | **String**| The Date header field indicates the date when the request was sent. | 
 **fspiop_source** | **String**| The FSPIOP-Source header field is a non-HTTP standard field used by the API for identifying the sender of the HTTP request. The field should be set by the original sender of the request. Required for routing and signature verification (see header field FSPIOP-Signature). | 
 **accept** | **String**| The Accept header field indicates the version of the API the client would like the server to use. | 
 **x_forwarded_for** | **String**| The X-Forwarded-For header field is an unofficially accepted standard used for informational purposes of the originating client IP address, as a request might pass multiple proxies, firewalls, and so on. Multiple X-Forwarded-For values as in the example shown here should be expected and supported by implementers of the API. Note - An alternative to X-Forwarded-For is defined in RFC 7239. However, to this point RFC 7239 is less-used and supported than X-Forwarded-For. | 
 **fspiop_destination** | **String**| The &#x60;FSPIOP-Destination&#x60; header field is a non-HTTP standard field used by the API for HTTP header-based routing of requests and responses to the destination. The field must be set by the original sender of the request if the destination is known (valid for all services except GET /parties) so that any entities between the client and the server do not need to parse the payload for routing purposes. If the destination is not known (valid for service GET /parties), the field should be left empty. | 
 **fspiop_encryption** | **String**| The FSPIOP-Encryption header field is a non-HTTP standard field used by the API for applying end-to-end encryption of the request. | 
 **fspiop_signature** | **String**| The FSPIOP-Signature header field is a non-HTTP standard field used by the API for applying an end-to-end request signature. | 
 **FSPIOP_URI** | **String**| The FSPIOP-URI header field is a non-HTTP standard field used by the API for signature verification, should contain the service URI. Required if signature verification is used, for more information see API Signature document. | 
 **fspiop_http_method** | **String**| The FSPIOP-HTTP-Method header field is a non-HTTP standard field used by the API for signature verification, should contain the service HTTP method. Required if signature verification is used, for more information see API Signature document. | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **parties_by_type_and_id2**
> parties_by_type_and_id2(_type, ID, content_type, date, fspiop_source, body, optional)
PartiesByTypeAndID2

The callback PUT /parties/<Type>/<ID> (or PUT /parties/<Type>/<ID>/<SubId>) is used to inform the client of a successful result of the Party information lookup.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **_type** | **String**|  | 
  **ID** | **String**|  | 
  **content_type** | **String**| The Content-Type header indicates the specific version of the API used to send the payload body. | 
  **date** | **String**| The Date header field indicates the date when the request was sent. | 
  **fspiop_source** | **String**| The FSPIOP-Source header field is a non-HTTP standard field used by the API for identifying the sender of the HTTP request. The field should be set by the original sender of the request. Required for routing and signature verification (see header field FSPIOP-Signature). | 
  **body** | [**PartiesTypeIdPutResponse**](PartiesTypeIdPutResponse.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **_type** | **String**|  | 
 **ID** | **String**|  | 
 **content_type** | **String**| The Content-Type header indicates the specific version of the API used to send the payload body. | 
 **date** | **String**| The Date header field indicates the date when the request was sent. | 
 **fspiop_source** | **String**| The FSPIOP-Source header field is a non-HTTP standard field used by the API for identifying the sender of the HTTP request. The field should be set by the original sender of the request. Required for routing and signature verification (see header field FSPIOP-Signature). | 
 **body** | [**PartiesTypeIdPutResponse**](PartiesTypeIdPutResponse.md)|  | 
 **x_forwarded_for** | **String**| The X-Forwarded-For header field is an unofficially accepted standard used for informational purposes of the originating client IP address, as a request might pass multiple proxies, firewalls, and so on. Multiple X-Forwarded-For values as in the example shown here should be expected and supported by implementers of the API. Note - An alternative to X-Forwarded-For is defined in RFC 7239. However, to this point RFC 7239 is less-used and supported than X-Forwarded-For. | 
 **fspiop_destination** | **String**| The &#x60;FSPIOP-Destination&#x60; header field is a non-HTTP standard field used by the API for HTTP header-based routing of requests and responses to the destination. The field must be set by the original sender of the request if the destination is known (valid for all services except GET /parties) so that any entities between the client and the server do not need to parse the payload for routing purposes. If the destination is not known (valid for service GET /parties), the field should be left empty. | 
 **fspiop_encryption** | **String**| The FSPIOP-Encryption header field is a non-HTTP standard field used by the API for applying end-to-end encryption of the request. | 
 **fspiop_signature** | **String**| The FSPIOP-Signature header field is a non-HTTP standard field used by the API for applying an end-to-end request signature. | 
 **FSPIOP_URI** | **String**| The FSPIOP-URI header field is a non-HTTP standard field used by the API for signature verification, should contain the service URI. Required if signature verification is used, for more information see API Signature document. | 
 **fspiop_http_method** | **String**| The FSPIOP-HTTP-Method header field is a non-HTTP standard field used by the API for signature verification, should contain the service HTTP method. Required if signature verification is used, for more information see API Signature document. | 
 **content_length** | **i32**| The Content-Length header field indicates the anticipated size of the payload body. Only sent if there is a body. Note - The API supports a maximum size of 5242880 bytes (5 Megabytes) | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **parties_error_by_type_and_id**
> parties_error_by_type_and_id(_type, ID, body, content_type, date, fspiop_source, optional)
PartiesErrorByTypeAndID

If the server is unable to find Party information of the provided identity, or another processing error occurred, the error callback PUT /parties/<Type>/<ID>/error (or PUT /parties/<Type>/<ID>/<SubId>/error) is used.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **_type** | **String**|  | 
  **ID** | **String**|  | 
  **body** | [**ErrorInformationObject**](ErrorInformationObject.md)|  | 
  **content_type** | **String**| The Content-Type header indicates the specific version of the API used to send the payload body. | 
  **date** | **String**| The Date header field indicates the date when the request was sent. | 
  **fspiop_source** | **String**| The FSPIOP-Source header field is a non-HTTP standard field used by the API for identifying the sender of the HTTP request. The field should be set by the original sender of the request. Required for routing and signature verification (see header field FSPIOP-Signature). | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **_type** | **String**|  | 
 **ID** | **String**|  | 
 **body** | [**ErrorInformationObject**](ErrorInformationObject.md)|  | 
 **content_type** | **String**| The Content-Type header indicates the specific version of the API used to send the payload body. | 
 **date** | **String**| The Date header field indicates the date when the request was sent. | 
 **fspiop_source** | **String**| The FSPIOP-Source header field is a non-HTTP standard field used by the API for identifying the sender of the HTTP request. The field should be set by the original sender of the request. Required for routing and signature verification (see header field FSPIOP-Signature). | 
 **content_length** | **i32**| The Content-Length header field indicates the anticipated size of the payload body. Only sent if there is a body. Note - The API supports a maximum size of 5242880 bytes (5 Megabytes) | 
 **x_forwarded_for** | **String**| The X-Forwarded-For header field is an unofficially accepted standard used for informational purposes of the originating client IP address, as a request might pass multiple proxies, firewalls, and so on. Multiple X-Forwarded-For values as in the example shown here should be expected and supported by implementers of the API. Note - An alternative to X-Forwarded-For is defined in RFC 7239. However, to this point RFC 7239 is less-used and supported than X-Forwarded-For. | 
 **fspiop_destination** | **String**| The &#x60;FSPIOP-Destination&#x60; header field is a non-HTTP standard field used by the API for HTTP header-based routing of requests and responses to the destination. The field must be set by the original sender of the request if the destination is known (valid for all services except GET /parties) so that any entities between the client and the server do not need to parse the payload for routing purposes. If the destination is not known (valid for service GET /parties), the field should be left empty. | 
 **fspiop_encryption** | **String**| The FSPIOP-Encryption header field is a non-HTTP standard field used by the API for applying end-to-end encryption of the request. | 
 **fspiop_signature** | **String**| The FSPIOP-Signature header field is a non-HTTP standard field used by the API for applying an end-to-end request signature. | 
 **FSPIOP_URI** | **String**| The FSPIOP-URI header field is a non-HTTP standard field used by the API for signature verification, should contain the service URI. Required if signature verification is used, for more information see API Signature document. | 
 **fspiop_http_method** | **String**| The FSPIOP-HTTP-Method header field is a non-HTTP standard field used by the API for signature verification, should contain the service HTTP method. Required if signature verification is used, for more information see API Signature document. | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **parties_sub_id_by_type_and_id**
> parties_sub_id_by_type_and_id(_type, ID, sub_id, content_type, date, fspiop_source, accept, optional)
PartiesSubIdByTypeAndID

The HTTP request GET /parties/<Type>/<ID> (or GET /parties/<Type>/<ID>/<SubId>) is used to lookup information regarding the requested Party, defined by <Type>, <ID> and optionally <SubId> (for example, GET /parties/MSISDN/123456789, or GET /parties/BUSINESS/shoecompany/employee1).

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **_type** | **String**|  | 
  **ID** | **String**|  | 
  **sub_id** | **String**|  | 
  **content_type** | **String**| The Content-Type header indicates the specific version of the API used to send the payload body. | 
  **date** | **String**| The Date header field indicates the date when the request was sent. | 
  **fspiop_source** | **String**| The FSPIOP-Source header field is a non-HTTP standard field used by the API for identifying the sender of the HTTP request. The field should be set by the original sender of the request. Required for routing and signature verification (see header field FSPIOP-Signature). | 
  **accept** | **String**| The Accept header field indicates the version of the API the client would like the server to use. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **_type** | **String**|  | 
 **ID** | **String**|  | 
 **sub_id** | **String**|  | 
 **content_type** | **String**| The Content-Type header indicates the specific version of the API used to send the payload body. | 
 **date** | **String**| The Date header field indicates the date when the request was sent. | 
 **fspiop_source** | **String**| The FSPIOP-Source header field is a non-HTTP standard field used by the API for identifying the sender of the HTTP request. The field should be set by the original sender of the request. Required for routing and signature verification (see header field FSPIOP-Signature). | 
 **accept** | **String**| The Accept header field indicates the version of the API the client would like the server to use. | 
 **x_forwarded_for** | **String**| The X-Forwarded-For header field is an unofficially accepted standard used for informational purposes of the originating client IP address, as a request might pass multiple proxies, firewalls, and so on. Multiple X-Forwarded-For values as in the example shown here should be expected and supported by implementers of the API. Note - An alternative to X-Forwarded-For is defined in RFC 7239. However, to this point RFC 7239 is less-used and supported than X-Forwarded-For. | 
 **fspiop_destination** | **String**| The &#x60;FSPIOP-Destination&#x60; header field is a non-HTTP standard field used by the API for HTTP header-based routing of requests and responses to the destination. The field must be set by the original sender of the request if the destination is known (valid for all services except GET /parties) so that any entities between the client and the server do not need to parse the payload for routing purposes. If the destination is not known (valid for service GET /parties), the field should be left empty. | 
 **fspiop_encryption** | **String**| The FSPIOP-Encryption header field is a non-HTTP standard field used by the API for applying end-to-end encryption of the request. | 
 **fspiop_signature** | **String**| The FSPIOP-Signature header field is a non-HTTP standard field used by the API for applying an end-to-end request signature. | 
 **FSPIOP_URI** | **String**| The FSPIOP-URI header field is a non-HTTP standard field used by the API for signature verification, should contain the service URI. Required if signature verification is used, for more information see API Signature document. | 
 **fspiop_http_method** | **String**| The FSPIOP-HTTP-Method header field is a non-HTTP standard field used by the API for signature verification, should contain the service HTTP method. Required if signature verification is used, for more information see API Signature document. | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **parties_sub_id_by_type_and_id_put**
> parties_sub_id_by_type_and_id_put(_type, ID, sub_id, content_type, date, fspiop_source, body, optional)
PartiesSubIdByTypeAndID

The callback PUT /parties/<Type>/<ID> (or PUT /parties/<Type>/<ID>/<SubId>) is used to inform the client of a successful result of the Party information lookup.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **_type** | **String**|  | 
  **ID** | **String**|  | 
  **sub_id** | **String**|  | 
  **content_type** | **String**| The Content-Type header indicates the specific version of the API used to send the payload body. | 
  **date** | **String**| The Date header field indicates the date when the request was sent. | 
  **fspiop_source** | **String**| The FSPIOP-Source header field is a non-HTTP standard field used by the API for identifying the sender of the HTTP request. The field should be set by the original sender of the request. Required for routing and signature verification (see header field FSPIOP-Signature). | 
  **body** | [**PartiesTypeIdPutResponse**](PartiesTypeIdPutResponse.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **_type** | **String**|  | 
 **ID** | **String**|  | 
 **sub_id** | **String**|  | 
 **content_type** | **String**| The Content-Type header indicates the specific version of the API used to send the payload body. | 
 **date** | **String**| The Date header field indicates the date when the request was sent. | 
 **fspiop_source** | **String**| The FSPIOP-Source header field is a non-HTTP standard field used by the API for identifying the sender of the HTTP request. The field should be set by the original sender of the request. Required for routing and signature verification (see header field FSPIOP-Signature). | 
 **body** | [**PartiesTypeIdPutResponse**](PartiesTypeIdPutResponse.md)|  | 
 **x_forwarded_for** | **String**| The X-Forwarded-For header field is an unofficially accepted standard used for informational purposes of the originating client IP address, as a request might pass multiple proxies, firewalls, and so on. Multiple X-Forwarded-For values as in the example shown here should be expected and supported by implementers of the API. Note - An alternative to X-Forwarded-For is defined in RFC 7239. However, to this point RFC 7239 is less-used and supported than X-Forwarded-For. | 
 **fspiop_destination** | **String**| The &#x60;FSPIOP-Destination&#x60; header field is a non-HTTP standard field used by the API for HTTP header-based routing of requests and responses to the destination. The field must be set by the original sender of the request if the destination is known (valid for all services except GET /parties) so that any entities between the client and the server do not need to parse the payload for routing purposes. If the destination is not known (valid for service GET /parties), the field should be left empty. | 
 **fspiop_encryption** | **String**| The FSPIOP-Encryption header field is a non-HTTP standard field used by the API for applying end-to-end encryption of the request. | 
 **fspiop_signature** | **String**| The FSPIOP-Signature header field is a non-HTTP standard field used by the API for applying an end-to-end request signature. | 
 **FSPIOP_URI** | **String**| The FSPIOP-URI header field is a non-HTTP standard field used by the API for signature verification, should contain the service URI. Required if signature verification is used, for more information see API Signature document. | 
 **fspiop_http_method** | **String**| The FSPIOP-HTTP-Method header field is a non-HTTP standard field used by the API for signature verification, should contain the service HTTP method. Required if signature verification is used, for more information see API Signature document. | 
 **content_length** | **i32**| The Content-Length header field indicates the anticipated size of the payload body. Only sent if there is a body. Note - The API supports a maximum size of 5242880 bytes (5 Megabytes) | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **parties_sub_id_error_by_type_and_id**
> parties_sub_id_error_by_type_and_id(_type, ID, sub_id, body, content_type, date, fspiop_source, optional)
PartiesSubIdErrorByTypeAndID

If the server is unable to find Party information of the provided identity, or another processing error occurred, the error callback PUT /parties/<Type>/<ID>/error (or PUT /parties/<Type>/<ID>/<SubId>/error) is used.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **_type** | **String**|  | 
  **ID** | **String**|  | 
  **sub_id** | **String**|  | 
  **body** | [**ErrorInformationObject**](ErrorInformationObject.md)|  | 
  **content_type** | **String**| The Content-Type header indicates the specific version of the API used to send the payload body. | 
  **date** | **String**| The Date header field indicates the date when the request was sent. | 
  **fspiop_source** | **String**| The FSPIOP-Source header field is a non-HTTP standard field used by the API for identifying the sender of the HTTP request. The field should be set by the original sender of the request. Required for routing and signature verification (see header field FSPIOP-Signature). | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **_type** | **String**|  | 
 **ID** | **String**|  | 
 **sub_id** | **String**|  | 
 **body** | [**ErrorInformationObject**](ErrorInformationObject.md)|  | 
 **content_type** | **String**| The Content-Type header indicates the specific version of the API used to send the payload body. | 
 **date** | **String**| The Date header field indicates the date when the request was sent. | 
 **fspiop_source** | **String**| The FSPIOP-Source header field is a non-HTTP standard field used by the API for identifying the sender of the HTTP request. The field should be set by the original sender of the request. Required for routing and signature verification (see header field FSPIOP-Signature). | 
 **content_length** | **i32**| The Content-Length header field indicates the anticipated size of the payload body. Only sent if there is a body. Note - The API supports a maximum size of 5242880 bytes (5 Megabytes) | 
 **x_forwarded_for** | **String**| The X-Forwarded-For header field is an unofficially accepted standard used for informational purposes of the originating client IP address, as a request might pass multiple proxies, firewalls, and so on. Multiple X-Forwarded-For values as in the example shown here should be expected and supported by implementers of the API. Note - An alternative to X-Forwarded-For is defined in RFC 7239. However, to this point RFC 7239 is less-used and supported than X-Forwarded-For. | 
 **fspiop_destination** | **String**| The &#x60;FSPIOP-Destination&#x60; header field is a non-HTTP standard field used by the API for HTTP header-based routing of requests and responses to the destination. The field must be set by the original sender of the request if the destination is known (valid for all services except GET /parties) so that any entities between the client and the server do not need to parse the payload for routing purposes. If the destination is not known (valid for service GET /parties), the field should be left empty. | 
 **fspiop_encryption** | **String**| The FSPIOP-Encryption header field is a non-HTTP standard field used by the API for applying end-to-end encryption of the request. | 
 **fspiop_signature** | **String**| The FSPIOP-Signature header field is a non-HTTP standard field used by the API for applying an end-to-end request signature. | 
 **FSPIOP_URI** | **String**| The FSPIOP-URI header field is a non-HTTP standard field used by the API for signature verification, should contain the service URI. Required if signature verification is used, for more information see API Signature document. | 
 **fspiop_http_method** | **String**| The FSPIOP-HTTP-Method header field is a non-HTTP standard field used by the API for signature verification, should contain the service HTTP method. Required if signature verification is used, for more information see API Signature document. | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

