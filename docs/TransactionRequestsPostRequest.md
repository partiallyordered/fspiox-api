# TransactionRequestsPostRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**transaction_request_id** | **String** | Identifier that correlates all messages of the same sequence. The API data type UUID (Universally Unique Identifier) is a JSON String in canonical format, conforming to RFC 4122, that is restricted by a regular expression for interoperability reasons. An UUID is always 36 characters long, 32 hexadecimal symbols and 4 dashes (‘-‘). | 
**payee** | [**crate::models::Party**](Party.md) |  | 
**payer** | [**crate::models::PartyIdInfo**](PartyIdInfo.md) |  | 
**amount** | [**crate::models::Money**](Money.md) |  | 
**transaction_type** | [**crate::models::TransactionType**](TransactionType.md) |  | 
**note** | Option<**String**> | Memo assigned to transaction | [optional]
**geo_code** | Option<[**crate::models::GeoCode**](GeoCode.md)> |  | [optional]
**authentication_type** | Option<[**crate::models::AuthenticationType**](AuthenticationType.md)> |  | [optional]
**expiration** | Option<**String**> | The API data type DateTime is a JSON String in a lexical format that is restricted by a regular expression for interoperability reasons. The format is according to ISO 8601, expressed in a combined date, time and time zone format. A more readable version of the format is yyyy-MM-ddTHH:mm:ss.SSS[-HH:MM]. Examples -  \"2016-05-24T08:38:08.699-04:00\", \"2016-05-24T08:38:08.699Z\" (where Z indicates Zulu time zone, same as UTC). | [optional]
**extension_list** | Option<[**crate::models::ExtensionList**](ExtensionList.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


