# WinBackOfferCreateRequestDataAttributes

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**reference_name** | **String** |  | 
**offer_id** | **String** |  | 
**duration** | [**models::SubscriptionOfferDuration**](SubscriptionOfferDuration.md) |  | 
**offer_mode** | [**models::SubscriptionOfferMode**](SubscriptionOfferMode.md) |  | 
**period_count** | **i32** |  | 
**customer_eligibility_paid_subscription_duration_in_months** | **i32** |  | 
**customer_eligibility_time_since_last_subscribed_in_months** | [**models::IntegerRange**](IntegerRange.md) |  | 
**customer_eligibility_wait_between_offers_in_months** | Option<**i32**> |  | [optional]
**start_date** | [**String**](string.md) |  | 
**end_date** | Option<[**String**](string.md)> |  | [optional]
**priority** | **String** |  | 
**promotion_intent** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


