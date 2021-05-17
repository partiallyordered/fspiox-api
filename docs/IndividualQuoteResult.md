# IndividualQuoteResult

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**quote_id** | **String** | Identifier that correlates all messages of the same sequence. The API data type UUID (Universally Unique Identifier) is a JSON String in canonical format, conforming to RFC 4122, that is restricted by a regular expression for interoperability reasons. An UUID is always 36 characters long, 32 hexadecimal symbols and 4 dashes (‘-‘). | 
**payee** | Option<[**crate::models::Party**](Party.md)> |  | [optional]
**transfer_amount** | Option<[**crate::models::Money**](Money.md)> |  | [optional]
**payee_receive_amount** | Option<[**crate::models::Money**](Money.md)> |  | [optional]
**payee_fsp_fee** | Option<[**crate::models::Money**](Money.md)> |  | [optional]
**payee_fsp_commission** | Option<[**crate::models::Money**](Money.md)> |  | [optional]
**ilp_packet** | Option<**String**> | Information for recipient (transport layer information). | [optional]
**condition** | Option<**String**> | Condition that must be attached to the transfer by the Payer. | [optional]
**error_information** | Option<[**crate::models::ErrorInformation**](ErrorInformation.md)> |  | [optional]
**extension_list** | Option<[**crate::models::ExtensionList**](ExtensionList.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


