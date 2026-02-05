use axum::{extract::State, response::Html};
use crate::Emulator;
use std::sync::Arc;

pub async fn render_dashboard(State(emulator): State<Arc<Emulator>>) -> Html<String> {
    let mut html = String::from(r#"
        <!DOCTYPE html>
        <html>
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>CloudEmu Dashboard</title>
            <style>
                :root { --primary: #3498db; --secondary: #2c3e50; --bg: #f5f7fa; --card-bg: #ffffff; --text: #333; }
                body { font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif; margin: 0; background-color: var(--bg); color: var(--text); }
                .container { max-width: 1200px; margin: 0 auto; padding: 2rem; }
                header { display: flex; align-items: center; justify-content: space-between; margin-bottom: 2rem; }
                .card { background: var(--card-bg); border-radius: 12px; box-shadow: 0 8px 16px rgba(0,0,0,0.05); margin-bottom: 2rem; padding: 1_5rem; transition: transform 0_2s; }
                .card:hover { transform: translateY(-2px); }
                h1 { margin: 0; color: var(--secondary); font-size: 2rem; }
                h2 { margin-top: 0; border-bottom: 2px solid var(--bg); padding-bottom: 0_5rem; color: var(--primary); font-size: 1_25rem; display: flex; align-items: center; }
                h2 .icon { margin-right: 0_5rem; font-size: 1_5rem; }
                ul { list-style: none; padding: 0; margin: 0; }
                li { padding: 0_75rem 0; border-bottom: 1px solid #f0f0f0; display: flex; flex-direction: column; }
                li:last-child { border-bottom: none; }
                .res-name { font-weight: 600; color: var(--secondary); margin-bottom: 0_2rem; }
                .res-meta { font-size: 0_85rem; color: #7f8c8d; font-family: monospace; word-break: break-all; }
                .badge { background: #34495e; color: white; padding: 0_25rem 0_6rem; border-radius: 20px; font-size: 0_75rem; font-weight: bold; }
                .empty { color: #bdc3c7; font-style: italic; text-align: center; padding: 1rem; }
                .stats { display: grid; grid-template-columns: repeat(auto-fit, minmax(180px, 1fr)); gap: 1rem; margin-bottom: 2rem; }
                .stat-box { background: var(--primary); color: white; padding: 1rem; border-radius: 8px; text-align: center; }
                .stat-val { font-size: 1_5rem; font-weight: bold; }
                .stat-label { font-size: 0_8rem; text-transform: uppercase; opacity: 0_9; }
            </style>
        </head>
        <body>
            <div class="container">
                <header>
                    <h1>CloudEmu Dashboard</h1>
                    <span class="badge">AWS EMULATOR READY</span>
                </header>

                <div class="stats">
                    <div class="stat-box">
                        <div class="stat-val">"#,
    );

    let buckets = emulator.storage.list_buckets().unwrap_or_default();
    let tables = emulator.storage.list_tables().unwrap_or_default();
    let functions = emulator.storage.list_functions().unwrap_or_default();
    let instances = emulator.storage.list_instances().unwrap_or_default();
    let vpcs = emulator.storage.list_vpcs().unwrap_or_default();
    let _queues = emulator.storage.list_queues().unwrap_or_default();

    html.push_str(&format!("{}", buckets.len()));
    html.push_str(r#"</div><div class="stat-label">Buckets</div></div>
                    <div class="stat-box">
                        <div class="stat-val">"#);
    html.push_str(&format!("{}", tables.len()));
    html.push_str(r#"</div><div class="stat-label">Tables</div></div>
                    <div class="stat-box">
                        <div class="stat-val">"#);
    html.push_str(&format!("{}", functions.len()));
    html.push_str(r#"</div><div class="stat-label">Functions</div></div>
                    <div class="stat-box">
                        <div class="stat-val">"#);
    html.push_str(&format!("{}", instances.len()));
    html.push_str(r#"</div><div class="stat-label">Instances</div></div>
                    <div class="stat-box">
                        <div class="stat-val">"#);
    html.push_str(&format!("{}", vpcs.len()));
    html.push_str(r#"</div><div class="stat-label">VPCs</div></div>
                </div>

                <div class="card">
                    <h2><span class="icon">🖥️</span> EC2 Instances</h2>
                    <ul>"#);
    if instances.is_empty() { html.push_str("<li class='empty'>No instances running</li>"); }
    for i in instances {
        html.push_str(&format!("<li><span class='res-name'>{} ({})</span> <span class='res-meta'>Type: {} | IP: {} | Launched: {}</span></li>", i.id, i.state, i.instance_type, i.public_ip.unwrap_or_default(), i.launch_time));
    }
    html.push_str("</ul></div>");

    html.push_str(r#"<div class="card">
                    <h2><span class="icon">🌐</span> VPC Networking</h2>
                    <ul>"#);
    if vpcs.is_empty() { html.push_str("<li class='empty'>No VPCs created</li>"); }
    for v in vpcs {
        html.push_str(&format!("<li><span class='res-name'>{}</span> <span class='res-meta'>CIDR: {} | State: {}</span></li>", v.id, v.cidr_block, v.state));
    }
    html.push_str("</ul></div>");

    html.push_str(r#"<div class="card">
                    <h2><span class="icon">📦</span> S3 Buckets</h2>
                    <ul>"#);
    if buckets.is_empty() { html.push_str("<li class='empty'>No buckets created</li>"); }
    for b in buckets {
        html.push_str(&format!("<li><span class='res-name'>{}</span> <span class='res-meta'>Region: {} | Created: {}</span></li>", b.name, b.region, b.created_at));
    }
    html.push_str("</ul></div>");

    html.push_str(r#"<div class="card">
                    <h2><span class="icon">📊</span> DynamoDB Tables</h2>
                    <ul>"#);
    if tables.is_empty() { html.push_str("<li class='empty'>No tables created</li>"); }
    for t in tables {
        html.push_str(&format!("<li><span class='res-name'>{}</span> <span class='res-meta'>ARN: {} | Status: {}</span></li>", t.name, t.arn, t.status));
    }
    html.push_str("</ul></div>");

    html.push_str(r#"<div class="card">
                    <h2><span class="icon">λ</span> Lambda Functions</h2>
                    <ul>"#);
    if functions.is_empty() { html.push_str("<li class='empty'>No functions created</li>"); }
    for f in functions {
        html.push_str(&format!("<li><span class='res-name'>{}</span> <span class='res-meta'>Runtime: {} | Handler: {}</span> <span class='res-meta'>ARN: {}</span></li>", f.name, f.runtime, f.handler, f.arn));
    }
    html.push_str("</ul></div>");

    html.push_str("</div></body></html>");
    Html(html)
}
