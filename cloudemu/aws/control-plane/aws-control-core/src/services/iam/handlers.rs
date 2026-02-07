use crate::Emulator;
use crate::error::EmulatorError;
use axum::{
    extract::State,
    http::HeaderMap,
    response::{IntoResponse, Response},
    Json,
};
use axum::extract::Query;
use std::collections::HashMap;
use serde_json::{json, Value};
use std::sync::Arc;
use crate::adapters::aws_query::parse_query_string;

pub async fn handle_request(
    State(emulator): State<Arc<Emulator>>,
    headers: HeaderMap,
    body_str: String,
) -> Response {
    // AWS IAM uses application/x-www-form-urlencoded
    let params: HashMap<String, String> = parse_query_string(&body_str);
    let action = params.get("Action").map(|s| s.as_str()).unwrap_or("");

    let result = match action {
        "CreateRole" => create_role(&emulator, &params).await,
        "GetRole" => get_role(&emulator, &params).await,
        "ListRoles" => list_roles(&emulator, &params).await,
        "CreatePolicy" => create_policy(&emulator, &params).await,
        "ListPolicies" => list_policies(&emulator, &params).await,
        "AttachRolePolicy" => attach_role_policy(&emulator, &params).await,
        "CreateUser" => create_user(&emulator, &params).await,
        "ListUsers" => list_users(&emulator, &params).await,
        "CreateAccessKey" => create_access_key(&emulator, &params).await,
        _ => Err(EmulatorError::NotImplemented(format!("IAM action: {}", action))),
    };

    match result {
        Ok(val) => (axum::http::StatusCode::OK, axum::response::Json(val)).into_response(), 
        Err(e) => crate::error::ApiError(e).into_response(),
    }
}

async fn create_role(emulator: &Emulator, params: &HashMap<String, String>) -> Result<Value, EmulatorError> {
    let name = params.get("RoleName").ok_or_else(|| EmulatorError::InvalidArgument("Missing RoleName".into()))?;
    let doc = params.get("AssumeRolePolicyDocument").ok_or_else(|| EmulatorError::InvalidArgument("Missing PolicyDocument".into()))?;
    
    let role = emulator.storage.create_role(name, doc)?;

    Ok(json!({
        "CreateRoleResponse": {
            "CreateRoleResult": {
                "Role": {
                    "RoleName": role.name,
                    "RoleId": "AROA...", 
                    "Arn": role.arn,
                    "CreateDate": "2023-01-01T00:00:00Z",
                    "Path": role.path,
                    "AssumeRolePolicyDocument": role.assume_role_policy_document
                }
            },
            "ResponseMetadata": {
                "RequestId": "req-123"
            }
        }
    }))
}

async fn get_role(emulator: &Emulator, params: &HashMap<String, String>) -> Result<Value, EmulatorError> {
    let name = params.get("RoleName").ok_or_else(|| EmulatorError::InvalidArgument("Missing RoleName".into()))?;
    let role = emulator.storage.get_role(name)?;
    
     Ok(json!({
        "GetRoleResponse": {
            "GetRoleResult": {
                "Role": {
                    "RoleName": role.name,
                    "RoleId": "AROA...", 
                    "Arn": role.arn,
                    "CreateDate": "2023-01-01T00:00:00Z",
                    "Path": role.path,
                    "AssumeRolePolicyDocument": role.assume_role_policy_document
                }
            },
            "ResponseMetadata": {
                "RequestId": "req-123"
            }
        }
    }))
}

async fn list_roles(emulator: &Emulator, _params: &HashMap<String, String>) -> Result<Value, EmulatorError> {
    let roles = emulator.storage.list_roles()?;
    
    let roles_json: Vec<Value> = roles.into_iter().map(|r| {
        json!({
            "RoleName": r.name,
            "RoleId": "AROA...",
            "Arn": r.arn,
            "CreateDate": "2023-01-01T00:00:00Z",
            "Path": r.path,
            "AssumeRolePolicyDocument": r.assume_role_policy_document
        })
    }).collect();

     Ok(json!({
        "ListRolesResponse": {
            "ListRolesResult": {
                "Roles": roles_json,
                "IsTruncated": false
            },
            "ResponseMetadata": {
                "RequestId": "req-123"
            }
        }
    }))
}

async fn create_policy(emulator: &Emulator, params: &HashMap<String, String>) -> Result<Value, EmulatorError> {
    let name = params.get("PolicyName").ok_or_else(|| EmulatorError::InvalidArgument("Missing PolicyName".into()))?;
    let doc = params.get("PolicyDocument").ok_or_else(|| EmulatorError::InvalidArgument("Missing PolicyDocument".into()))?;

    let policy = emulator.storage.create_policy(name, doc)?;

    Ok(json!({
        "CreatePolicyResponse": {
            "CreatePolicyResult": {
                "Policy": {
                    "PolicyName": policy.name,
                    "PolicyId": "ANPA...",
                    "Arn": policy.arn,
                    "Path": policy.path,
                    "DefaultVersionId": policy.default_version_id,
                    "AttachmentCount": 0,
                    "PermissionsBoundaryUsageCount": 0,
                    "IsAttachable": true,
                    "CreateDate": "2023-01-01T00:00:00Z",
                    "UpdateDate": "2023-01-01T00:00:00Z"
                }
            },
            "ResponseMetadata": {
                "RequestId": "req-123"
            }
        }
    }))
}

async fn list_policies(emulator: &Emulator, _params: &HashMap<String, String>) -> Result<Value, EmulatorError> {
    let policies = emulator.storage.list_policies()?;
    
    let policies_json: Vec<Value> = policies.into_iter().map(|p| {
        json!({
            "PolicyName": p.name,
            "PolicyId": "ANPA...",
            "Arn": p.arn,
            "Path": p.path,
            "DefaultVersionId": p.default_version_id,
            "AttachmentCount": 0,
            "PermissionsBoundaryUsageCount": 0,
            "IsAttachable": true,
            "CreateDate": "2023-01-01T00:00:00Z",
            "UpdateDate": "2023-01-01T00:00:00Z"
        })
    }).collect();

     Ok(json!({
        "ListPoliciesResponse": {
            "ListPoliciesResult": {
                 "Policies": policies_json,
                 "IsTruncated": false
            },
            "ResponseMetadata": {
                "RequestId": "req-123"
            }
        }
    }))
}

async fn attach_role_policy(emulator: &Emulator, params: &HashMap<String, String>) -> Result<Value, EmulatorError> {
    let role_name = params.get("RoleName").ok_or_else(|| EmulatorError::InvalidArgument("Missing RoleName".into()))?;
    let policy_arn = params.get("PolicyArn").ok_or_else(|| EmulatorError::InvalidArgument("Missing PolicyArn".into()))?;

    emulator.storage.attach_role_policy(role_name, policy_arn)?;

     Ok(json!({
        "AttachRolePolicyResponse": {
            "ResponseMetadata": {
                "RequestId": "req-123"
            }
        }
    }))
}

async fn create_user(emulator: &Emulator, params: &HashMap<String, String>) -> Result<Value, EmulatorError> {
    let name = params.get("UserName").ok_or_else(|| EmulatorError::InvalidArgument("Missing UserName".into()))?;
    
    let user = emulator.storage.create_user(name)?;

    Ok(json!({
        "CreateUserResponse": {
            "CreateUserResult": {
                "User": {
                    "UserName": user.name,
                    "UserId": user.id, 
                    "Arn": user.arn,
                    "CreateDate": "2023-01-01T00:00:00Z",
                    "Path": user.path
                }
            },
            "ResponseMetadata": {
                "RequestId": "req-123"
            }
        }
    }))
}

async fn list_users(emulator: &Emulator, _params: &HashMap<String, String>) -> Result<Value, EmulatorError> {
    let users = emulator.storage.list_users()?;
    
    let users_json: Vec<Value> = users.into_iter().map(|u| {
        json!({
            "UserName": u.name,
            "UserId": u.id,
            "Arn": u.arn,
            "CreateDate": "2023-01-01T00:00:00Z",
            "Path": u.path
        })
    }).collect();

     Ok(json!({
        "ListUsersResponse": {
            "ListUsersResult": {
                "Users": users_json,
                "IsTruncated": false
            },
            "ResponseMetadata": {
                "RequestId": "req-123"
            }
        }
    }))
}

async fn create_access_key(emulator: &Emulator, params: &HashMap<String, String>) -> Result<Value, EmulatorError> {
    let name = params.get("UserName").ok_or_else(|| EmulatorError::InvalidArgument("Missing UserName".into()))?;
    
    let key = emulator.storage.create_access_key(name)?;

    Ok(json!({
        "CreateAccessKeyResponse": {
            "CreateAccessKeyResult": {
                "AccessKey": {
                     "UserName": key.user_name,
                     "AccessKeyId": key.access_key_id,
                     "Status": key.status,
                     "SecretAccessKey": key.secret_access_key,
                     "CreateDate": "2023-01-01T00:00:00Z"
                }
            },
            "ResponseMetadata": {
                "RequestId": "req-123"
            }
        }
    }))
}
