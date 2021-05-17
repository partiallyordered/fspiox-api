# IndividualTransfer

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**transfer_id** | **String** | Identifier that correlates all messages of the same sequence. The API data type UUID (Universally Unique Identifier) is a JSON String in canonical format, conforming to RFC 4122, that is restricted by a regular expression for interoperability reasons. An UUID is always 36 characters long, 32 hexadecimal symbols and 4 dashes (‘-‘). | 
**transfer_amount** | [**crate::models::Money**](Money.md) |  | 
**ilp_packet** | **String** | Information for recipient (transport layer information). | 
**condition** | **String** | Condition that must be attached to the transfer by the Payer. | 
**extension_list** | Option<[**crate::models::ExtensionList**](ExtensionList.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


