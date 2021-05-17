# IndividualQuote

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**quote_id** | [***::models::CorrelationId**](CorrelationId.md) | Identifies quote message. | [default to null]
**transaction_id** | [***::models::CorrelationId**](CorrelationId.md) | Identifies transaction message. | [default to null]
**payee** | [***::models::Party**](Party.md) | Information about the Payee in the proposed financial transaction. | [default to null]
**amount_type** | [***::models::AmountType**](AmountType.md) | SEND for sendAmount, RECEIVE for receiveAmount. | [default to null]
**amount** | [***::models::Money**](Money.md) | Depending on amountType - If SEND - The amount the Payer would like to send, that is, the amount that should be withdrawn from the Payer account including any fees. The amount is updated by each participating entity in the transaction. If RECEIVE - The amount the Payee should receive, that is, the amount that should be sent to the receiver exclusive any fees. The amount is not updated by any of the participating entities. | [default to null]
**fees** | [***::models::Money**](Money.md) | The fees in the transaction. The fees element should be empty if fees should be non-disclosed. The fees element should be non-empty if fees should be disclosed. | [optional] [default to null]
**transaction_type** | [***::models::TransactionType**](TransactionType.md) | Type of transaction that the quote is requested for. | [default to null]
**note** | [***::models::Note**](Note.md) | Memo that will be attached to the transaction. | [optional] [default to null]
**extension_list** | [***::models::ExtensionList**](ExtensionList.md) | Optional extension, specific to deployment. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


