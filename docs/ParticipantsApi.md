# \ParticipantsApi

All URIs are relative to *http://localhost/fsp*

Method | HTTP request | Description
------------- | ------------- | -------------
[**participants1**](ParticipantsApi.md#participants1) | **post** /participants | Participants
[**participants_by_id_and_error**](ParticipantsApi.md#participants_by_id_and_error) | **put** /participants/{ID}/error | ParticipantsByIDAndError
[**participants_by_id_and_type**](ParticipantsApi.md#participants_by_id_and_type) | **post** /participants/{Type}/{ID} | ParticipantsByIDAndType
[**participants_by_id_put**](ParticipantsApi.md#participants_by_id_put) | **put** /participants/{ID} | ParticipantsByID
[**participants_by_type_and_id**](ParticipantsApi.md#participants_by_type_and_id) | **get** /participants/{Type}/{ID} | ParticipantsByTypeAndID
[**participants_by_type_and_id2**](ParticipantsApi.md#participants_by_type_and_id2) | **delete** /participants/{Type}/{ID} | ParticipantsByTypeAndID
[**participants_by_type_and_id3**](ParticipantsApi.md#participants_by_type_and_id3) | **put** /participants/{Type}/{ID} | ParticipantsByTypeAndID
[**participants_error_by_type_and_id**](ParticipantsApi.md#participants_error_by_type_and_id) | **put** /participants/{Type}/{ID}/error | ParticipantsErrorByTypeAndID
[**participants_sub_id_by_type_and_id**](ParticipantsApi.md#participants_sub_id_by_type_and_id) | **get** /participants/{Type}/{ID}/{SubId} | ParticipantsSubIdByTypeAndID
[**participants_sub_id_by_type_and_id2**](ParticipantsApi.md#participants_sub_id_by_type_and_id2) | **delete** /participants/{Type}/{ID}/{SubId} | ParticipantsSubIdByTypeAndID
[**participants_sub_id_by_type_and_id3**](ParticipantsApi.md#participants_sub_id_by_type_and_id3) | **put** /participants/{Type}/{ID}/{SubId} | ParticipantsSubIdByTypeAndID
[**participants_sub_id_by_type_and_id_post**](ParticipantsApi.md#participants_sub_id_by_type_and_id_post) | **post** /participants/{Type}/{ID}/{SubId} | ParticipantsSubIdByTypeAndID
[**participants_sub_id_error_by_type_and_id**](ParticipantsApi.md#participants_sub_id_error_by_type_and_id) | **put** /participants/{Type}/{ID}/{SubId}/error | ParticipantsSubIdErrorByTypeAndID



## participants1

> participants1(accept, content_type, date, fspiop_source, body, content_length, x_forwarded_for, fspiop_destination, fspiop_encryption, fspiop_signature, FSPIOP_URI, fspiop_http_method)
Participants

The HTTP request POST /participants is used to create information in the server regarding the provided list of identities. This request should be used for bulk creation of FSP information for more than one Party. The optional currency parameter should indicate that each provided Party supports the currency

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**accept** | **String** | The Accept header field indicates the version of the API the client would like the server to use. | [required] |
**content_type** | **String** | The Content-Type header indicates the specific version of the API used to send the payload body. | [required] |
**date** | **String** | The Date header field indicates the date when the request was sent. | [required] |
**fspiop_source** | **String** | The FSPIOP-Source header field is a non-HTTP standard field used by the API for identifying the sender of the HTTP request. The field should be set by the original sender of the request. Required for routing and signature verification (see header field FSPIOP-Signature). | [required] |
**body** | [**ParticipantsPostRequest**](ParticipantsPostRequest.md) |  | [required] |
**content_length** | Option<**i32**> | The Content-Length header field indicates the anticipated size of the payload body. Only sent if there is a body. Note - The API supports a maximum size of 5242880 bytes (5 Megabytes) |  |
**x_forwarded_for** | Option<**String**> | The X-Forwarded-For header field is an unofficially accepted standard used for informational purposes of the originating client IP address, as a request might pass multiple proxies, firewalls, and so on. Multiple X-Forwarded-For values as in the example shown here should be expected and supported by implementers of the API. Note - An alternative to X-Forwarded-For is defined in RFC 7239. However, to this point RFC 7239 is less-used and supported than X-Forwarded-For. |  |
**fspiop_destination** | Option<**String**> | The `FSPIOP-Destination` header field is a non-HTTP standard field used by the API for HTTP header-based routing of requests and responses to the destination. The field must be set by the original sender of the request if the destination is known (valid for all services except GET /parties) so that any entities between the client and the server do not need to parse the payload for routing purposes. If the destination is not known (valid for service GET /parties), the field should be left empty. |  |
**fspiop_encryption** | Option<**String**> | The FSPIOP-Encryption header field is a non-HTTP standard field used by the API for applying end-to-end encryption of the request. |  |
**fspiop_signature** | Option<**String**> | The FSPIOP-Signature header field is a non-HTTP standard field used by the API for applying an end-to-end request signature. |  |
**FSPIOP_URI** | Option<**String**> | The FSPIOP-URI header field is a non-HTTP standard field used by the API for signature verification, should contain the service URI. Required if signature verification is used, for more information see API Signature document. |  |
**fspiop_http_method** | Option<**String**> | The FSPIOP-HTTP-Method header field is a non-HTTP standard field used by the API for signature verification, should contain the service HTTP method. Required if signature verification is used, for more information see API Signature document. |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## participants_by_id_and_error

> participants_by_id_and_error(ID, content_type, date, fspiop_source, body, content_length, x_forwarded_for, fspiop_destination, fspiop_encryption, fspiop_signature, FSPIOP_URI, fspiop_http_method)
ParticipantsByIDAndError

If there is an error during FSP information creation in the server, the error callback PUT /participants/{ID}/error is used. The <ID> in the URI should contain the requestId that was used for the creation of the participant information.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ID** | **String** |  | [required] |
**content_type** | **String** | The Content-Type header indicates the specific version of the API used to send the payload body. | [required] |
**date** | **String** | The Date header field indicates the date when the request was sent. | [required] |
**fspiop_source** | **String** | The FSPIOP-Source header field is a non-HTTP standard field used by the API for identifying the sender of the HTTP request. The field should be set by the original sender of the request. Required for routing and signature verification (see header field FSPIOP-Signature). | [required] |
**body** | [**ErrorInformationObject**](ErrorInformationObject.md) |  | [required] |
**content_length** | Option<**i32**> | The Content-Length header field indicates the anticipated size of the payload body. Only sent if there is a body. Note - The API supports a maximum size of 5242880 bytes (5 Megabytes) |  |
**x_forwarded_for** | Option<**String**> | The X-Forwarded-For header field is an unofficially accepted standard used for informational purposes of the originating client IP address, as a request might pass multiple proxies, firewalls, and so on. Multiple X-Forwarded-For values as in the example shown here should be expected and supported by implementers of the API. Note - An alternative to X-Forwarded-For is defined in RFC 7239. However, to this point RFC 7239 is less-used and supported than X-Forwarded-For. |  |
**fspiop_destination** | Option<**String**> | The `FSPIOP-Destination` header field is a non-HTTP standard field used by the API for HTTP header-based routing of requests and responses to the destination. The field must be set by the original sender of the request if the destination is known (valid for all services except GET /parties) so that any entities between the client and the server do not need to parse the payload for routing purposes. If the destination is not known (valid for service GET /parties), the field should be left empty. |  |
**fspiop_encryption** | Option<**String**> | The FSPIOP-Encryption header field is a non-HTTP standard field used by the API for applying end-to-end encryption of the request. |  |
**fspiop_signature** | Option<**String**> | The FSPIOP-Signature header field is a non-HTTP standard field used by the API for applying an end-to-end request signature. |  |
**FSPIOP_URI** | Option<**String**> | The FSPIOP-URI header field is a non-HTTP standard field used by the API for signature verification, should contain the service URI. Required if signature verification is used, for more information see API Signature document. |  |
**fspiop_http_method** | Option<**String**> | The FSPIOP-HTTP-Method header field is a non-HTTP standard field used by the API for signature verification, should contain the service HTTP method. Required if signature verification is used, for more information see API Signature document. |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## participants_by_id_and_type

> participants_by_id_and_type(_type, ID, content_type, date, fspiop_source, accept, body, x_forwarded_for, fspiop_destination, fspiop_encryption, fspiop_signature, FSPIOP_URI, fspiop_http_method, content_length)
ParticipantsByIDAndType

The HTTP request POST /participants/<Type>/<ID> (or POST /participants/<Type>/<ID>/<SubId>) is used to create information in the server regarding the provided identity, defined by <Type>, <ID>, and optionally <SubId> (for example, POST /participants/MSISDN/123456789 or POST /participants/BUSINESS/shoecompany/employee1). An ExtensionList element has been added to this request in version v1.1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**_type** | **String** |  | [required] |
**ID** | **String** |  | [required] |
**content_type** | **String** | The Content-Type header indicates the specific version of the API used to send the payload body. | [required] |
**date** | **String** | The Date header field indicates the date when the request was sent. | [required] |
**fspiop_source** | **String** | The FSPIOP-Source header field is a non-HTTP standard field used by the API for identifying the sender of the HTTP request. The field should be set by the original sender of the request. Required for routing and signature verification (see header field FSPIOP-Signature). | [required] |
**accept** | **String** | The Accept header field indicates the version of the API the client would like the server to use. | [required] |
**body** | [**ParticipantsTypeIdSubIdPostRequest**](ParticipantsTypeIdSubIdPostRequest.md) |  | [required] |
**x_forwarded_for** | Option<**String**> | The X-Forwarded-For header field is an unofficially accepted standard used for informational purposes of the originating client IP address, as a request might pass multiple proxies, firewalls, and so on. Multiple X-Forwarded-For values as in the example shown here should be expected and supported by implementers of the API. Note - An alternative to X-Forwarded-For is defined in RFC 7239. However, to this point RFC 7239 is less-used and supported than X-Forwarded-For. |  |
**fspiop_destination** | Option<**String**> | The `FSPIOP-Destination` header field is a non-HTTP standard field used by the API for HTTP header-based routing of requests and responses to the destination. The field must be set by the original sender of the request if the destination is known (valid for all services except GET /parties) so that any entities between the client and the server do not need to parse the payload for routing purposes. If the destination is not known (valid for service GET /parties), the field should be left empty. |  |
**fspiop_encryption** | Option<**String**> | The FSPIOP-Encryption header field is a non-HTTP standard field used by the API for applying end-to-end encryption of the request. |  |
**fspiop_signature** | Option<**String**> | The FSPIOP-Signature header field is a non-HTTP standard field used by the API for applying an end-to-end request signature. |  |
**FSPIOP_URI** | Option<**String**> | The FSPIOP-URI header field is a non-HTTP standard field used by the API for signature verification, should contain the service URI. Required if signature verification is used, for more information see API Signature document. |  |
**fspiop_http_method** | Option<**String**> | The FSPIOP-HTTP-Method header field is a non-HTTP standard field used by the API for signature verification, should contain the service HTTP method. Required if signature verification is used, for more information see API Signature document. |  |
**content_length** | Option<**i32**> | The Content-Length header field indicates the anticipated size of the payload body. Only sent if there is a body. Note - The API supports a maximum size of 5242880 bytes (5 Megabytes) |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## participants_by_id_put

> participants_by_id_put(ID, content_type, date, fspiop_source, body, content_length, x_forwarded_for, fspiop_destination, fspiop_encryption, fspiop_signature, FSPIOP_URI, fspiop_http_method)
ParticipantsByID

The callback PUT /participants/<ID> is used to inform the client of the result of the creation of the provided list of identities.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ID** | **String** |  | [required] |
**content_type** | **String** | The Content-Type header indicates the specific version of the API used to send the payload body. | [required] |
**date** | **String** | The Date header field indicates the date when the request was sent. | [required] |
**fspiop_source** | **String** | The FSPIOP-Source header field is a non-HTTP standard field used by the API for identifying the sender of the HTTP request. The field should be set by the original sender of the request. Required for routing and signature verification (see header field FSPIOP-Signature). | [required] |
**body** | [**ParticipantsIdPutResponse**](ParticipantsIdPutResponse.md) |  | [required] |
**content_length** | Option<**i32**> | The Content-Length header field indicates the anticipated size of the payload body. Only sent if there is a body. Note - The API supports a maximum size of 5242880 bytes (5 Megabytes) |  |
**x_forwarded_for** | Option<**String**> | The X-Forwarded-For header field is an unofficially accepted standard used for informational purposes of the originating client IP address, as a request might pass multiple proxies, firewalls, and so on. Multiple X-Forwarded-For values as in the example shown here should be expected and supported by implementers of the API. Note - An alternative to X-Forwarded-For is defined in RFC 7239. However, to this point RFC 7239 is less-used and supported than X-Forwarded-For. |  |
**fspiop_destination** | Option<**String**> | The `FSPIOP-Destination` header field is a non-HTTP standard field used by the API for HTTP header-based routing of requests and responses to the destination. The field must be set by the original sender of the request if the destination is known (valid for all services except GET /parties) so that any entities between the client and the server do not need to parse the payload for routing purposes. If the destination is not known (valid for service GET /parties), the field should be left empty. |  |
**fspiop_encryption** | Option<**String**> | The FSPIOP-Encryption header field is a non-HTTP standard field used by the API for applying end-to-end encryption of the request. |  |
**fspiop_signature** | Option<**String**> | The FSPIOP-Signature header field is a non-HTTP standard field used by the API for applying an end-to-end request signature. |  |
**FSPIOP_URI** | Option<**String**> | The FSPIOP-URI header field is a non-HTTP standard field used by the API for signature verification, should contain the service URI. Required if signature verification is used, for more information see API Signature document. |  |
**fspiop_http_method** | Option<**String**> | The FSPIOP-HTTP-Method header field is a non-HTTP standard field used by the API for signature verification, should contain the service HTTP method. Required if signature verification is used, for more information see API Signature document. |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## participants_by_type_and_id

> participants_by_type_and_id(_type, ID, content_type, date, fspiop_source, accept, x_forwarded_for, fspiop_destination, fspiop_encryption, fspiop_signature, FSPIOP_URI, fspiop_http_method)
ParticipantsByTypeAndID

The HTTP request GET /participants/<Type>/<ID> (or GET /participants/<Type>/<ID>/<SubId>) is used to find out in which FSP the requested Party, defined by <Type>, <ID> and optionally <SubId>, is located (for example, GET /participants/MSISDN/123456789, or GET /participants/BUSINESS/shoecompany/employee1). This HTTP request should support a query string for filtering of currency. To use filtering of currency, the HTTP request GET /participants/<Type>/<ID>?currency=XYZ should be used, where XYZ is the requested currency.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**_type** | **String** |  | [required] |
**ID** | **String** |  | [required] |
**content_type** | **String** | The Content-Type header indicates the specific version of the API used to send the payload body. | [required] |
**date** | **String** | The Date header field indicates the date when the request was sent. | [required] |
**fspiop_source** | **String** | The FSPIOP-Source header field is a non-HTTP standard field used by the API for identifying the sender of the HTTP request. The field should be set by the original sender of the request. Required for routing and signature verification (see header field FSPIOP-Signature). | [required] |
**accept** | **String** | The Accept header field indicates the version of the API the client would like the server to use. | [required] |
**x_forwarded_for** | Option<**String**> | The X-Forwarded-For header field is an unofficially accepted standard used for informational purposes of the originating client IP address, as a request might pass multiple proxies, firewalls, and so on. Multiple X-Forwarded-For values as in the example shown here should be expected and supported by implementers of the API. Note - An alternative to X-Forwarded-For is defined in RFC 7239. However, to this point RFC 7239 is less-used and supported than X-Forwarded-For. |  |
**fspiop_destination** | Option<**String**> | The `FSPIOP-Destination` header field is a non-HTTP standard field used by the API for HTTP header-based routing of requests and responses to the destination. The field must be set by the original sender of the request if the destination is known (valid for all services except GET /parties) so that any entities between the client and the server do not need to parse the payload for routing purposes. If the destination is not known (valid for service GET /parties), the field should be left empty. |  |
**fspiop_encryption** | Option<**String**> | The FSPIOP-Encryption header field is a non-HTTP standard field used by the API for applying end-to-end encryption of the request. |  |
**fspiop_signature** | Option<**String**> | The FSPIOP-Signature header field is a non-HTTP standard field used by the API for applying an end-to-end request signature. |  |
**FSPIOP_URI** | Option<**String**> | The FSPIOP-URI header field is a non-HTTP standard field used by the API for signature verification, should contain the service URI. Required if signature verification is used, for more information see API Signature document. |  |
**fspiop_http_method** | Option<**String**> | The FSPIOP-HTTP-Method header field is a non-HTTP standard field used by the API for signature verification, should contain the service HTTP method. Required if signature verification is used, for more information see API Signature document. |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## participants_by_type_and_id2

> participants_by_type_and_id2(_type, ID, content_type, date, fspiop_source, accept, x_forwarded_for, fspiop_destination, fspiop_encryption, fspiop_signature, FSPIOP_URI, fspiop_http_method)
ParticipantsByTypeAndID

The HTTP request DELETE /participants/<Type>/<ID> (or DELETE /participants/<Type>/<ID>/<SubId>) is used to delete information in the server regarding the provided identity, defined by <Type> and <ID>) (for example, DELETE /participants/MSISDN/123456789), and optionally <SubId>. This HTTP request should support a query string to delete FSP information regarding a specific currency only. To delete a specific currency only, the HTTP request DELETE /participants/<Type>/<ID>?currency=XYZ should be used, where XYZ is the requested currency. Note -  The Account Lookup System should verify that it is the Party’s current FSP that is deleting the FSP information.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**_type** | **String** |  | [required] |
**ID** | **String** |  | [required] |
**content_type** | **String** | The Content-Type header indicates the specific version of the API used to send the payload body. | [required] |
**date** | **String** | The Date header field indicates the date when the request was sent. | [required] |
**fspiop_source** | **String** | The FSPIOP-Source header field is a non-HTTP standard field used by the API for identifying the sender of the HTTP request. The field should be set by the original sender of the request. Required for routing and signature verification (see header field FSPIOP-Signature). | [required] |
**accept** | **String** | The Accept header field indicates the version of the API the client would like the server to use. | [required] |
**x_forwarded_for** | Option<**String**> | The X-Forwarded-For header field is an unofficially accepted standard used for informational purposes of the originating client IP address, as a request might pass multiple proxies, firewalls, and so on. Multiple X-Forwarded-For values as in the example shown here should be expected and supported by implementers of the API. Note - An alternative to X-Forwarded-For is defined in RFC 7239. However, to this point RFC 7239 is less-used and supported than X-Forwarded-For. |  |
**fspiop_destination** | Option<**String**> | The `FSPIOP-Destination` header field is a non-HTTP standard field used by the API for HTTP header-based routing of requests and responses to the destination. The field must be set by the original sender of the request if the destination is known (valid for all services except GET /parties) so that any entities between the client and the server do not need to parse the payload for routing purposes. If the destination is not known (valid for service GET /parties), the field should be left empty. |  |
**fspiop_encryption** | Option<**String**> | The FSPIOP-Encryption header field is a non-HTTP standard field used by the API for applying end-to-end encryption of the request. |  |
**fspiop_signature** | Option<**String**> | The FSPIOP-Signature header field is a non-HTTP standard field used by the API for applying an end-to-end request signature. |  |
**FSPIOP_URI** | Option<**String**> | The FSPIOP-URI header field is a non-HTTP standard field used by the API for signature verification, should contain the service URI. Required if signature verification is used, for more information see API Signature document. |  |
**fspiop_http_method** | Option<**String**> | The FSPIOP-HTTP-Method header field is a non-HTTP standard field used by the API for signature verification, should contain the service HTTP method. Required if signature verification is used, for more information see API Signature document. |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## participants_by_type_and_id3

> participants_by_type_and_id3(_type, ID, content_type, date, fspiop_source, body, x_forwarded_for, fspiop_destination, fspiop_encryption, fspiop_signature, FSPIOP_URI, fspiop_http_method, content_length)
ParticipantsByTypeAndID

The callback PUT /participants/<Type>/<ID> (or PUT /participants/<Type>/<ID>/<SubId>) is used to inform the client of a successful result of the lookup, creation, or deletion of the FSP information related to the Party. If the FSP information is deleted, the fspId element should be empty; otherwise the element should include the FSP information for the Party.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**_type** | **String** |  | [required] |
**ID** | **String** |  | [required] |
**content_type** | **String** | The Content-Type header indicates the specific version of the API used to send the payload body. | [required] |
**date** | **String** | The Date header field indicates the date when the request was sent. | [required] |
**fspiop_source** | **String** | The FSPIOP-Source header field is a non-HTTP standard field used by the API for identifying the sender of the HTTP request. The field should be set by the original sender of the request. Required for routing and signature verification (see header field FSPIOP-Signature). | [required] |
**body** | [**ParticipantsTypeIdPutResponse**](ParticipantsTypeIdPutResponse.md) |  | [required] |
**x_forwarded_for** | Option<**String**> | The X-Forwarded-For header field is an unofficially accepted standard used for informational purposes of the originating client IP address, as a request might pass multiple proxies, firewalls, and so on. Multiple X-Forwarded-For values as in the example shown here should be expected and supported by implementers of the API. Note - An alternative to X-Forwarded-For is defined in RFC 7239. However, to this point RFC 7239 is less-used and supported than X-Forwarded-For. |  |
**fspiop_destination** | Option<**String**> | The `FSPIOP-Destination` header field is a non-HTTP standard field used by the API for HTTP header-based routing of requests and responses to the destination. The field must be set by the original sender of the request if the destination is known (valid for all services except GET /parties) so that any entities between the client and the server do not need to parse the payload for routing purposes. If the destination is not known (valid for service GET /parties), the field should be left empty. |  |
**fspiop_encryption** | Option<**String**> | The FSPIOP-Encryption header field is a non-HTTP standard field used by the API for applying end-to-end encryption of the request. |  |
**fspiop_signature** | Option<**String**> | The FSPIOP-Signature header field is a non-HTTP standard field used by the API for applying an end-to-end request signature. |  |
**FSPIOP_URI** | Option<**String**> | The FSPIOP-URI header field is a non-HTTP standard field used by the API for signature verification, should contain the service URI. Required if signature verification is used, for more information see API Signature document. |  |
**fspiop_http_method** | Option<**String**> | The FSPIOP-HTTP-Method header field is a non-HTTP standard field used by the API for signature verification, should contain the service HTTP method. Required if signature verification is used, for more information see API Signature document. |  |
**content_length** | Option<**i32**> | The Content-Length header field indicates the anticipated size of the payload body. Only sent if there is a body. Note - The API supports a maximum size of 5242880 bytes (5 Megabytes) |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## participants_error_by_type_and_id

> participants_error_by_type_and_id(_type, ID, content_type, date, fspiop_source, body, content_length, x_forwarded_for, fspiop_destination, fspiop_encryption, fspiop_signature, FSPIOP_URI, fspiop_http_method)
ParticipantsErrorByTypeAndID

If the server is unable to find, create or delete the associated FSP of the provided identity, or another processing error occurred, the error callback PUT /participants/<Type>/<ID>/error (or PUT /participants/<Type>/<ID>/<SubId>/error) is used.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**_type** | **String** |  | [required] |
**ID** | **String** |  | [required] |
**content_type** | **String** | The Content-Type header indicates the specific version of the API used to send the payload body. | [required] |
**date** | **String** | The Date header field indicates the date when the request was sent. | [required] |
**fspiop_source** | **String** | The FSPIOP-Source header field is a non-HTTP standard field used by the API for identifying the sender of the HTTP request. The field should be set by the original sender of the request. Required for routing and signature verification (see header field FSPIOP-Signature). | [required] |
**body** | [**ErrorInformationObject**](ErrorInformationObject.md) |  | [required] |
**content_length** | Option<**i32**> | The Content-Length header field indicates the anticipated size of the payload body. Only sent if there is a body. Note - The API supports a maximum size of 5242880 bytes (5 Megabytes) |  |
**x_forwarded_for** | Option<**String**> | The X-Forwarded-For header field is an unofficially accepted standard used for informational purposes of the originating client IP address, as a request might pass multiple proxies, firewalls, and so on. Multiple X-Forwarded-For values as in the example shown here should be expected and supported by implementers of the API. Note - An alternative to X-Forwarded-For is defined in RFC 7239. However, to this point RFC 7239 is less-used and supported than X-Forwarded-For. |  |
**fspiop_destination** | Option<**String**> | The `FSPIOP-Destination` header field is a non-HTTP standard field used by the API for HTTP header-based routing of requests and responses to the destination. The field must be set by the original sender of the request if the destination is known (valid for all services except GET /parties) so that any entities between the client and the server do not need to parse the payload for routing purposes. If the destination is not known (valid for service GET /parties), the field should be left empty. |  |
**fspiop_encryption** | Option<**String**> | The FSPIOP-Encryption header field is a non-HTTP standard field used by the API for applying end-to-end encryption of the request. |  |
**fspiop_signature** | Option<**String**> | The FSPIOP-Signature header field is a non-HTTP standard field used by the API for applying an end-to-end request signature. |  |
**FSPIOP_URI** | Option<**String**> | The FSPIOP-URI header field is a non-HTTP standard field used by the API for signature verification, should contain the service URI. Required if signature verification is used, for more information see API Signature document. |  |
**fspiop_http_method** | Option<**String**> | The FSPIOP-HTTP-Method header field is a non-HTTP standard field used by the API for signature verification, should contain the service HTTP method. Required if signature verification is used, for more information see API Signature document. |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## participants_sub_id_by_type_and_id

> participants_sub_id_by_type_and_id(_type, ID, sub_id, content_type, date, fspiop_source, accept, x_forwarded_for, fspiop_destination, fspiop_encryption, fspiop_signature, FSPIOP_URI, fspiop_http_method)
ParticipantsSubIdByTypeAndID

The HTTP request GET /participants/<Type>/<ID> (or GET /participants/<Type>/<ID>/<SubId>) is used to find out in which FSP the requested Party, defined by <Type>, <ID> and optionally <SubId>, is located (for example, GET /participants/MSISDN/123456789, or GET /participants/BUSINESS/shoecompany/employee1). This HTTP request should support a query string for filtering of currency. To use filtering of currency, the HTTP request GET /participants/<Type>/<ID>?currency=XYZ should be used, where XYZ is the requested currency.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**_type** | **String** |  | [required] |
**ID** | **String** |  | [required] |
**sub_id** | **String** |  | [required] |
**content_type** | **String** | The Content-Type header indicates the specific version of the API used to send the payload body. | [required] |
**date** | **String** | The Date header field indicates the date when the request was sent. | [required] |
**fspiop_source** | **String** | The FSPIOP-Source header field is a non-HTTP standard field used by the API for identifying the sender of the HTTP request. The field should be set by the original sender of the request. Required for routing and signature verification (see header field FSPIOP-Signature). | [required] |
**accept** | **String** | The Accept header field indicates the version of the API the client would like the server to use. | [required] |
**x_forwarded_for** | Option<**String**> | The X-Forwarded-For header field is an unofficially accepted standard used for informational purposes of the originating client IP address, as a request might pass multiple proxies, firewalls, and so on. Multiple X-Forwarded-For values as in the example shown here should be expected and supported by implementers of the API. Note - An alternative to X-Forwarded-For is defined in RFC 7239. However, to this point RFC 7239 is less-used and supported than X-Forwarded-For. |  |
**fspiop_destination** | Option<**String**> | The `FSPIOP-Destination` header field is a non-HTTP standard field used by the API for HTTP header-based routing of requests and responses to the destination. The field must be set by the original sender of the request if the destination is known (valid for all services except GET /parties) so that any entities between the client and the server do not need to parse the payload for routing purposes. If the destination is not known (valid for service GET /parties), the field should be left empty. |  |
**fspiop_encryption** | Option<**String**> | The FSPIOP-Encryption header field is a non-HTTP standard field used by the API for applying end-to-end encryption of the request. |  |
**fspiop_signature** | Option<**String**> | The FSPIOP-Signature header field is a non-HTTP standard field used by the API for applying an end-to-end request signature. |  |
**FSPIOP_URI** | Option<**String**> | The FSPIOP-URI header field is a non-HTTP standard field used by the API for signature verification, should contain the service URI. Required if signature verification is used, for more information see API Signature document. |  |
**fspiop_http_method** | Option<**String**> | The FSPIOP-HTTP-Method header field is a non-HTTP standard field used by the API for signature verification, should contain the service HTTP method. Required if signature verification is used, for more information see API Signature document. |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## participants_sub_id_by_type_and_id2

> participants_sub_id_by_type_and_id2(_type, ID, sub_id, content_type, date, fspiop_source, accept, x_forwarded_for, fspiop_destination, fspiop_encryption, fspiop_signature, FSPIOP_URI, fspiop_http_method)
ParticipantsSubIdByTypeAndID

The HTTP request DELETE /participants/<Type>/<ID> (or DELETE /participants/<Type>/<ID>/<SubId>) is used to delete information in the server regarding the provided identity, defined by <Type> and <ID>) (for example, DELETE /participants/MSISDN/123456789), and optionally <SubId>. This HTTP request should support a query string to delete FSP information regarding a specific currency only. To delete a specific currency only, the HTTP request DELETE /participants/<Type>/<ID>?currency=XYZ should be used, where XYZ is the requested currency. Note -  The Account Lookup System should verify that it is the Party’s current FSP that is deleting the FSP information.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**_type** | **String** |  | [required] |
**ID** | **String** |  | [required] |
**sub_id** | **String** |  | [required] |
**content_type** | **String** | The Content-Type header indicates the specific version of the API used to send the payload body. | [required] |
**date** | **String** | The Date header field indicates the date when the request was sent. | [required] |
**fspiop_source** | **String** | The FSPIOP-Source header field is a non-HTTP standard field used by the API for identifying the sender of the HTTP request. The field should be set by the original sender of the request. Required for routing and signature verification (see header field FSPIOP-Signature). | [required] |
**accept** | **String** | The Accept header field indicates the version of the API the client would like the server to use. | [required] |
**x_forwarded_for** | Option<**String**> | The X-Forwarded-For header field is an unofficially accepted standard used for informational purposes of the originating client IP address, as a request might pass multiple proxies, firewalls, and so on. Multiple X-Forwarded-For values as in the example shown here should be expected and supported by implementers of the API. Note - An alternative to X-Forwarded-For is defined in RFC 7239. However, to this point RFC 7239 is less-used and supported than X-Forwarded-For. |  |
**fspiop_destination** | Option<**String**> | The `FSPIOP-Destination` header field is a non-HTTP standard field used by the API for HTTP header-based routing of requests and responses to the destination. The field must be set by the original sender of the request if the destination is known (valid for all services except GET /parties) so that any entities between the client and the server do not need to parse the payload for routing purposes. If the destination is not known (valid for service GET /parties), the field should be left empty. |  |
**fspiop_encryption** | Option<**String**> | The FSPIOP-Encryption header field is a non-HTTP standard field used by the API for applying end-to-end encryption of the request. |  |
**fspiop_signature** | Option<**String**> | The FSPIOP-Signature header field is a non-HTTP standard field used by the API for applying an end-to-end request signature. |  |
**FSPIOP_URI** | Option<**String**> | The FSPIOP-URI header field is a non-HTTP standard field used by the API for signature verification, should contain the service URI. Required if signature verification is used, for more information see API Signature document. |  |
**fspiop_http_method** | Option<**String**> | The FSPIOP-HTTP-Method header field is a non-HTTP standard field used by the API for signature verification, should contain the service HTTP method. Required if signature verification is used, for more information see API Signature document. |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## participants_sub_id_by_type_and_id3

> participants_sub_id_by_type_and_id3(_type, ID, sub_id, content_type, date, fspiop_source, body, x_forwarded_for, fspiop_destination, fspiop_encryption, fspiop_signature, FSPIOP_URI, fspiop_http_method, content_length)
ParticipantsSubIdByTypeAndID

The callback PUT /participants/<Type>/<ID> (or PUT /participants/<Type>/<ID>/<SubId>) is used to inform the client of a successful result of the lookup, creation, or deletion of the FSP information related to the Party. If the FSP information is deleted, the fspId element should be empty; otherwise the element should include the FSP information for the Party.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**_type** | **String** |  | [required] |
**ID** | **String** |  | [required] |
**sub_id** | **String** |  | [required] |
**content_type** | **String** | The Content-Type header indicates the specific version of the API used to send the payload body. | [required] |
**date** | **String** | The Date header field indicates the date when the request was sent. | [required] |
**fspiop_source** | **String** | The FSPIOP-Source header field is a non-HTTP standard field used by the API for identifying the sender of the HTTP request. The field should be set by the original sender of the request. Required for routing and signature verification (see header field FSPIOP-Signature). | [required] |
**body** | [**ParticipantsTypeIdPutResponse**](ParticipantsTypeIdPutResponse.md) |  | [required] |
**x_forwarded_for** | Option<**String**> | The X-Forwarded-For header field is an unofficially accepted standard used for informational purposes of the originating client IP address, as a request might pass multiple proxies, firewalls, and so on. Multiple X-Forwarded-For values as in the example shown here should be expected and supported by implementers of the API. Note - An alternative to X-Forwarded-For is defined in RFC 7239. However, to this point RFC 7239 is less-used and supported than X-Forwarded-For. |  |
**fspiop_destination** | Option<**String**> | The `FSPIOP-Destination` header field is a non-HTTP standard field used by the API for HTTP header-based routing of requests and responses to the destination. The field must be set by the original sender of the request if the destination is known (valid for all services except GET /parties) so that any entities between the client and the server do not need to parse the payload for routing purposes. If the destination is not known (valid for service GET /parties), the field should be left empty. |  |
**fspiop_encryption** | Option<**String**> | The FSPIOP-Encryption header field is a non-HTTP standard field used by the API for applying end-to-end encryption of the request. |  |
**fspiop_signature** | Option<**String**> | The FSPIOP-Signature header field is a non-HTTP standard field used by the API for applying an end-to-end request signature. |  |
**FSPIOP_URI** | Option<**String**> | The FSPIOP-URI header field is a non-HTTP standard field used by the API for signature verification, should contain the service URI. Required if signature verification is used, for more information see API Signature document. |  |
**fspiop_http_method** | Option<**String**> | The FSPIOP-HTTP-Method header field is a non-HTTP standard field used by the API for signature verification, should contain the service HTTP method. Required if signature verification is used, for more information see API Signature document. |  |
**content_length** | Option<**i32**> | The Content-Length header field indicates the anticipated size of the payload body. Only sent if there is a body. Note - The API supports a maximum size of 5242880 bytes (5 Megabytes) |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## participants_sub_id_by_type_and_id_post

> participants_sub_id_by_type_and_id_post(_type, ID, sub_id, content_type, date, fspiop_source, accept, body, x_forwarded_for, fspiop_destination, fspiop_encryption, fspiop_signature, FSPIOP_URI, fspiop_http_method, content_length)
ParticipantsSubIdByTypeAndID

The HTTP request POST /participants/<Type>/<ID> (or POST /participants/<Type>/<ID>/<SubId>) is used to create information in the server regarding the provided identity, defined by <Type>, <ID>, and optionally <SubId> (for example, POST /participants/MSISDN/123456789 or POST /participants/BUSINESS/shoecompany/employee1). An ExtensionList element has been added to this reqeust in version v1.1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**_type** | **String** |  | [required] |
**ID** | **String** |  | [required] |
**sub_id** | **String** |  | [required] |
**content_type** | **String** | The Content-Type header indicates the specific version of the API used to send the payload body. | [required] |
**date** | **String** | The Date header field indicates the date when the request was sent. | [required] |
**fspiop_source** | **String** | The FSPIOP-Source header field is a non-HTTP standard field used by the API for identifying the sender of the HTTP request. The field should be set by the original sender of the request. Required for routing and signature verification (see header field FSPIOP-Signature). | [required] |
**accept** | **String** | The Accept header field indicates the version of the API the client would like the server to use. | [required] |
**body** | [**ParticipantsTypeIdSubIdPostRequest**](ParticipantsTypeIdSubIdPostRequest.md) |  | [required] |
**x_forwarded_for** | Option<**String**> | The X-Forwarded-For header field is an unofficially accepted standard used for informational purposes of the originating client IP address, as a request might pass multiple proxies, firewalls, and so on. Multiple X-Forwarded-For values as in the example shown here should be expected and supported by implementers of the API. Note - An alternative to X-Forwarded-For is defined in RFC 7239. However, to this point RFC 7239 is less-used and supported than X-Forwarded-For. |  |
**fspiop_destination** | Option<**String**> | The `FSPIOP-Destination` header field is a non-HTTP standard field used by the API for HTTP header-based routing of requests and responses to the destination. The field must be set by the original sender of the request if the destination is known (valid for all services except GET /parties) so that any entities between the client and the server do not need to parse the payload for routing purposes. If the destination is not known (valid for service GET /parties), the field should be left empty. |  |
**fspiop_encryption** | Option<**String**> | The FSPIOP-Encryption header field is a non-HTTP standard field used by the API for applying end-to-end encryption of the request. |  |
**fspiop_signature** | Option<**String**> | The FSPIOP-Signature header field is a non-HTTP standard field used by the API for applying an end-to-end request signature. |  |
**FSPIOP_URI** | Option<**String**> | The FSPIOP-URI header field is a non-HTTP standard field used by the API for signature verification, should contain the service URI. Required if signature verification is used, for more information see API Signature document. |  |
**fspiop_http_method** | Option<**String**> | The FSPIOP-HTTP-Method header field is a non-HTTP standard field used by the API for signature verification, should contain the service HTTP method. Required if signature verification is used, for more information see API Signature document. |  |
**content_length** | Option<**i32**> | The Content-Length header field indicates the anticipated size of the payload body. Only sent if there is a body. Note - The API supports a maximum size of 5242880 bytes (5 Megabytes) |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## participants_sub_id_error_by_type_and_id

> participants_sub_id_error_by_type_and_id(_type, ID, sub_id, content_type, date, fspiop_source, body, content_length, x_forwarded_for, fspiop_destination, fspiop_encryption, fspiop_signature, FSPIOP_URI, fspiop_http_method)
ParticipantsSubIdErrorByTypeAndID

If the server is unable to find, create or delete the associated FSP of the provided identity, or another processing error occurred, the error callback PUT /participants/<Type>/<ID>/error (or PUT /participants/<Type>/<ID>/<SubId>/error) is used.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**_type** | **String** |  | [required] |
**ID** | **String** |  | [required] |
**sub_id** | **String** |  | [required] |
**content_type** | **String** | The Content-Type header indicates the specific version of the API used to send the payload body. | [required] |
**date** | **String** | The Date header field indicates the date when the request was sent. | [required] |
**fspiop_source** | **String** | The FSPIOP-Source header field is a non-HTTP standard field used by the API for identifying the sender of the HTTP request. The field should be set by the original sender of the request. Required for routing and signature verification (see header field FSPIOP-Signature). | [required] |
**body** | [**ErrorInformationObject**](ErrorInformationObject.md) |  | [required] |
**content_length** | Option<**i32**> | The Content-Length header field indicates the anticipated size of the payload body. Only sent if there is a body. Note - The API supports a maximum size of 5242880 bytes (5 Megabytes) |  |
**x_forwarded_for** | Option<**String**> | The X-Forwarded-For header field is an unofficially accepted standard used for informational purposes of the originating client IP address, as a request might pass multiple proxies, firewalls, and so on. Multiple X-Forwarded-For values as in the example shown here should be expected and supported by implementers of the API. Note - An alternative to X-Forwarded-For is defined in RFC 7239. However, to this point RFC 7239 is less-used and supported than X-Forwarded-For. |  |
**fspiop_destination** | Option<**String**> | The `FSPIOP-Destination` header field is a non-HTTP standard field used by the API for HTTP header-based routing of requests and responses to the destination. The field must be set by the original sender of the request if the destination is known (valid for all services except GET /parties) so that any entities between the client and the server do not need to parse the payload for routing purposes. If the destination is not known (valid for service GET /parties), the field should be left empty. |  |
**fspiop_encryption** | Option<**String**> | The FSPIOP-Encryption header field is a non-HTTP standard field used by the API for applying end-to-end encryption of the request. |  |
**fspiop_signature** | Option<**String**> | The FSPIOP-Signature header field is a non-HTTP standard field used by the API for applying an end-to-end request signature. |  |
**FSPIOP_URI** | Option<**String**> | The FSPIOP-URI header field is a non-HTTP standard field used by the API for signature verification, should contain the service URI. Required if signature verification is used, for more information see API Signature document. |  |
**fspiop_http_method** | Option<**String**> | The FSPIOP-HTTP-Method header field is a non-HTTP standard field used by the API for signature verification, should contain the service HTTP method. Required if signature verification is used, for more information see API Signature document. |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

