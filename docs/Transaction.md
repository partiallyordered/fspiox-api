# Transaction

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**transaction_id** | [***::models::CorrelationId**](CorrelationId.md) | ID of the transaction, the ID is decided by the Payer FSP during the creation of the quote. | [default to null]
**quote_id** | [***::models::CorrelationId**](CorrelationId.md) | ID of the quote, the ID is decided by the Payer FSP during the creation of the quote. | [default to null]
**payee** | [***::models::Party**](Party.md) | Information about the Payee in the proposed financial transaction. | [default to null]
**payer** | [***::models::Party**](Party.md) | Information about the Payer in the proposed financial transaction. | [default to null]
**amount** | [***::models::Money**](Money.md) | Transaction amount to be sent. | [default to null]
**transaction_type** | [***::models::TransactionType**](TransactionType.md) | Type of the transaction. | [default to null]
**note** | [***::models::Note**](Note.md) | Memo associated to the transaction, intended to the Payee. | [optional] [default to null]
**extension_list** | [***::models::ExtensionList**](ExtensionList.md) | Optional extension, specific to deployment. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


