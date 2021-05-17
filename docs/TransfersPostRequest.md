# TransfersPostRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**transfer_id** | **String** | Identifier that correlates all messages of the same sequence. The API data type UUID (Universally Unique Identifier) is a JSON String in canonical format, conforming to RFC 4122, that is restricted by a regular expression for interoperability reasons. An UUID is always 36 characters long, 32 hexadecimal symbols and 4 dashes (‘-‘). | 
**payee_fsp** | **String** | FSP identifier. | 
**payer_fsp** | **String** | FSP identifier. | 
**amount** | [**crate::models::Money**](Money.md) |  | 
**ilp_packet** | **String** | Information for recipient (transport layer information). | 
**condition** | **String** | Condition that must be attached to the transfer by the Payer. | 
**expiration** | **String** | The API data type DateTime is a JSON String in a lexical format that is restricted by a regular expression for interoperability reasons. The format is according to ISO 8601, expressed in a combined date, time and time zone format. A more readable version of the format is yyyy-MM-ddTHH:mm:ss.SSS[-HH:MM]. Examples -  \"2016-05-24T08:38:08.699-04:00\", \"2016-05-24T08:38:08.699Z\" (where Z indicates Zulu time zone, same as UTC). | 
**extension_list** | Option<[**crate::models::ExtensionList**](ExtensionList.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


