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
use tracing::info;
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use chrono;

pub async fn handle_request(
    State(emulator): State<Arc<Emulator>>,
    headers: HeaderMap,
    Json(body): Json<Value>,
) -> Response {
    let target = headers
        .get("x-amz-target")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("");
    
    info!("Cognito: {}", target);
    let action = target.split('.').next_back().unwrap_or(target);

    let result = match action {
        "CreateUserPool" => create_user_pool(&emulator, body).await,
        "ListUserPools" => list_user_pools(&emulator, body).await,
        "AdminCreateUser" => admin_create_user(&emulator, body).await,
        "AdminGetUser" => admin_get_user(&emulator, body).await,
        "CreateGroup" => create_group(&emulator, body).await,
        "ListGroups" => list_groups(&emulator, body).await,
        "InitiateAuth" => initiate_auth(&emulator, body).await,
        _ => Err(EmulatorError::InvalidRequest(format!("Unknown or unsupported target: {}", target))),
    };

    match result {
        Ok(json_val) => Json::<Value>(json_val).into_response(),
        Err(e) => {
            let code = e.code();
            let msg = e.message();
            let status = e.status_code();
            
            let json_err = json!({
                "__type": code,
                "message": msg
            });
            
            (status, Json::<Value>(json_err)).into_response()
        }
    }
}

async fn create_user_pool(emulator: &Emulator, body: Value) -> Result<Value, EmulatorError> {
    let name = body["PoolName"].as_str().ok_or_else(|| EmulatorError::InvalidArgument("Missing PoolName".into()))?;
    let pool = emulator.storage.create_user_pool(name, &emulator.config.account_id, &emulator.config.region)?;

    Ok(json!({
        "UserPool": {
            "Id": pool.id,
            "Name": pool.name,
            "Arn": pool.arn,
            "CreationDate": 1234567890.0,
            "Status": "Enabled"
        }
    }))
}

async fn list_user_pools(emulator: &Emulator, _body: Value) -> Result<Value, EmulatorError> {
    let pools = emulator.storage.list_user_pools()?;
    let pool_list: Vec<Value> = pools.into_iter().map(|p| {
        json!({
            "Id": p.id,
            "Name": p.name,
            "CreationDate": 1234567890.0
        })
    }).collect();

    Ok(json!({
        "UserPools": pool_list
    }))
}

async fn admin_create_user(emulator: &Emulator, body: Value) -> Result<Value, EmulatorError> {
    let pool_id = body["UserPoolId"].as_str().ok_or_else(|| EmulatorError::InvalidArgument("Missing UserPoolId".into()))?;
    let username = body["Username"].as_str().ok_or_else(|| EmulatorError::InvalidArgument("Missing Username".into()))?;
    
    let mut attributes = Vec::new();
    if let Some(attrs) = body["UserAttributes"].as_array() {
        for a in attrs {
            let name = a["Name"].as_str().unwrap_or("");
            let value = a["Value"].as_str().unwrap_or("");
            attributes.push((name.to_string(), value.to_string()));
        }
    }

    let user = emulator.storage.admin_create_user(pool_id, username, attributes.clone())?;
    
    let attr_list: Vec<Value> = attributes.into_iter().map(|(n, v)| {
        json!({ "Name": n, "Value": v })
    }).collect();

    Ok(json!({
        "User": {
            "Username": user.username,
            "Attributes": attr_list,
            "UserCreateDate": 1234567890.0,
            "UserStatus": user.status,
            "Enabled": user.enabled
        }
    }))
}

async fn admin_get_user(emulator: &Emulator, body: Value) -> Result<Value, EmulatorError> {
    let pool_id = body["UserPoolId"].as_str().ok_or_else(|| EmulatorError::InvalidArgument("Missing UserPoolId".into()))?;
    let username = body["Username"].as_str().ok_or_else(|| EmulatorError::InvalidArgument("Missing Username".into()))?;
    
    let (user, attrs) = emulator.storage.admin_get_user(pool_id, username)?;
    
    let attr_list: Vec<Value> = attrs.into_iter().map(|(n, v)| {
        json!({ "Name": n, "Value": v })
    }).collect();

    Ok(json!({
        "Username": user.username,
        "UserAttributes": attr_list,
        "UserCreateDate": 1234567890.0,
        "UserStatus": user.status,
        "Enabled": user.enabled
    }))
}

async fn create_group(emulator: &Emulator, body: Value) -> Result<Value, EmulatorError> {
    let pool_id = body["UserPoolId"].as_str().ok_or_else(|| EmulatorError::InvalidArgument("Missing UserPoolId".into()))?;
    let group_name = body["GroupName"].as_str().ok_or_else(|| EmulatorError::InvalidArgument("Missing GroupName".into()))?;
    let description = body["Description"].as_str();
    let precedence = body["Precedence"].as_i64().map(|i| i as i32);

    let group = emulator.storage.create_group(pool_id, group_name, description, precedence)?;

    Ok(json!({
        "Group": {
            "GroupName": group.group_name,
            "UserPoolId": group.user_pool_id,
            "Description": group.description,
            "Precedence": group.precedence,
            "CreationDate": 1234567890.0
        }
    }))
}

async fn list_groups(emulator: &Emulator, body: Value) -> Result<Value, EmulatorError> {
    let pool_id = body["UserPoolId"].as_str().ok_or_else(|| EmulatorError::InvalidArgument("Missing UserPoolId".into()))?;
    
    let groups = emulator.storage.list_groups(pool_id)?;
    let group_list: Vec<Value> = groups.into_iter().map(|g| {
        json!({
            "GroupName": g.group_name,
            "UserPoolId": g.user_pool_id,
            "Description": g.description,
            "Precedence": g.precedence,
            "CreationDate": 1234567890.0
        })
    }).collect();

    Ok(json!({
        "Groups": group_list
    }))
}

async fn initiate_auth(emulator: &Emulator, body: Value) -> Result<Value, EmulatorError> {
    let client_id = body["ClientId"].as_str().ok_or_else(|| EmulatorError::InvalidArgument("Missing ClientId".into()))?;
    let auth_params = body["AuthParameters"].as_object().ok_or_else(|| EmulatorError::InvalidArgument("Missing AuthParameters".into()))?;
    
    let username = auth_params.get("USERNAME").and_then(|v| v.as_str()).ok_or_else(|| EmulatorError::InvalidArgument("Missing USERNAME".into()))?;
    
    // Check if user exists (Assuming ClientId == PoolId)
    let _user_data = emulator.storage.admin_get_user(client_id, username)?;
    
    // Generate Tokens
    let access_token = generate_mock_jwt(username, "access");
    let id_token = generate_mock_jwt(username, "id");
    let refresh_token = "mock-refresh-token";
    
    Ok(json!({
        "AuthenticationResult": {
            "AccessToken": access_token,
            "ExpiresIn": 3600,
            "IdToken": id_token,
            "RefreshToken": refresh_token,
            "TokenType": "Bearer"
        },
        "ChallengeParameters": {}
    }))
}

fn generate_mock_jwt(username: &str, token_type: &str) -> String {
    let header = URL_SAFE_NO_PAD.encode(json!({"alg":"HS256","typ":"JWT"}).to_string());
    let payload = URL_SAFE_NO_PAD.encode(json!({
        "sub": username,
        "token_use": token_type,
        "exp": chrono::Utc::now().timestamp() + 3600,
        "iss": "cloudemu"
    }).to_string());
    format!("{}.{}.mock-signature", header, payload)
}
