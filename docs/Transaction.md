# Transaction

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**transaction_id** | **String** | Identifier that correlates all messages of the same sequence. The API data type UUID (Universally Unique Identifier) is a JSON String in canonical format, conforming to RFC 4122, that is restricted by a regular expression for interoperability reasons. An UUID is always 36 characters long, 32 hexadecimal symbols and 4 dashes (‘-‘). | 
**quote_id** | **String** | Identifier that correlates all messages of the same sequence. The API data type UUID (Universally Unique Identifier) is a JSON String in canonical format, conforming to RFC 4122, that is restricted by a regular expression for interoperability reasons. An UUID is always 36 characters long, 32 hexadecimal symbols and 4 dashes (‘-‘). | 
**payee** | [**crate::models::Party**](Party.md) |  | 
**payer** | [**crate::models::Party**](Party.md) |  | 
**amount** | [**crate::models::Money**](Money.md) |  | 
**transaction_type** | [**crate::models::TransactionType**](TransactionType.md) |  | 
**note** | Option<**String**> | Memo assigned to transaction | [optional]
**extension_list** | Option<[**crate::models::ExtensionList**](ExtensionList.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


