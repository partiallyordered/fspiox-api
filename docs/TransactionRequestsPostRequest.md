# TransactionRequestsPostRequest

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**transaction_request_id** | [***::models::CorrelationId**](CorrelationId.md) | Common ID between the FSPs for the transaction request object, decided by the Payee FSP. The ID should be reused for resends of the same transaction request. A new ID should be generated for each new transaction request. | [default to null]
**payee** | [***::models::Party**](Party.md) | Information about the Payee in the proposed financial transaction. | [default to null]
**payer** | [***::models::PartyIdInfo**](PartyIdInfo.md) | Information about the Payer type, id, sub-type/id, FSP Id in the proposed financial transaction. | [default to null]
**amount** | [***::models::Money**](Money.md) | Requested amount to be transferred from the Payer to Payee. | [default to null]
**transaction_type** | [***::models::TransactionType**](TransactionType.md) | Type of transaction. | [default to null]
**note** | [***::models::Note**](Note.md) | Reason for the transaction request, intended to the Payer. | [optional] [default to null]
**geo_code** | [***::models::GeoCode**](GeoCode.md) | Longitude and Latitude of the initiating Party. Can be used to detect fraud. | [optional] [default to null]
**authentication_type** | [***::models::AuthenticationType**](AuthenticationType.md) | OTP or QR Code, otherwise empty. | [optional] [default to null]
**expiration** | **String** | Can be set to get a quick failure in case the peer FSP takes too long to respond. Also, it may be beneficial for Consumer, Agent, Merchant to know that their request has a time limit. | [optional] [default to null]
**extension_list** | [***::models::ExtensionList**](ExtensionList.md) | Optional extension, specific to deployment. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


