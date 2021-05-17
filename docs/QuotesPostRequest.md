# QuotesPostRequest

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**quote_id** | [***::models::CorrelationId**](CorrelationId.md) | Common ID between the FSPs for the quote object, decided by the Payer FSP. The ID should be reused for resends of the same quote for a transaction. A new ID should be generated for each new quote for a transaction. | [default to null]
**transaction_id** | [***::models::CorrelationId**](CorrelationId.md) | Common ID (decided by the Payer FSP) between the FSPs for the future transaction object. The actual transaction will be created as part of a successful transfer process. The ID should be reused for resends of the same quote for a transaction. A new ID should be generated for each new quote for a transaction. | [default to null]
**transaction_request_id** | [***::models::CorrelationId**](CorrelationId.md) | Identifies an optional previously-sent transaction request. | [optional] [default to null]
**payee** | [***::models::Party**](Party.md) | Information about the Payee in the proposed financial transaction. | [default to null]
**payer** | [***::models::Party**](Party.md) | Information about the Payer in the proposed financial transaction. | [default to null]
**amount_type** | [***::models::AmountType**](AmountType.md) | SEND for send amount, RECEIVE for receive amount. | [default to null]
**amount** | [***::models::Money**](Money.md) | Depending on amountType. If SEND - The amount the Payer would like to send, that is, the amount that should be withdrawn from the Payer account including any fees. The amount is updated by each participating entity in the transaction. If RECEIVE - The amount the Payee should receive, that is, the amount that should be sent to the receiver exclusive any fees. The amount is not updated by any of the participating entities. | [default to null]
**fees** | [***::models::Money**](Money.md) | The fees in the transaction. The fees element should be empty if fees should be non-disclosed. The fees element should be non-empty if fees should be disclosed. | [optional] [default to null]
**transaction_type** | [***::models::TransactionType**](TransactionType.md) | Type of transaction for which the quote is requested. | [default to null]
**geo_code** | [***::models::GeoCode**](GeoCode.md) | Longitude and Latitude of the initiating Party. Can be used to detect fraud. | [optional] [default to null]
**note** | [***::models::Note**](Note.md) | A memo that will be attached to the transaction. | [optional] [default to null]
**expiration** | **String** | Expiration is optional. It can be set to get a quick failure in case the peer FSP takes too long to respond. Also, it may be beneficial for Consumer, Agent, and Merchant to know that their request has a time limit. | [optional] [default to null]
**extension_list** | [***::models::ExtensionList**](ExtensionList.md) | Optional extension, specific to deployment. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


