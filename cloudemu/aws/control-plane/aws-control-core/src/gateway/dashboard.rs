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
                .card { background: var(--card-bg); border-radius: 12px; box-shadow: 0 8px 16px rgba(0,0,0,0.05); margin-bottom: 2rem; padding: 1.5rem; transition: transform 0.2s; }
                .card:hover { transform: translateY(-2px); }
                h1 { margin: 0; color: var(--secondary); font-size: 2rem; }
                h2 { margin-top: 0; border-bottom: 2px solid var(--bg); padding-bottom: 0.5rem; color: var(--primary); font-size: 1.25rem; display: flex; align-items: center; }
                h2 .icon { margin-right: 0.5rem; font-size: 1.5rem; }
                ul { list-style: none; padding: 0; margin: 0; }
                li { padding: 0.75rem 0; border-bottom: 1px solid #f0f0f0; display: flex; flex-direction: column; }
                li:last-child { border-bottom: none; }
                .res-name { font-weight: 600; color: var(--secondary); margin-bottom: 0.2rem; }
                .res-meta { font-size: 0.85rem; color: #7f8c8d; font-family: monospace; word-break: break-all; }
                .badge { background: #34495e; color: white; padding: 0.25rem 0.6rem; border-radius: 20px; font-size: 0.75rem; font-weight: bold; }
                .empty { color: #bdc3c7; font-style: italic; text-align: center; padding: 1rem; }
                .stats { display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 1rem; margin-bottom: 2rem; }
                .stat-box { background: var(--primary); color: white; padding: 1rem; border-radius: 8px; text-align: center; }
                .stat-val { font-size: 1.5rem; font-weight: bold; }
                .stat-label { font-size: 0.8rem; text-transform: uppercase; opacity: 0.9; }
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
    let queues = emulator.storage.list_queues().unwrap_or_default();
    let topics = emulator.storage.list_topics().unwrap_or_default();

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
    html.push_str(&format!("{}", queues.len() + topics.len()));
    html.push_str(r#"</div><div class="stat-label">Messaging</div></div>
                </div>

                <div class="card">
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

    html.push_str(r#"<div class="card">
                    <h2><span class="icon">✉️</span> SQS Queues</h2>
                    <ul>"#);
    if queues.is_empty() { html.push_str("<li class='empty'>No queues created</li>"); }
    for q in queues {
        html.push_str(&format!("<li><span class='res-name'>{}</span> <span class='res-meta'>URL: {}</span><span class='res-meta'>ARN: {}</span></li>", q.name, q.url, q.arn));
    }
    html.push_str("</ul></div>");

    html.push_str(r#"<div class="card">
                    <h2><span class="icon">🔔</span> SNS Topics</h2>
                    <ul>"#);
    if topics.is_empty() { html.push_str("<li class='empty'>No topics created</li>"); }
    for t in topics {
        html.push_str(&format!("<li><span class='res-name'>{}</span> <span class='res-meta'>ARN: {}</span></li>", t.name, t.arn));
    }
    html.push_str("</ul></div>");

    html.push_str("</div></body></html>");
    Html(html)
}
