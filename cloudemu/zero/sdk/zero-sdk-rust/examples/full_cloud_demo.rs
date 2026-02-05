use zero_sdk::ZeroClient;
use serde_json::json;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. Initialize Client
    // Uses ZERO_URL env var or defaults to http://localhost:8080
    let client = ZeroClient::from_env();

    println!("🚀 Starting ZeroCloud SDK Demo...");

    // 2. ZeroStore (S3) Demo
    println!("\n📦 [ZeroStore]");
    client.store().create_bucket("media-assets").await?;
    let buckets = client.store().list_buckets().await?;
    println!("   Buckets: {:?}", buckets);

    // 3. ZeroDB (DynamoDB) Demo
    println!("\n📊 [ZeroDB]");
    client.db().create_table("users", "id").await?;
    client.db().put_item("users", "user_001", json!({
        "name": "Alice",
        "email": "alice@zero.local",
        "tags": ["beta", "tester"]
    })).await?;
    let tables = client.db().list_tables().await?;
    println!("   Tables: {:?}", tables);

    // 4. ZeroQueue (SQS) Demo
    println!("\n📨 [ZeroQueue]");
    client.queue().create_queue("notifications").await?;
    let msg_id = client.queue().send_message("notifications", "Welcome to ZeroCloud!").await?;
    println!("   Sent Message ID: {}", msg_id);

    if let Some(msg) = client.queue().receive_message("notifications").await? {
        println!("   Received Message: {}", msg.body);
        client.queue().delete_message("notifications", &msg.receipt_handle).await?;
        println!("   Deleted message using handle.");
    }

    // 5. ZeroID (IAM) Demo
    println!("\n👤 [ZeroID]");
    client.iam().create_user("dev-user").await?;
    client.iam().create_role("app-executor").await?;
    println!("   User and Role created.");

    // 6. ZeroLB (ALB) Demo
    println!("\n⚖️ [ZeroLB]");
    client.lb().create_load_balancer("web-lb", "application").await?;
    let tg_arn = client.lb().create_target_group("backend-tg", 8080).await?;
    client.lb().create_listener("web-lb", 80, &tg_arn).await?;
    println!("   ALB configured with listener on port 80.");

    println!("\n✅ SDK Demo Completed Successfully!");

    Ok(())
}
