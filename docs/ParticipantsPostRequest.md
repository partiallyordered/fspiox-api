# ParticipantsPostRequest

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**request_id** | [***::models::CorrelationId**](CorrelationId.md) | The ID of the request, decided by the client. Used for identification of the callback from the server. | [default to null]
**party_list** | [**Vec<::models::PartyIdInfo>**](PartyIdInfo.md) | List of PartyIdInfo elements that the client would like to update or create FSP information about. | [default to null]
**currency** | [***::models::Currency**](Currency.md) | Indicate that the provided Currency is supported by each PartyIdInfo in the list. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


