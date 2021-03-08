use serde::{Deserialize, Serialize};
use reqwest::header::AUTHORIZATION;
use mysql::Params;
use mysql::prelude::Queryable;

use crate::database::Database;
use crate::types::wix::{BuyerInfo, WeightUnit, Totals, PaymentStatus, FulfilmentStatus, IdentityType};

const WIX_QUERY_ORDERS_ENDPOINT: &str = "https://www.wixapis.com/stores/v2/orders/query";

#[derive(Serialize)]
struct QueryOrdersRequest {
    query: Query
}

#[derive(Serialize)]
struct Query {
    paging: Paging
}

#[derive(Serialize)]
struct Paging {
    limit:  u8,
    offset: u64
}

#[derive(Deserialize)]
#[serde(rename_all = "lowerCamelCase")]
struct QueryOrdersResponse {
    /// Order data
    orders:         Vec<Order>,

    /// Paging metadata
    metadata:       Metadata,

    /// Total results
    total_results:  u64
}

/// Order data
#[derive(Deserialize)]
struct Order {
    /// Order ID (auto-generated upon order creation).
    id: String,

    /// Order number displayed in the owner's store (auto-generated).
    number: i64,

    /// Order creation date and time.
    #[serde(rename(deserialize = "dateCreated"))]
    date_created: String,

    /// Buyer information.
    #[serde(rename(deserialize = "buyerInfo"))]
    buyer_info: BuyerInfo,

    /// Currency used for pricing in this store.
    currency: String,

    /// Weight unit used in this store.
    #[serde(rename(deserialize = "weightUnit"))]
    weight_unit: WeightUnit,

    /// Totals for order's line items.
    totals: Totals,

    /// Billing information.
    #[serde(rename(deserialize = "billingInfo"))]
    billing_info: BillingInfo,

    /// Shipping information.
    #[serde(rename(deserialize = "shippingInfo"))]
    shipping_info: ShippingInfo,

    /// A note added by the buyer.
    #[serde(rename(deserialize = "buyerNote"))]
    buyer_note: String,

    /// Current status of the payment.
    #[serde(rename(deserialize = "paymentStatus"))]
    payment_status: PaymentStatus,

    /// Order's current fulfillment status (whether the order received a tracking number or was delivered/picked up).
    #[serde(rename(deserialize = "fulfillmentStatus"))]
    fulfillment_status: FulfilmentStatus,

    /// Line items ordered.
    #[serde(rename(deserialize = "lineItems"))]
    line_items: Vec<LineItem>,

    /// Log of updates related to the order.
    activities: Vec<Activity>,

    /// Invoice information.
    #[serde(rename(deserialize = "invoiceInfo"))]
    invoice_info: InvoiceInfo,

    /// Order fulfillment information.
    fulfillments: Vec<Fulfillment>,

    /// Discount information.
    discount: Discount,

    /// Custom field information.
    custom_field: CustomField,

    /// Shopping cart ID.
    #[serde(rename(deserialize = "cartId"))]
    cart_id: String,

    /// Language to be used when communicating with the customer. For a site that supports multiple languages, this is the language the customer selected (otherwise this defaults to the site language).
    #[serde(rename(deserialize = "buyerLanguage"))]
    buyer_language: String,

    /// Information about the sales channel that submitted this order.
    #[serde(rename(deserialize = "channelInfo"))]
    channel_info: ChannelInfo,

    /// Identity of the order's initiator.
    #[serde(rename(deserialize = "enteredBy"))]
    entered_by: EnteredBy,

    /// Date and time of latest update.
    #[serde(rename(deserialize = "lastUpdated"))]
    last_updated: String,

    /// Order’s unique numeric ID. Primarily used for sorting and filtering when crawling all orders.
    #[serde(rename(deserialize = "numericId"))]
    numeric_id: String,

    /// Refund information
    refunds: Vec<Refund>
}

/// Line items ordered.
#[serde(rename_all = "lowerCamelCase")]
#[derive(Deserialize)]
struct LineItems {

    /// Line item ID (auto-generated, stable within this order only)
    index: i64,

    /// Line item quantity
    quantity: i64,

    /// Line item name
    name: String,

    /// Product name, translated into the customer's language
    translated_name: String,

    /// Line item product ID (optional for POS orders)
    product_id: std::Option<String>,

    /// Line item type (may be extended)
    line_item_type: LineItemType,

    /// Line item options ordered
    options: Vec<Option>,

    /// Line item custom text field entry
    custom_text_fields: Vec<CustomTextField>,

    /// Line item weight
    weight: String,

    /// Primary media for preview of the line item
    media_item: Vec<MediaItem>,

    /// Line item SKU
    sku: String,

    /// Line item notes
    notes: String,

    /// Line item variantId (from Stores Catalog)
    variant_id: String,

    /// Line item fulfillerId from stores fulfillers. No value equals self fulfilled
    fulfiller_id: std::Option<String>,

    /// Discount applied for this line item
    discount: String,

    /// Tax applied for this line item
    tax: String,

    /// Tax group ID
    tax_group_id: String,

    /// Price data
    price_data: PriceData
}

/// Price data
#[serde(rename_all = "lowerCameLCase")]
#[derive(Deserialize)]
struct PriceData {
    tax_included_in_price:  bool,
    price:                  String,
    total_price:            String
}

/// Primary media for preview of the line item
#[serde(rename_all = "lowerCameLCase")]
#[derive(Deserialize)]
struct MediaItem {
    /// Media type
    media_type:         MediaType,

    /// Media URL
    url:                String,

    /// Media item width
    width:              i64,

    /// Media item height
    height:             i64,

    /// Media ID (for media items previously saved in Wix Media)
    id:                 String,

    /// Media external URL
    external_image_url: String,

    /// Alternative text for presentation when media cannot be displayed
    alt_text:           String
}

/// Media type
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[derive(Deserialize)]
enum MediaType {
    UnspecifiedMediaTypeItem,
    Image
}

/// Line item custom text field entry
#[derive(Deserialize)]
struct CustomTextField {
    title: String,
    value: String
}

/// Line item options ordered
#[derive(Deserialize)]
struct Option {
    /// Option name
    option:     String,

    /// Selected choice for this option
    selection:  Stirng
}

/// Line item type (may be extended)
#[derive(Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum LineItemType {
    UnspecifiedLineItemType,
    Physical,
    Digital,
    CustomAmountItem
}

/// Log of updates related to the order.
#[derive(Deserialize)]
struct Activity {
    ///Activity item type
    #[serde(rename(deserialize = "type"))]
    activity_type:  ActivityType,

    /// Activity item author
    author:         String,

    /// Comment added to activity item
    message:        String,

    /// Activity item timestamp
    timestamp:      String,
}

/// Activity item type
#[derive(Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum ActivityType {
    UnspecifiedOrderHistoryItemType,
    MerchantComment,
    OrderPlaced,
    OrderPaid,
    OrderFulfilled,
    OrderNotFulfilled,
    DownloadLinkSent,
    PickupReadyEmailSent,
    TrackingNumberAdded,
    TrackingNumberEdited,
    TrackingLinkWasSet,
    ShippingConfirmationEmailWasSent,
    InvoiceWasSet,
    InvoiceWasRemoved,
    InvoiceWasSent,
    FulfillerEmailSent,
    ShippingAddressEdited,
    EmailEdited
}

/// Invoice information.
#[derive(Deserialize)]
struct InvoiceInfo {
    /// Invoice ID
    id: String,

    /// Invoice source
    source: InvoiceSource
}

/// Invoice source
#[derive(Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum InvoiceSource {
    UnspecifiedInvoiceSource,
    Wix
}

/// Order fulfillment information.
#[serde(rename_all = "lowerCamelCase")]
#[derive(Deserialize)]
struct Fulfillments {
    /// Fulfillment ID (auto generated upon fulfillment creation).
    id: String,

    /// Fulfillment creation date and time.
    data_created: String,

    /// Information about the line items in the fulfilled order.
    line_items: Vec<FulfillmentLineItems>,

    /// Tracking information.
    tracking_info: TrackingInfo,
}

/// Tracking information.
#[serde(rename_all = "lowerCamelCase")]
#[derive(Deserialize)]
struct TrackingInfo {
    /// Tracking number.
    tracking_number: String,

    /// Shipping provider.
    shipping_provider: String,

    /// Tracking link - autofilled if using a predefined shipping provider, otherwise provided on creation.
    tracking_link: String
}

/// Discount information
#[serde(rename_all = "lowerCamelCase")]
#[derive(Deserialize)]
struct Discount {
    /// Applied coupon
    applied_coupon: AppliedCoupon
}

/// Applied coupon
#[serde(rename_all = "lowerCamelCase")]
#[derive(Deserialize)]
struct AppliedCoupon {
    /// Coupon ID
    coupon_id: string,

    /// Coupon name
    name: String,

    /// Coupon code
    code: String,
}

/// Information about the line items in the fulfilled order.
#[derive(Deserialize)]
struct FulfillmentLineItems {
    /// Line item ID (mirrors the line item index of the order).
    index:      i64,

    /// Line item quantity. On creation, if this parameter isn't passed, the new fulfillment will automatically include all items of this line item that have not already been linked to a fulfillment. If the order does not have the requested quantity of line items available to add to this fulfillment, the fulfillment will not be created and an error will be returned. This property will always have a value when returned.
    quantity:   i64
}

/// Custom field information.
#[derive(Deserialize)]
#[serde(rename_all = "lowerCamelCase")]
struct CustomField {
    /// Free text that the customer entered in the custom field during the checkout process
    value: String,

    /// Title for the custom field
    title: String,

    /// The title translated according to the buyer language
    translated_title: String,
}

/// Sales channel that submitted the order
#[serde(rename_all = "lowerCamelCase")]
#[derive(Deserialize)]
struct ChannelInfo {
    /// Sales channel that submitted the order
    #[serde(rename(deserialize = "type"))]
    channel_info_type: ChannelInfoType,

    /// Reference to an order ID from an external system, as defined in channelInfo (e.g., eBay or Amazon)
    external_order_id: String,

    /// URL to the order in the external system, as defined in channelInfo (e.g., eBay or Amazon)
    external_order_url: String
}

/// Sales channel that submitted the order
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[derive(Deserialize)]
enum ChannelInfoType {
    Unspecified,
    Web,
    Pos,
    Ebay,
    Amazon,
    OtherPlatform,
    WixAppStore
}

/// Identity of the order's initiator.
#[serde(rename_all = "lowerCamelCase")]
#[derive(Deserialize)]
struct EnteredBy {
    id: String,
    identity_type: IdentityType
}

/// Refund information
#[derive(Deserialize)]
#[serde(rename_all = "lowerCamelCase")]
struct Refund {
    /// Refund created timestamp.
    date_created:                       String,

    /// Refund amount.
    amount:                             String,

    /// Reason for refund, given by user (optional).
    reason:                             std::Option<String>,

    /// Payment provider transaction ID. Used to find refund transaction info on the payment provider's side.
    payment_provider_transaction_id:    String,

    /// Refund ID.
    id:                                 String,

    /// Whether refund was made externally (on the payment provider's side).
    external_refund:                    bool
}

/// Paging metadata
#[derive(Deserialize)]
struct Metadata {
    /// Requested number of items to load
    items:      i64,

    /// Requested offset since the beginning of the collection
    offset:     i64,
}

pub fn fetch_orders(database: Database, instance_id: String) {
    std::thread::spawn(move || {
        //First we have to get an access token;
        let access_token = crate::auth::wix_access_token::get_access_token(database.clone(), instance_id.clone());

        if access_token.is_err() {
            eprintln!("An error occurred while fetching orders for instance {}", instance_id);
            return;
        }

        //Create a database connection
        let mut conn = database.pool.get_conn().unwrap();

        //Now we need to fetch the orders from Wix
        let mut orders_received: u64 = 0;
        let mut order_ids: Vec<String> = Vec::new();

        while orders_received % 100 == 0 {

            let query = QueryOrdersRequest {
                query: Query {
                    paging: Paging {
                        limit: 100,
                        offset: orders_received
                    }
                }
            };

            let query_orders_request = reqwest::blocking::Client::new().post(WIX_QUERY_ORDERS_ENDPOINT)
                .body(serde_json::to_string(&query).unwrap())
                .header(AUTHORIZATION, access_token.clone())
                .send();

            if query_orders_request.is_err() {
                eprintln!("Something went wrong querying orders from Wix for instance {}", instance_id);
                continue;
            }

            let query_order_response: QueryOrdersResponse = query_orders_request.unwrap().json().unwrap();
            orders_received += query_order_response.total_results;


            //Write this order to the database
            //and loose my sanity in the progress ._.
            /*conn.exec::<usize, &str, Params>("", params!{

            })*/
            //TODO
        }
    });
}