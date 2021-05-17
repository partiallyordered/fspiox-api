# \AuthorizationsApi

All URIs are relative to *http://localhost/fsp*

Method | HTTP request | Description
------------- | ------------- | -------------
[**authorizations_by_id_and_error**](AuthorizationsApi.md#authorizations_by_id_and_error) | **put** /authorizations/{ID}/error | AuthorizationsByIDAndError
[**authorizations_by_id_get**](AuthorizationsApi.md#authorizations_by_id_get) | **get** /authorizations/{ID} | AuthorizationsByID
[**authorizations_by_id_put**](AuthorizationsApi.md#authorizations_by_id_put) | **put** /authorizations/{ID} | AuthorizationsByID



## authorizations_by_id_and_error

> authorizations_by_id_and_error(ID, content_type, date, fspiop_source, body, content_length, x_forwarded_for, fspiop_destination, fspiop_encryption, fspiop_signature, FSPIOP_URI, fspiop_http_method)
AuthorizationsByIDAndError

If the server is unable to find the transaction request, or another processing error occurs, the error callback PUT /authorizations/<ID>/error is used. The <ID> in the URI should contain the <ID> that was used in the GET /authorizations/<ID>.

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


## authorizations_by_id_get

> authorizations_by_id_get(ID, content_type, date, fspiop_source, accept, x_forwarded_for, fspiop_destination, fspiop_encryption, fspiop_signature, FSPIOP_URI, fspiop_http_method)
AuthorizationsByID

The HTTP request GET /authorizations/<ID> is used to request the Payer to enter the applicable credentials in the Payee FSP system. The <ID> in the URI should contain the transactionRequestID, received from the POST /transactionRequests service earlier in the process. This request requires a query string to be included in the URI, with the following key-value pairs - authenticationType=<Type>, where <Type> value is a valid authentication type from the enumeration AuthenticationType. retriesLeft==<NrOfRetries>, where <NrOfRetries> is the number of retries left before the financial transaction is rejected. <NrOfRetries> must be expressed in the form of the data type Integer. retriesLeft=1 means that this is the last retry before the financial transaction is rejected. amount=<Amount>, where <Amount> is the transaction amount that will be withdrawn from the Payer’s account. <Amount> must be expressed in the form of the data type Amount. currency=<Currency>, where <Currency> is the transaction currency for the amount that will be withdrawn from the Payer’s account. The <Currency> value must be expressed in the form of the enumeration CurrencyCode. An example URI containing all the required key-value pairs in the query string is the following - GET /authorization/3d492671-b7af-4f3f-88de-76169b1bdf88?authenticationType=OTP&retriesLeft=2&amount=102&currency=USD

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
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


## authorizations_by_id_put

> authorizations_by_id_put(ID, content_type, date, fspiop_source, body, x_forwarded_for, fspiop_destination, fspiop_encryption, fspiop_signature, FSPIOP_URI, fspiop_http_method, content_length)
AuthorizationsByID

The callback PUT /authorizations/<ID> is used to inform the client of the result of a previously-requested authorization. The <ID> in the URI should contain the <ID> that was used in the GET /authorizations/<ID>.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ID** | **String** |  | [required] |
**content_type** | **String** | The Content-Type header indicates the specific version of the API used to send the payload body. | [required] |
**date** | **String** | The Date header field indicates the date when the request was sent. | [required] |
**fspiop_source** | **String** | The FSPIOP-Source header field is a non-HTTP standard field used by the API for identifying the sender of the HTTP request. The field should be set by the original sender of the request. Required for routing and signature verification (see header field FSPIOP-Signature). | [required] |
**body** | [**AuthorizationsIdPutResponse**](AuthorizationsIdPutResponse.md) |  | [required] |
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

