use serde::Deserialize;

/** Raw JWT Payload */
#[derive(Deserialize)]
pub struct OrderCreatedJwtData {
    pub data:       String,
    pub iat:        i64,
    pub exp:        i64
}

#[derive(Deserialize)]
pub struct OrderData {
    /** Whether the order was read by the store owner */
    pub read:               bool,

    /** ID displayed in the owner's store (auto generated) */
    pub number:             String,

    /** Order payment status */
    #[serde(rename(deserialize = "paymentStatus"))]
    pub payment_status:     PaymentStatus,

    /** Order ID (auto generated upon order creation) */
    #[serde(rename(deserialize = "orderId"))]
    pub order_id:           String,

    /** Order ID (auto generated upon order creation) */
    #[serde(rename(deserialize = "buyerInfo"))]
    pub buyer_info:         BuyerInfo,

    /** Currency used for pricing in this store */
    pub currency:           String,

    /** Totals for order's line items */
    pub totals:             Totals,

    /** Weight unit used in this store */
    #[serde(rename(deserialize = "weightUnit"))]
    pub weight_unit:        WeightUnit,

    /** Order fulfillment status */
    #[serde(rename(deserialize = "fulfillmentStatus"))]
    pub fulfillment_status: FulfilmentStatus
}

/** Totals for order's line items */
#[derive(Deserialize)]
pub struct Totals {
    /** Total items weight. */
    pub weight:     String,

    /** Total number of line items. */
    pub quantity:   i16,

    /** Total tax. */
    pub tax:        String,

    /** Total price charged. */
    pub total:      String,

    /** Subtotal of all the line items, before tax. */
    pub subtotal:   String,

    /** Total calculated discount value. */
    pub discount:   String,

    /** Total shipping price, before tax. */
    pub shipping:   String,

    //Omitted: refund, giftCard
}

/** Customer information */
#[derive(Deserialize)]
pub struct BuyerInfo {
    /** Customer's email address */
    pub email:          String,

    /** Customer's last name */
    #[serde(rename(deserialize = "lastName"))]
    pub last_name:      String,

    /** Customer's first name */
    #[serde(rename(deserialize = "firstName"))]
    pub first_name:     String,

    /** Wix customer ID */
    pub id:             String,

    /** Customer type */
    #[serde(rename(deserialize = "identityType"))]
    pub identity_type:  IdentityType,

    /** Customer's phone number */
    pub  phone:          String
}

/** Order payment status */
#[derive(Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PaymentStatus {
    UnspecifiedPaymentStatus,
    Paid,
    NotPaid,
    PartiallyRefunded,
    FullyRefunded,
    Pending
}

/** Order fulfillment status */
#[derive(Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FulfilmentStatus {
    Fulfilled,
    NotFulfilled,
    Canceled,
    PartiallyFulfilled
}

/** Customer type */
#[derive(Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum IdentityType {
    Contact,
    UnspecifiedIdentityType,
    Member
}

/** Weight unit used in this store */
#[derive(Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WeightUnit {
    UnspecifiedWeightUnit,
    Kg,
    Lb
}