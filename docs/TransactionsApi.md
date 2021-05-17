# \TransactionsApi

All URIs are relative to *http://localhost/fsp*

Method | HTTP request | Description
------------- | ------------- | -------------
[**transactions_by_id**](TransactionsApi.md#transactions_by_id) | **Get** /transactions/{ID} | TransactionsByID
[**transactions_by_id1**](TransactionsApi.md#transactions_by_id1) | **Put** /transactions/{ID} | TransactionsByID
[**transactions_error_by_id**](TransactionsApi.md#transactions_error_by_id) | **Put** /transactions/{ID}/error | TransactionsErrorByID


# **transactions_by_id**
> transactions_by_id(ID, content_type, date, fspiop_source, accept, optional)
TransactionsByID

The HTTP request GET /transactions/<ID> is used to get transaction information regarding a financial transaction created earlier. The <ID> in the URI should contain the transactionId that was used for the creation of the quote, as the transaction is created as part of another process (the transfer process).

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
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

# **transactions_by_id1**
> transactions_by_id1(ID, content_type, date, fspiop_source, body, optional)
TransactionsByID

The callback PUT /transactions/<ID> is used to inform the client of a requested transaction. The <ID> in the URI should contain the <ID> that was used in the GET /transactions/<ID>.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **ID** | **String**|  | 
  **content_type** | **String**| The Content-Type header indicates the specific version of the API used to send the payload body. | 
  **date** | **String**| The Date header field indicates the date when the request was sent. | 
  **fspiop_source** | **String**| The FSPIOP-Source header field is a non-HTTP standard field used by the API for identifying the sender of the HTTP request. The field should be set by the original sender of the request. Required for routing and signature verification (see header field FSPIOP-Signature). | 
  **body** | [**TransactionsIdPutResponse**](TransactionsIdPutResponse.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ID** | **String**|  | 
 **content_type** | **String**| The Content-Type header indicates the specific version of the API used to send the payload body. | 
 **date** | **String**| The Date header field indicates the date when the request was sent. | 
 **fspiop_source** | **String**| The FSPIOP-Source header field is a non-HTTP standard field used by the API for identifying the sender of the HTTP request. The field should be set by the original sender of the request. Required for routing and signature verification (see header field FSPIOP-Signature). | 
 **body** | [**TransactionsIdPutResponse**](TransactionsIdPutResponse.md)|  | 
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

# **transactions_error_by_id**
> transactions_error_by_id(ID, body, content_type, date, fspiop_source, optional)
TransactionsErrorByID

If the server is unable to find or create a transaction, or another processing error occurs, the error callback PUT /transactions/<ID>/error is used. The <ID> in the URI should contain the <ID> that was used in the GET /transactions/<ID>.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
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

