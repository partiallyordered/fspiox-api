# \QuotesApi

All URIs are relative to *http://localhost/fsp*

Method | HTTP request | Description
------------- | ------------- | -------------
[**quotes**](QuotesApi.md#quotes) | **post** /quotes | Quotes
[**quotes_by_id**](QuotesApi.md#quotes_by_id) | **get** /quotes/{ID} | QuotesByID
[**quotes_by_id1**](QuotesApi.md#quotes_by_id1) | **put** /quotes/{ID} | QuotesByID
[**quotes_by_id_and_error**](QuotesApi.md#quotes_by_id_and_error) | **put** /quotes/{ID}/error | QuotesByIDAndError



## quotes

> quotes(accept, content_type, date, fspiop_source, body, content_length, x_forwarded_for, fspiop_destination, fspiop_encryption, fspiop_signature, FSPIOP_URI, fspiop_http_method)
Quotes

The HTTP request POST /quotes is used to request the creation of a quote for the provided financial transaction in the server.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**accept** | **String** | The Accept header field indicates the version of the API the client would like the server to use. | [required] |
**content_type** | **String** | The Content-Type header indicates the specific version of the API used to send the payload body. | [required] |
**date** | **String** | The Date header field indicates the date when the request was sent. | [required] |
**fspiop_source** | **String** | The FSPIOP-Source header field is a non-HTTP standard field used by the API for identifying the sender of the HTTP request. The field should be set by the original sender of the request. Required for routing and signature verification (see header field FSPIOP-Signature). | [required] |
**body** | [**QuotesPostRequest**](QuotesPostRequest.md) |  | [required] |
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


## quotes_by_id

> quotes_by_id(ID, content_type, date, fspiop_source, accept, x_forwarded_for, fspiop_destination, fspiop_encryption, fspiop_signature, FSPIOP_URI, fspiop_http_method)
QuotesByID

The HTTP request GET /quotes/<ID> is used to get information regarding an earlier created or requested quote. The <ID> in the URI should contain the quoteId that was used for the creation of the quote.

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


## quotes_by_id1

> quotes_by_id1(ID, content_type, date, fspiop_source, body, x_forwarded_for, fspiop_destination, fspiop_encryption, fspiop_signature, FSPIOP_URI, fspiop_http_method, content_length)
QuotesByID

The callback PUT /quotes/<ID> is used to inform the client of a requested or created quote. The <ID> in the URI should contain the quoteId that was used for the creation of the quote, or the <ID> that was used in the GET /quotes/<ID>GET /quotes/<ID>.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ID** | **String** |  | [required] |
**content_type** | **String** | The Content-Type header indicates the specific version of the API used to send the payload body. | [required] |
**date** | **String** | The Date header field indicates the date when the request was sent. | [required] |
**fspiop_source** | **String** | The FSPIOP-Source header field is a non-HTTP standard field used by the API for identifying the sender of the HTTP request. The field should be set by the original sender of the request. Required for routing and signature verification (see header field FSPIOP-Signature). | [required] |
**body** | [**QuotesIdPutResponse**](QuotesIdPutResponse.md) |  | [required] |
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


## quotes_by_id_and_error

> quotes_by_id_and_error(ID, content_type, date, fspiop_source, body, content_length, x_forwarded_for, fspiop_destination, fspiop_encryption, fspiop_signature, FSPIOP_URI, fspiop_http_method)
QuotesByIDAndError

If the server is unable to find or create a quote, or some other processing error occurs, the error callback PUT /quotes/<ID>/error is used. The <ID> in the URI should contain the quoteId that was used for the creation of the quote, or the <ID> that was used in the GET /quotes/<ID>.

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

