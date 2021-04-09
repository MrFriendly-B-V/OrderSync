use serde::{Deserialize, Serialize};
use reqwest::header::AUTHORIZATION;
use mysql::{Params, params};
use mysql::prelude::Queryable;
use rand::Rng;

use crate::database::Database;
use crate::types::wix::{BuyerInfo, WeightUnit, Totals, PaymentStatus, FulfilmentStatus, IdentityType};
use term::terminfo::parm::Param;

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
#[derive(Deserialize, Clone)]
#[serde(rename_all = "lowerCamelCase")]
struct Order {
    /// Order ID (auto-generated upon order creation).
    id: String,

    /// Order number displayed in the owner's store (auto-generated).
    number: i64,

    /// Order creation date and time.
    date_created: String,

    /// Buyer information.
    buyer_info: BuyerInfo,

    /// Currency used for pricing in this store.
    currency: String,

    /// Weight unit used in this store.
    weight_unit: WeightUnit,

    /// Totals for order's line items.
    totals: Totals,

    /// Billing information.
    billing_info: BillingInfo,

    /// Shipping information.
    shipping_info: ShippingInfo,

    /// A note added by the buyer.
    buyer_note: String,

    /// Current status of the payment.
    payment_status: PaymentStatus,

    /// Order's current fulfillment status (whether the order received a tracking number or was delivered/picked up).
    fulfillment_status: FulfilmentStatus,

    /// Line items ordered.
    line_items: Vec<LineItem>,

    /// Log of updates related to the order.
    activities: Vec<Activity>,

    /// Invoice information.
    invoice_info: InvoiceInfo,

    /// Order fulfillment information.
    fulfillments: Vec<Fulfillment>,

    /// Discount information.
    discount: Discount,

    /// Custom field information.
    custom_field: CustomField,

    /// Shopping cart ID.
    cart_id: String,

    /// Language to be used when communicating with the customer. For a site that supports multiple languages, this is the language the customer selected (otherwise this defaults to the site language).
    buyer_language: String,

    /// Information about the sales channel that submitted this order.
    channel_info: ChannelInfo,

    /// Identity of the order's initiator.
    entered_by: EnteredBy,

    /// Date and time of latest update.
    last_updated: String,

    /// Order’s unique numeric ID. Primarily used for sorting and filtering when crawling all orders.
    numeric_id: String,

    /// Refund information
    refunds: Vec<Refund>
}

/// Shipping information.
#[serde(rename_all = "lowerCamelCase")]
#[derive(Deserialize)]
struct ShippingInfo {
    /// Shipment details (when this object describes shipment).
    shipment_details: std::option::Option<ShipmentDetails>,

    /// Pickup details (when this object describes pickup).
    pickup_details: std::option::Option<PickupDetails>
}

/// Shipment details (when this object describes shipment).
/// Not all fields are included here!
#[derive(Deserialize)]
struct ShipmentDetails {

    /// Shipping destination address.
    address: Address
}

/// Pickup details (when this object describes pickup).
/// Not all fields are included here
#[serde(rename_all = "lowerCamelCase")]
#[derive(Deserialize)]
struct PickupDetails {
    /// Pickup address
    pickup_address: Address
}

/// Billing information.
#[serde(rename_all = "lowerCamelCase")]
#[derive(Deserialize)]
struct BillingInfo {
    /// Payment method used for this order
    payment_method: String,

    /// Transaction ID from payment provider (e.g., PayPal, Square, Stripe) transaction ID
    payment_provider_transaction_id: String,

    /// Transaction ID from payment gateway (e.g., Wix Payments)
    payment_gateway_transaction_id: String,

    /// Full billing address
    address: Address,

    /// Payment date
    paid_date: String,

    /// Whether order can be refunded by payment provider (manually or automatic)
    refundable_by_payment_provider: bool
}

///Full billing address
#[serde(rename_all = "lowerCamelCase")]
#[derive(Deserialize, Clone)]
struct Address {
    /// City name
    city: String,

    /// Email address
    email: String,

    /// Addressee name
    full_name: FullName,

    /// ZIP/postal code
    zip_code: String,

    /// Country code (2 letters)
    country: String,

    /// Company name
    company: String,

    /// address line
    address_line_2: String,

    /// Address line 1 (free text)
    address_line_1: std::option::Option<String>,

    /// Address line 1 (street)
    street: std::option::Option<Street>
}

/// Address line 1 (street)
#[serde(rename_all = "lowerCamelCase")]
#[derive(Deserialize)]
struct Street {
    /// Street number
    number: String,

    /// Street name
    name: String
}

/// Addressee name
#[serde(rename_all = "lowerCamelCase")]
#[derive(Deserialize, Clone)]
struct FullName {
    first_name: String,
    last_name: String
}

/// Line items ordered.
#[serde(rename_all = "lowerCamelCase")]
#[derive(Deserialize)]
struct LineItem {

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
struct Fulfillment {
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

            //Write these orders to the database
            //and loose my sanity in the progress ._.

            //Iterate over all received orders
            for order in query_order_response.orders {
                let order_items = order.line_items;
                let order_id: String = rand::thread_rng().sample_iter(&rand::distributions::Alphanumeric).take(64).map(char::from).collect();

                //Create an entry in the order_items table for each order
                for item in order_items {
                    let cloned_order = order.clone();
                    let item_price_data = item.price_data;

                    let order_item_id: String = rand::thread_rng().sample_iter(&rand::distributions::Alphanumeric).take(64).map(char::from).collect();

                    conn.exec::<usize, &str, Params>("INSERT INTO order_items (order_item_id, order_id, name, sku, total, price) VALUES (:order_item_id, :order_id, :name, :sku, :total, :price)", params!{
                        "order_item_id" => order_item_id.clone(),
                        "order_id" => order_id.clone(),
                        "name" => cloned_order.name,
                        "sku" => cloned_order.sku,
                        "total" => item_price_data.total_price,
                        "price" => item_price_data.price
                    });
                }

                //Next insert the billing address details
                let billing_address_id: String = rand::thread_rng().sample_iter(&rand::distributions::Alphanumeric).take(64).map(char::from).collect();
                let billing_address = order.billing_info.address;

                let street: std::option::Option<Street> = billing_address.street;
                let address_line_1 =
                    if street.is_some() {
                        let street_unw = street.unwrap();
                        format!("{} {}", street_unw.name, street_unw.number)
                    } else {
                        billing_address.address_line_1.unwrap()
                    };

                conn.exec::<usize, &str, Params>("INSERT INTO addresses (address_id, city, zip_code, country, address_line_2, address_line_!) VALUES (:address_id, :city, :zip_code, :country, :address_line_2, :address_line_1)", params! {
                    "address_id" => billing_address_id,
                    "city" => billing_address.city,
                    "zip_code" =>  billing_address.zip_code,
                    "country" => billing_address.country,
                    "address_line_2" => billing_address.address_line_2,
                    "address_line_1" => address_line_1
                });

                //Next insert the shipping address
                let shipping_address_id: String = rand::thread_rng().sample_iter(&rand::distributions::Alphanumeric).take(64).map(char::from).collect();
                let shipping_address =
                    if order.shipping_info.pickup_details.is_some() {
                        let pickup_details = order.shipping_info.pickup_details.unwrap();
                        pickup_details.pickup_address.clone()
                    } else {
                        let shipping_details = order.shipping_info.shipment_details.unwrap();
                        shipping_details.address.clone()
                    };

                let street = shipping_address.street;
                let shipping_address_line_1 =
                    if street.is_some() {
                        let street_unw = street.unwrap();
                        format!("{} {}", street_unw.name, street_unw.number)
                    } else {
                        shipping_address.address_line_1.unwrap()
                    };

                conn.exec::<usize, &str, Params>("INSERT INTO addresses (address_id, city, zip_code, country, address_line_2, address_line_!) VALUES (:address_id, :city, :zip_code, :country, :address_line_2, :address_line_1)", params! {
                    "address_id" => shipping_address_id,
                    "city" => shipping_address.city,
                    "zip_code" =>  shipping_address.zip_code,
                    "country" => shipping_address.country,
                    "address_line_2" => shipping_address.address_line_2,
                    "address_line_1" => shipping_address_line_1
                });

                let order_created_dt = chrono::DateTime::parse_from_rfc3339(&order.date_created).unwrap();
                let order_created_epoch = order_created_dt.timestamp();

                let total: f64 = order.totals.total.parse().unwrap();
                let weight: f64 = order.totals.weight.parse().unwrap();
                let quantity: i64 = order.totals.quantity.parse().unwrap();
                let subtotal: f64 = order.totals.subtotal.parse().unwrap();
                let tax: f64 = order.totals.tax.parse().unwrap();

                //Now we're going to insert the order details itself into the database
                conn.exec::<usize, &str, Params>(
                    "INSERT INTO orders \
                    (order_id, wix_order_id, order_date, currency, weight_unit, payment_status, fulfillment_status, total_price, weight, quantity, subtotal, tax, buyer_email, \
                    buyer_name, buyer_phone, billing_address_id, shipping_address_id) \
                    VALUES (:order_id, :wix_order_id, :order_date, :currency, :weight_unit, :payment_status, :fulfillment_status, :total_price, :weight, :quantity, :subtotal, \
                    :tax, :buyer_email, :buyer_name, :buyer_phone, :billing_address_id, :shipping_address_id)", params! {

                    "order_id" => order_id,
                    "wix_order_id" => order.number,
                    "order_date" => order_created_epoch,
                    "currency" => order.currency,
                    "weight_unit" => order.weight_unit.to_string(),
                    "payment_status" => order.payment_status.to_string(),
                    "fulfillment_status" => order.fulfillment_status.to_string(),
                    "total_price" => total,
                    "weight" => weight,
                    "quantity" => quantity,
                    "subtotal" => subtotal,
                    "tax" => tax,
                    "buyer_email" => order.buyer_info.email,
                    "buyer_name" => format!("{} {}", order.buyer_info.first_name, order.buyer_info.last_name),
                    "buyer_phone" =>  order.buyer_info.phone,
                    "billing_address_id" => billing_address_id,
                    "shipping_address_id" => shipping_address_id
                 });
            }
        }
    });
}