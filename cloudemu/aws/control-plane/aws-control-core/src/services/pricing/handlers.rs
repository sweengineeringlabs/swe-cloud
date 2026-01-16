use crate::Emulator;
use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use serde_json::Value;
use std::sync::Arc;
use tracing::info;

pub async fn handle_request(
    State(_emulator): State<Arc<Emulator>>,
    headers: HeaderMap,
    Json(_body): Json<Value>,
) -> Response {
    let target = headers
        .get("x-amz-target")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("");

    info!("Pricing request: {}", target);

    let op = target.split('.').last().unwrap_or("");


    // Seed data on first access (simplification)
    let _ = _emulator.storage.seed_pricing_data().await;

    match op {
        "GetServices" => {
            // Mock response
            Json(serde_json::json!({
                "Services": [
                    {
                        "ServiceCode": "AmazonEC2",
                        "AttributeNames": ["Location", "InstanceType", "Operating System"]
                    },
                    {
                        "ServiceCode": "AmazonS3",
                        "AttributeNames": ["Location", "StorageClass", "VolumeType"]
                    }
                ],
                "FormatVersion": "aws_v1",
                "NextToken": null
            })).into_response()
        },
         "GetAttributeValues" => {
             Json(serde_json::json!({
                 "AttributeValues": [
                     { "Value": "US East (N. Virginia)" },
                     { "Value": "US West (Oregon)" }
                 ],
                 "NextToken": null
             })).into_response()
         },
         "GetProducts" => {
             let service_code = _body["ServiceCode"].as_str().unwrap_or("AmazonEC2");
             let filters = _body["Filters"].as_array();
             
             let products = _emulator.storage.get_products(service_code, |p: &aws_data_core::Product| {
                 if let Some(filters) = filters {
                     for filter in filters {
                         let field = filter["Field"].as_str().unwrap_or("");
                         let value = filter["Value"].as_str().unwrap_or("");
                         // Type "TERM_MATCH" is default
                         
                         // Check attributes
                         if let Some(attr_val) = p.attributes.get(field) {
                            if attr_val.as_str().unwrap_or("") != value {
                                return false;
                            }
                         } else {
                             // If filter field not in attributes, skip? or strict?
                             // AWS Usually strict.
                             if field == "ServiceCode" && p.service_code != value {
                                 return false;
                             }
                             // Simple check for now
                         }
                     }
                 }
                 true
             }).await.unwrap_or_default();

             let price_list: Vec<String> = products.into_iter().map(|(p, terms)| {
                let mut product_json = serde_json::to_value(p).unwrap();
                let terms_json = serde_json::json!({
                    "OnDemand": terms.into_iter().map(|t| (t.id.clone(), t)).collect::<std::collections::HashMap<String, aws_data_core::OfferTerm>>()
                });
                
                // transform to AWS structure which is weird: { product: {...}, terms: {...} }
                serde_json::json!({
                    "product": product_json,
                    "terms": terms_json,
                    "serviceCode": service_code,
                    "version": "20240101",
                    "publicationDate": "2024-01-01T00:00:00Z"
                }).to_string()
             }).collect();

             Json(serde_json::json!({
                 "PriceList": price_list,
                 "FormatVersion": "aws_v1",
                 "NextToken": null
             })).into_response()
         },
        _ => {
            info!("Pricing operation not implemented: {}", op);
            (StatusCode::NOT_IMPLEMENTED, "Not Implemented").into_response()
        }
    }
}
