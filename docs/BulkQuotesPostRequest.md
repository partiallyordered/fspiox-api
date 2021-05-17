# BulkQuotesPostRequest

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**bulk_quote_id** | [***::models::CorrelationId**](CorrelationId.md) | Common ID between the FSPs for the bulk quote object, decided by the Payer FSP. The ID should be reused for resends of the same bulk quote. A new ID should be generated for each new bulk quote. | [default to null]
**payer** | [***::models::Party**](Party.md) | Information about the Payer in the proposed financial transaction. | [default to null]
**geo_code** | [***::models::GeoCode**](GeoCode.md) | Longitude and Latitude of the initiating Party. Can be used to detect fraud. | [optional] [default to null]
**expiration** | **String** | Expiration is optional to let the Payee FSP know when a quote no longer needs to be returned. | [optional] [default to null]
**individual_quotes** | [**Vec<::models::IndividualQuote>**](IndividualQuote.md) | List of quotes elements. | [default to null]
**extension_list** | [***::models::ExtensionList**](ExtensionList.md) | Optional extension, specific to deployment. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


