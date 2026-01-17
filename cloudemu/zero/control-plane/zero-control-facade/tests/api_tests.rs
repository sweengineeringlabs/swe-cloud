use zero_control_facade::start_server;
use tokio::time::{sleep, Duration};
use serde_json::json;

#[tokio::test]
async fn test_zero_facade_api_integration() {
    let port = 8081; // Use a different port for tests

    // Start server in background
    let server_handle = tokio::spawn(async move {
        start_server(port, false, true).await.unwrap();
    });

    // Wait for server to start
    sleep(Duration::from_millis(500)).await;

    let client = reqwest::Client::new();
    let base_url = format!("http://localhost:{}", port);

    // 1. Test Node List (GET /v1/nodes)
    let resp = client.get(format!("{}/v1/nodes", base_url))
        .send().await.unwrap();
    assert_eq!(resp.status(), 200);
    let nodes: serde_json::Value = resp.json().await.unwrap();
    assert!(nodes["nodes"].is_array());

    // 2. Test Create Workload (POST /v1/workloads)
    let resp = client.post(format!("{}/v1/workloads", base_url))
        .json(&json!({ "id": "test-vm", "image": "ubuntu" }))
        .send().await.unwrap();
    assert_eq!(resp.status(), 200);
    let workload: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(workload["id"], "test-vm");
    assert_eq!(workload["state"], "Running");

    // 3. Test Create Network (POST /v1/networks)
    let resp = client.post(format!("{}/v1/networks", base_url))
        .json(&json!({ "id": "test-net", "cidr": "10.0.1.0/24" }))
        .send().await.unwrap();
    assert_eq!(resp.status(), 200);
    let network: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(network["id"], "test-net");

    // 4. Test Delete Workload (DELETE /v1/workloads)
    let resp = client.delete(format!("{}/v1/workloads", base_url))
        .json(&json!({ "id": "test-vm" }))
        .send().await.unwrap();
    assert_eq!(resp.status(), 200);

    // 5. Test Create Bucket (POST /v1/store/buckets)
    let resp = client.post(format!("{}/v1/store/buckets", base_url))
        .json(&json!({ "name": "test-bucket" }))
        .send().await.unwrap();
    assert_eq!(resp.status(), 200);
    let bucket_resp: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(bucket_resp["status"], "Created");
    assert_eq!(bucket_resp["name"], "test-bucket");

    // 6. Test List Buckets (GET /v1/store/buckets)
    let resp = client.get(format!("{}/v1/store/buckets", base_url))
        .send().await.unwrap();
    assert_eq!(resp.status(), 200);
    let buckets: serde_json::Value = resp.json().await.unwrap();
    let bucket_array = buckets["buckets"].as_array().unwrap();
    // Check if test-bucket exists in the array
    let found = bucket_array.iter().any(|b| b.as_str() == Some("test-bucket"));
    assert!(found, "Bucket 'test-bucket' not found in response: {:?}", bucket_array);

    // 7. Test Create ZeroDB Table (POST /v1/db/tables)
    let resp = client.post(format!("{}/v1/db/tables", base_url))
        .json(&json!({ "name": "TestTable", "pk": "UserId" }))
        .send().await.unwrap();
    assert_eq!(resp.status(), 200);
    
    // 8. Test Put Item (POST /v1/db/tables/TestTable/items)
    let resp = client.post(format!("{}/v1/db/tables/TestTable/items", base_url))
        .json(&json!({ "pk": "user-1", "data": "some-value" }))
        .send().await.unwrap();
    if resp.status() != 200 {
        let text = resp.text().await.unwrap();
        println!("Put Item Failed: {}", text);
        panic!("Put Item status: {}", 500);
    }

    // 9. Test List Tables (GET /v1/db/tables)
    let resp = client.get(format!("{}/v1/db/tables", base_url))
        .send().await.unwrap();
    assert_eq!(resp.status(), 200);
    let tables: serde_json::Value = resp.json().await.unwrap();
    let table_array = tables["tables"].as_array().unwrap();
    let found_table = table_array.iter().any(|t| t.as_str() == Some("TestTable"));
    assert!(found_table, "Table 'TestTable' not found");

    // 10. Test Create Function (POST /v1/func/functions)
    let resp = client.post(format!("{}/v1/func/functions", base_url))
        .json(&json!({ "name": "hello-func", "code": "print('hello')" }))
        .send().await.unwrap();
    assert_eq!(resp.status(), 200);

    // 11. Test List Functions (GET /v1/func/functions)
    let resp = client.get(format!("{}/v1/func/functions", base_url))
        .send().await.unwrap();
    assert_eq!(resp.status(), 200);
    let funcs: serde_json::Value = resp.json().await.unwrap();
    let func_array = funcs["functions"].as_array().unwrap();
    let found_func = func_array.iter().any(|f| f.as_str() == Some("hello-func"));
    assert!(found_func, "Function 'hello-func' not found");

    // 12. Test Invoke Function (POST /v1/func/functions/hello-func/invocations)
    let resp = client.post(format!("{}/v1/func/functions/hello-func/invocations", base_url))
        .json(&json!({ "key": "value" }))
        .send().await.unwrap();
    if resp.status() != 200 {
        let text = resp.text().await.unwrap();
        panic!("Invoke Failed: {}", text);
    }
    let result: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(result["status"], "Executed");
    assert_eq!(result["function"], "hello-func");

    // 13. Test Create Queue (POST /v1/queue/queues)
    let resp = client.post(format!("{}/v1/queue/queues", base_url))
        .json(&json!({ "name": "test-queue" }))
        .send().await.unwrap();
    assert_eq!(resp.status(), 200);

    // 14. Test Send Message (POST /v1/queue/queues/test-queue/messages)
    let resp = client.post(format!("{}/v1/queue/queues/test-queue/messages", base_url))
        .json(&json!({ "body": "hello-queue" }))
        .send().await.unwrap();
    if resp.status() != 200 {
        let text = resp.text().await.unwrap();
        panic!("Send Message Failed: {}", text);
    }

    // 15. Test Receive Message (GET /v1/queue/queues/test-queue/messages)
    let resp = client.get(format!("{}/v1/queue/queues/test-queue/messages", base_url))
        .send().await.unwrap();
    assert_eq!(resp.status(), 200);
    let msg: serde_json::Value = resp.json().await.unwrap();
    // Assuming delete-on-read, we should get one message
    let body_str = msg["Messages"]["Body"].as_str().unwrap_or("");
    assert_eq!(body_str, "hello-queue");

    // 16. Test Create IAM User (POST /v1/iam/users)
    let resp = client.post(format!("{}/v1/iam/users", base_url))
        .json(&json!({ "username": "zero-admin" }))
        .send().await.unwrap();
    assert_eq!(resp.status(), 200);

    // 17. Test List IAM Users (GET /v1/iam/users)
    let resp = client.get(format!("{}/v1/iam/users", base_url))
        .send().await.unwrap();
    assert_eq!(resp.status(), 200);
    let users: serde_json::Value = resp.json().await.unwrap();
    let user_array = users["Users"].as_array().unwrap();
    assert!(user_array.iter().any(|u| u["UserName"] == "zero-admin"));

    // 18. Test Attach IAM Policy (POST /v1/iam/users/zero-admin/policy)
    let policy_doc = json!({
        "Version": "2012-10-17",
        "Statement": [
            { "Effect": "Allow", "Action": "*", "Resource": "*" }
        ]
    });
    let resp = client.post(format!("{}/v1/iam/users/zero-admin/policy", base_url))
        .json(&json!({ "PolicyDocument": policy_doc }))
        .send().await.unwrap();
    assert_eq!(resp.status(), 200, "Attach policy failed");

    // 19. Verify Policy Persistence (List Users again)
    let resp = client.get(format!("{}/v1/iam/users", base_url))
         .send().await.unwrap();
    let users_v2: serde_json::Value = resp.json().await.unwrap();
    let admin_user = users_v2["Users"].as_array().unwrap().iter().find(|u| u["UserName"] == "zero-admin").unwrap();
    assert!(admin_user["Policy"].as_str().unwrap().contains("2012-10-17"), "Policy not persisted");

    // 20. Test IAM Roles
    let resp = client.post(format!("{}/v1/iam/roles", base_url))
        .json(&json!({ "Rolename": "ZeroWorkerRole" }))
        .send().await.unwrap();
    assert_eq!(resp.status(), 200, "Create role failed");
    
    let resp = client.get(format!("{}/v1/iam/roles", base_url)).send().await.unwrap();
    let roles: serde_json::Value = resp.json().await.unwrap();
    let role_list = roles["Roles"].as_array().unwrap();
    assert!(role_list.iter().any(|r| r["RoleName"] == "ZeroWorkerRole"));

    // 21. Test IAM Groups
    let resp = client.post(format!("{}/v1/iam/groups", base_url))
        .json(&json!({ "Groupname": "ZeroDevelopers" }))
        .send().await.unwrap();
    assert_eq!(resp.status(), 200, "Create group failed");

    let resp = client.get(format!("{}/v1/iam/groups", base_url)).send().await.unwrap();
    let groups: serde_json::Value = resp.json().await.unwrap();
    let group_list = groups["Groups"].as_array().unwrap();
    assert!(group_list.iter().any(|g| g["GroupName"] == "ZeroDevelopers"));

    // Abort server
    server_handle.abort();
}
