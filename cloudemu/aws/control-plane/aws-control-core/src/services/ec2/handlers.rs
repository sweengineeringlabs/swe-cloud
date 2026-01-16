use crate::Emulator;
use crate::error::EmulatorError;
use axum::{
    extract::State,
    http::HeaderMap,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::{json, Value};
use std::sync::Arc;

pub async fn handle_request(
    State(emulator): State<Arc<Emulator>>,
    _headers: HeaderMap,
    Json(body): Json<Value>,
) -> Response {
    let action = body["Action"].as_str()
        .or_else(|| body["action"].as_str())
        .unwrap_or("");

    let result = match action {
        "RunInstances" => run_instances(&emulator, body).await,
        "DescribeInstances" => describe_instances(&emulator, body).await,
        "CreateVpc" => create_vpc(&emulator, body).await,
        "DescribeVpcs" => describe_vpcs(&emulator, body).await,
        "CreateSubnet" => create_subnet(&emulator, body).await,
        "DescribeSubnets" => describe_subnets(&emulator, body).await,
        "CreateSecurityGroup" => create_security_group(&emulator, body).await,
        "DescribeSecurityGroups" => describe_security_groups(&emulator, body).await,
        "CreateKeyPair" => create_key_pair(&emulator, body).await,
        "DescribeKeyPairs" => describe_key_pairs(&emulator, body).await,
        _ => Err(EmulatorError::NotImplemented(format!("EC2 action: {}", action))),
    };

    match result {
        Ok(val) => (axum::http::StatusCode::OK, Json(val)).into_response(),
        Err(e) => crate::error::ApiError(e).into_response(),
    }
}

async fn run_instances(emulator: &Emulator, body: Value) -> Result<Value, EmulatorError> {
    let image_id = body["ImageId"].as_str().unwrap_or("ami-mock");
    let instance_type = body["InstanceType"].as_str().unwrap_or("t3.micro");
    let subnet_id = body["SubnetId"].as_str();
    
    let instance = emulator.storage.run_instances(
        image_id,
        instance_type,
        None, // vpc_id
        subnet_id,
        body["KeyName"].as_str()
    )?;
    
    Ok(json!({
        "Instances": [instance]
    }))
}

async fn describe_instances(emulator: &Emulator, _body: Value) -> Result<Value, EmulatorError> {
    let instances = emulator.storage.list_instances()?;
    Ok(json!({
        "Reservations": [
            {
                "Instances": instances
            }
        ]
    }))
}

async fn create_vpc(emulator: &Emulator, body: Value) -> Result<Value, EmulatorError> {
    let cidr = body["CidrBlock"].as_str().ok_or_else(|| EmulatorError::InvalidArgument("Missing CidrBlock".into()))?;
    let vpc = emulator.storage.create_vpc(cidr)?;
    Ok(json!({ "Vpc": vpc }))
}

async fn describe_vpcs(emulator: &Emulator, _body: Value) -> Result<Value, EmulatorError> {
    let vpcs = emulator.storage.list_vpcs()?;
    Ok(json!({ "Vpcs": vpcs }))
}

async fn create_subnet(emulator: &Emulator, body: Value) -> Result<Value, EmulatorError> {
    let vpc_id = body["VpcId"].as_str().ok_or_else(|| EmulatorError::InvalidArgument("Missing VpcId".into()))?;
    let cidr = body["CidrBlock"].as_str().ok_or_else(|| EmulatorError::InvalidArgument("Missing CidrBlock".into()))?;
    let az = body["AvailabilityZone"].as_str().unwrap_or("us-east-1a");
    
    let subnet = emulator.storage.create_subnet(vpc_id, cidr, az)?;
    Ok(json!({ "Subnet": subnet }))
}

async fn describe_subnets(emulator: &Emulator, _body: Value) -> Result<Value, EmulatorError> {
    let subnets = emulator.storage.list_subnets()?;
    Ok(json!({ "Subnets": subnets }))
}

async fn create_security_group(emulator: &Emulator, body: Value) -> Result<Value, EmulatorError> {
    let vpc_id = body["VpcId"].as_str().ok_or_else(|| EmulatorError::InvalidArgument("Missing VpcId".into()))?;
    let name = body["GroupName"].as_str().ok_or_else(|| EmulatorError::InvalidArgument("Missing GroupName".into()))?;
    let desc = body["Description"].as_str().unwrap_or("");
    
    let sg = emulator.storage.create_security_group(vpc_id, name, desc)?;
    Ok(json!({ "GroupId": sg.id }))
}

async fn describe_security_groups(emulator: &Emulator, _body: Value) -> Result<Value, EmulatorError> {
    let sgs = emulator.storage.list_security_groups()?;
    Ok(json!({ "SecurityGroups": sgs }))
}

async fn create_key_pair(emulator: &Emulator, body: Value) -> Result<Value, EmulatorError> {
    let name = body["KeyName"].as_str().ok_or_else(|| EmulatorError::InvalidArgument("Missing KeyName".into()))?;
    let key = emulator.storage.create_key_pair(name)?;
    Ok(json!(key))
}

async fn describe_key_pairs(emulator: &Emulator, _body: Value) -> Result<Value, EmulatorError> {
    let keys = emulator.storage.list_key_pairs()?;
    Ok(json!({ "KeyPairs": keys }))
}
