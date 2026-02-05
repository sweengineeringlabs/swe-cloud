use crate::error::EmulatorError;
use std::process::Command;
use std::io::Write;
use tempfile::tempdir;
use zip::ZipArchive;
use serde_json::{Value, json};

pub fn execute_lambda(
    runtime: &str,
    handler: &str,
    zip_bytes: &[u8],
    payload: &Value,
) -> Result<Value, EmulatorError> {
    if zip_bytes.is_empty() {
        return Err(EmulatorError::InvalidArgument("Function code is missing".into()));
    }

    let tmp_dir = tempdir().map_err(|e| EmulatorError::Internal(format!("Failed to create temp dir: {}", e)))?;
    let mut archive = ZipArchive::new(std::io::Cursor::new(zip_bytes))
        .map_err(|e| EmulatorError::InvalidArgument(format!("Invalid zip archive: {}", e)))?;
    
    archive.extract(tmp_dir.path())
        .map_err(|e| EmulatorError::Internal(format!("Failed to extract zip: {}", e)))?;
        
    let handler_parts: Vec<&str> = handler.split('.').collect();
    if handler_parts.len() != 2 {
        return Err(EmulatorError::InvalidArgument("Invalid handler format (expected file.method)".into()));
    }
    let file_name = handler_parts[0];
    let method_name = handler_parts[1];
    
    if runtime.contains("python") {
        execute_python(tmp_dir.path(), file_name, method_name, payload)
    } else if runtime.contains("node") {
        execute_nodejs(tmp_dir.path(), file_name, method_name, payload)
    } else {
        Err(EmulatorError::InvalidArgument(format!("Unsupported runtime: {}", runtime)))
    }
}

fn execute_python(
    path: &std::path::Path,
    file_name: &str,
    method_name: &str,
    payload: &Value,
) -> Result<Value, EmulatorError> {
    let payload_str = serde_json::to_string(payload).unwrap_or_default();
    
    // Wrapper script to call the handler
    let wrapper = format!(
        r#"
import json
import {file_name}
import sys

try:
    event = json.loads(sys.stdin.read())
    context = {{}}
    result = {file_name}.{method_name}(event, context)
    print(json.dumps(result))
except Exception as e:
    import traceback
    print(json.dumps({{"error": str(e), "stack": traceback.format_exc()}}))
    sys.exit(1)
"#
    );
    
    let mut child = Command::new("python")
        .current_dir(path)
        .arg("-c")
        .arg(wrapper)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| EmulatorError::Internal(format!("Failed to spawn python: {}", e)))?;
        
    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(payload_str.as_bytes()).unwrap();
    }
    
    let output = child.wait_with_output()
        .map_err(|e| EmulatorError::Internal(format!("Failed to run python: {}", e)))?;
        
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(EmulatorError::Internal(format!("Python execution failed: {}\n{}", stdout, stderr)));
    }
    
    let val: Value = serde_json::from_str(&stdout)
        .map_err(|e| EmulatorError::Internal(format!("Failed to parse python output: {}. Output was: {}", e, stdout)))?;
        
    Ok(json!({
        "StatusCode": 200,
        "Payload": val
    }))
}

fn execute_nodejs(
    path: &std::path::Path,
    file_name: &str,
    method_name: &str,
    payload: &Value,
) -> Result<Value, EmulatorError> {
    let payload_str = serde_json::to_string(payload).unwrap_or_default();
    
    // Wrapper script for Node.js
    let wrapper = format!(
        r#"
const handler = require('./{file_name}');
let data = '';
process.stdin.on('data', chunk => {{ data += chunk; }});
process.stdin.on('end', async () => {{
    try {{
        const event = JSON.parse(data);
        const context = {{}};
        const result = await handler.{method_name}(event, context);
        console.log(JSON.stringify(result));
    }} catch (e) {{
        console.log(JSON.stringify({{error: e.message, stack: e.stack}}));
        process.exit(1);
    }}
}});
"#
    );
    
    let mut child = Command::new("node")
        .current_dir(path)
        .arg("-e")
        .arg(wrapper)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| EmulatorError::Internal(format!("Failed to spawn node: {}", e)))?;
        
    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(payload_str.as_bytes()).unwrap();
    }
    
    let output = child.wait_with_output()
        .map_err(|e| EmulatorError::Internal(format!("Failed to run node: {}", e)))?;
        
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(EmulatorError::Internal(format!("Node.js execution failed: {}\n{}", stdout, stderr)));
    }
    
    let val: Value = serde_json::from_str(&stdout)
        .map_err(|e| EmulatorError::Internal(format!("Failed to parse node output: {}. Output was: {}", e, stdout)))?;
        
    Ok(json!({
        "StatusCode": 200,
        "Payload": val
    }))
}
