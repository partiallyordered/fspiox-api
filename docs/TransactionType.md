# TransactionType

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**scenario** | [***::models::TransactionScenario**](TransactionScenario.md) | Deposit, withdrawal, refund, … | [default to null]
**sub_scenario** | [***::models::TransactionSubScenario**](TransactionSubScenario.md) | Possible sub-scenario, defined locally within the scheme. | [optional] [default to null]
**initiator** | [***::models::TransactionInitiator**](TransactionInitiator.md) | Who is initiating the transaction - Payer or Payee | [default to null]
**initiator_type** | [***::models::TransactionInitiatorType**](TransactionInitiatorType.md) | Consumer, agent, business, … | [default to null]
**refund_info** | [***::models::Refund**](Refund.md) | Extra information specific to a refund scenario. Should only be populated if scenario is REFUND | [optional] [default to null]
**balance_of_payments** | [***::models::BalanceOfPayments**](BalanceOfPayments.md) | Balance of Payments code. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


