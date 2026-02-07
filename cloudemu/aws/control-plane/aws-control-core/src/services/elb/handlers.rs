use axum::{
    extract::State,
    http::HeaderMap,
    response::{IntoResponse, Response},
    Json,
};
use std::sync::Arc;
use crate::Emulator;
use serde_json::{json, Value};
use crate::error::ApiError;

pub async fn handle_request(
    State(emulator): State<Arc<Emulator>>,
    headers: HeaderMap,
    Json(body): Json<Value>,
) -> Response {
    let target = headers.get("x-amz-target").and_then(|h| h.to_str().ok()).unwrap_or("");
    let action = target.split('.').last().unwrap_or("");

    match action {
        "CreateLoadBalancer" => {
            let name = body["Name"].as_str().unwrap_or("");
            let scheme = body["Scheme"].as_str().unwrap_or("internet-facing");
            // Parsing subnets is simplified
            let subnets = vec![]; 

            match emulator.storage.create_load_balancer(name, subnets, scheme) {
                Ok(lb) => {
                    let resp = json!({
                        "CreateLoadBalancerResponse": {
                            "CreateLoadBalancerResult": {
                                "LoadBalancers": [
                                    {
                                        "LoadBalancerArn": lb.arn,
                                        "DNSName": lb.dns_name,
                                        "CanonicalHostedZoneId": "Z2P70J7EXAMPLE",
                                        "CreatedTime": lb.created_at,
                                        "LoadBalancerName": lb.name,
                                        "Scheme": lb.scheme,
                                        "VpcId": lb.vpc_id,
                                        "State": {
                                            "Code": lb.state
                                        },
                                        "Type": "application",
                                        "AvailabilityZones": []
                                    }
                                ]
                            },
                            "ResponseMetadata": {
                                "RequestId": "req-123"
                            }
                        }
                    });
                    Json(resp).into_response()
                },
                Err(e) => ApiError(e).into_response()
            }
        },
        "DescribeLoadBalancers" => {
             match emulator.storage.list_load_balancers() {
                Ok(lbs) => {
                    let lbs_json: Vec<Value> = lbs.into_iter().map(|lb| {
                        json!({
                            "LoadBalancerArn": lb.arn,
                            "DNSName": lb.dns_name,
                            "LoadBalancerName": lb.name,
                            "Scheme": lb.scheme,
                            "VpcId": lb.vpc_id,
                            "State": {
                                "Code": lb.state
                            },
                            "CreatedTime": lb.created_at,
                            "Type": "application"
                        })
                    }).collect();

                    let resp = json!({
                        "DescribeLoadBalancersResponse": {
                            "DescribeLoadBalancersResult": {
                                "LoadBalancers": lbs_json
                            },
                            "ResponseMetadata": {
                                "RequestId": "req-123"
                            }
                        }
                    });
                    Json(resp).into_response()
                },
                Err(e) => ApiError(e).into_response()
            }
        },
        _ => (axum::http::StatusCode::NOT_IMPLEMENTED, "Not Implemented").into_response(),
    }
}
