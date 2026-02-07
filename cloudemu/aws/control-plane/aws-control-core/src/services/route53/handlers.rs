use crate::Emulator;
use crate::error::EmulatorError;
use axum::{
    extract::{State, Path},
    http::{HeaderMap, Method},
    response::{IntoResponse, Response},
    Json,
};
use serde_json::{json, Value};
use std::sync::Arc;
use axum::body::Bytes;

pub async fn handle_request(
    State(emulator): State<Arc<Emulator>>,
    method: axum::http::Method,
    path: String,
    body: Bytes
) -> Response {
    // Route53 uses REST-like API mostly with XML
    
    let result = if method == Method::POST && path.ends_with("/hostedzone") {
         create_hosted_zone(&emulator, body).await
    } else if method == Method::GET && path.ends_with("/hostedzone") {
         list_hosted_zones(&emulator).await
    } else if method == Method::POST && path.contains("/rrset") {
         // Path like /2013-04-01/hostedzone/{Id}/rrset
         let parts: Vec<&str> = path.split('/').collect();
         if let Some(pos) = parts.iter().position(|&p| p == "hostedzone") {
             if let Some(zone_id) = parts.get(pos+1) {
                  // reconstruct full zone id if needed, but storage takes simple ID usually or /hostedzone/ID
                  // Our storage create uses /hostedzone/Z... format.
                  let full_zone_id = format!("/hostedzone/{}", zone_id); 
                  change_resource_record_sets(&emulator, &full_zone_id, body).await
             } else {
                  Err(EmulatorError::InvalidArgument("Invalid zone id".into()))
             }
         } else {
              Err(EmulatorError::InvalidArgument("Invalid path for rrset".into()))
         }
    } else if method == Method::GET && path.contains("/rrset") {
        let parts: Vec<&str> = path.split('/').collect();
        if let Some(pos) = parts.iter().position(|&p| p == "hostedzone") {
             if let Some(zone_id) = parts.get(pos+1) {
                  let full_zone_id = format!("/hostedzone/{}", zone_id);
                  list_resource_record_sets(&emulator, &full_zone_id).await
             } else {
                  Err(EmulatorError::InvalidArgument("Invalid zone id".into()))
             }
        } else {
             Err(EmulatorError::InvalidArgument("Invalid path for rrset".into()))
        }
    } else {
        Err(EmulatorError::NotImplemented(format!("Route53 action: {} {}", method, path)))
    };

    match result {
        Ok(val) => (axum::http::StatusCode::OK, Json(val)).into_response(), 
        Err(e) => crate::error::ApiError(e).into_response(),
    }
}

async fn create_hosted_zone(emulator: &Emulator, _body: Bytes) -> Result<Value, EmulatorError> {
    // Mock parsing XML body
    let name = "example.com"; 
    let caller_ref = uuid::Uuid::new_v4().to_string();

    let zone = emulator.storage.create_hosted_zone(name, &caller_ref)?;
    
    Ok(json!({
        "CreateHostedZoneResponse": {
            "HostedZone": {
                "Id": zone.id,
                "Name": zone.name,
                "CallerReference": zone.caller_reference,
                "Config": {
                     "PrivateZone": zone.private_zone
                },
                "ResourceRecordSetCount": 2
            },
            "ChangeInfo": {
                "Id": "change-1",
                "Status": "PENDING",
                "SubmittedAt": "2023-01-01T00:00:00Z"
            },
             "DelegationSet": {
                "NameServers": [
                    "ns-01.awsdns-01.com",
                    "ns-02.awsdns-02.com"
                ]
            }
        }
    }))
}

async fn list_hosted_zones(emulator: &Emulator) -> Result<Value, EmulatorError> {
    let zones = emulator.storage.list_hosted_zones()?;
    
    let zones_json: Vec<Value> = zones.into_iter().map(|z| {
        json!({
            "Id": z.id,
            "Name": z.name,
            "CallerReference": z.caller_reference,
            "Config": { "PrivateZone": z.private_zone },
            "ResourceRecordSetCount": 2
        })
    }).collect();

    Ok(json!({
        "ListHostedZonesResponse": {
            "HostedZones": zones_json,
            "IsTruncated": false,
            "MaxItems": 100
        }
    }))
}

async fn change_resource_record_sets(emulator: &Emulator, zone_id: &str, _body: Bytes) -> Result<Value, EmulatorError> {
    // Mock XML parsing of ChangeBatch
    // We'll insert a dummy record for now to prove it works
    use aws_data_core::{ResourceRecordSet, ResourceRecord};
    
    let records = vec![
        ResourceRecordSet {
            name: "www.example.com".to_string(),
            r#type: "A".to_string(),
            ttl: 300,
            resource_records: vec![ResourceRecord { value: "1.2.3.4".to_string() }],
        }
    ];

    emulator.storage.change_resource_record_sets(zone_id, records)?;

    Ok(json!({
        "ChangeResourceRecordSetsResponse": {
            "ChangeInfo": {
                "Id": "change-2",
                "Status": "INSYNC",
                "SubmittedAt": "2023-01-01T00:00:00Z"
            }
        }
    }))
}

async fn list_resource_record_sets(emulator: &Emulator, zone_id: &str) -> Result<Value, EmulatorError> {
    let records = emulator.storage.list_resource_record_sets(zone_id)?;

    let records_json: Vec<Value> = records.into_iter().map(|r| {
        json!({
            "Name": r.name,
            "Type": r.r#type,
            "TTL": r.ttl,
            "ResourceRecords": r.resource_records.into_iter().map(|rr| json!({"Value": rr.value})).collect::<Vec<_>>()
        })
    }).collect();

    Ok(json!({
        "ListResourceRecordSetsResponse": {
            "ResourceRecordSets": records_json,
            "IsTruncated": false,
            "MaxItems": 100
        }
    }))
}
