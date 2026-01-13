/// Step Functions State Machine Interpreter
/// Implements Amazon States Language (ASL) execution
use serde_json::{json, Value};
use crate::error::{EmulatorError, Result};

pub struct StateMachineExecutor;

impl StateMachineExecutor {
    /// Execute a state machine with the given input
    pub fn execute(definition: &str, input: &str) -> Result<String> {
        let def: Value = serde_json::from_str(definition)
            .map_err(|e| EmulatorError::InvalidRequest(format!("Invalid state machine definition: {}", e)))?;
        
        let mut current_input: Value = serde_json::from_str(input)
            .map_err(|e| EmulatorError::InvalidArgument(format!("Invalid input JSON: {}", e)))?;
        
        let start_at = def["StartAt"].as_str()
            .ok_or_else(|| EmulatorError::InvalidRequest("Missing StartAt in definition".into()))?;
        
        let states = def["States"].as_object()
            .ok_or_else(|| EmulatorError::InvalidRequest("Missing States in definition".into()))?;
        
        let mut current_state = start_at.to_string();
        let mut iteration_count = 0;
        const MAX_ITERATIONS: usize = 1000; // Prevent infinite loops
        
        loop {
            if iteration_count >= MAX_ITERATIONS {
                return Err(EmulatorError::Internal("State machine exceeded maximum iterations".into()));
            }
            iteration_count += 1;
            
            let state_def = states.get(&current_state)
                .ok_or_else(|| EmulatorError::InvalidRequest(format!("State not found: {}", current_state)))?;
            
            let state_type = state_def["Type"].as_str()
                .ok_or_else(|| EmulatorError::InvalidRequest(format!("Missing Type for state: {}", current_state)))?;
            
            tracing::info!("StepFunctions: Executing state '{}' (Type: {})", current_state, state_type);
            
            match state_type {
                "Pass" => {
                    current_input = execute_pass_state(state_def, &current_input)?;
                },
                "Task" => {
                    current_input = execute_task_state(state_def, &current_input)?;
                },
                "Choice" => {
                    current_state = execute_choice_state(state_def, &current_input)?;
                    continue; // Don't check End, Choice handles its own transitions
                }
                "Wait" => {
                    current_input = execute_wait_state(state_def, &current_input)?;
                },
                "Succeed" => {
                    return Ok(current_input.to_string());
                },
                "Fail" => {
                    let error = state_def["Error"].as_str().unwrap_or("States.TaskFailed");
                    let cause = state_def["Cause"].as_str().unwrap_or("State machine failed");
                    return Err(EmulatorError::Internal(format!("{}: {}", error, cause)));
                },
                "Parallel" => {
                    current_input = execute_parallel_state(state_def, &current_input)?;
                },
                "Map" => {
                    current_input = execute_map_state(state_def, &current_input)?;
                },
                _ => {
                    return Err(EmulatorError::InvalidRequest(format!("Unknown state type: {}", state_type)));
                }
            }
            
            // Check if this is an end state
            if state_def["End"].as_bool().unwrap_or(false) {
                return Ok(current_input.to_string());
            }
            
            // Get next state
            current_state = state_def["Next"].as_str()
                .ok_or_else(|| EmulatorError::InvalidRequest(format!("State '{}' has no Next and is not an End state", current_state)))?.to_string();
        }
    }
}

fn execute_pass_state(state_def: &Value, input: &Value) -> Result<Value> {
    let mut output = input.clone();
    
    // Handle Result field (replaces the  state output)
    if let Some(result) = state_def.get("Result") {
        output = result.clone();
    }
    
    // Handle Parameters field (constructs input using JSONPath and intrinsic functions)
    if let Some(parameters) = state_def.get("Parameters") {
        output = apply_parameters(parameters, input)?;
    }
    
    // Handle ResultPath (where to put the result in the output)
    if let Some(result_path) = state_def["ResultPath"].as_str() {
        output = apply_result_path(&output, input, result_path);
    }
    
    // Handle OutputPath (filter the output)
    if let Some(output_path) = state_def["OutputPath"].as_str() {
        output = apply_output_path(&output, output_path)?;
    }
    
    Ok(output)
}

fn execute_task_state(_state_def: &Value, input: &Value) -> Result<Value> {
    // For emulator, Task states pass through their input as output
    // In a real implementation, this would invoke Lambda, run activities, etc.
    tracing::info!("StepFunctions: Task state executing (emulated - passing through)");
    Ok(input.clone())
}

fn execute_choice_state(state_def: &Value, input: &Value) -> Result<String> {
    let choices = state_def["Choices"].as_array()
        .ok_or_else(|| EmulatorError::InvalidRequest("Choice state missing Choices".into()))?;
    
    for choice in choices {
        if evaluate_choice_rule(choice, input) {
            return Ok(choice["Next"].as_str()
                .ok_or_else(|| EmulatorError::InvalidRequest("Choice rule missing Next".into()))?.to_string());
        }
    }
    
    // Default fallback
    Ok(state_def["Default"].as_str()
        .ok_or_else(|| EmulatorError::InvalidRequest("No matching choice and no Default".into()))?.to_string())
}

fn evaluate_choice_rule(rule: &Value, input: &Value) -> bool {
    // StringEquals
    if let Some(var) = rule["Variable"].as_str() {
        let value_at_path = json_path_get(input, var);
        
        if let Some(expected) = rule.get("StringEquals") {
            return value_at_path == Some(expected);
        }
        if let Some(expected) = rule.get("NumericEquals") {
            return value_at_path == Some(expected);
        }
        if let Some(expected) = rule.get("BooleanEquals") {
            return value_at_path == Some(expected);
        }
        if let Some(expected_val) = rule.get("NumericGreaterThan") {
            if let (Some(val), Some(exp)) = (value_at_path.and_then(|v| v.as_f64()), expected_val.as_f64()) {
                return val > exp;
            }
        }
        if let Some(expected_val) = rule.get("NumericLessThan") {
            if let (Some(val), Some(exp)) = (value_at_path.and_then(|v| v.as_f64()), expected_val.as_f64()) {
                return val < exp;
            }
        }
    }
    
    // And, Or, Not
    if let Some(and_rules) = rule["And"].as_array() {
        return and_rules.iter().all(|r| evaluate_choice_rule(r, input));
    }
    if let Some(or_rules) = rule["Or"].as_array() {
        return or_rules.iter().any(|r| evaluate_choice_rule(r, input));
    }
    if let Some(not_rule) = rule.get("Not") {
        return !evaluate_choice_rule(not_rule, input);
    }
    
    false
}

fn execute_wait_state(_state_def: &Value, input: &Value) -> Result<Value> {
    // For emulator, we don't actually wait
    tracing::info!("StepFunctions: Wait state (skipping wait in emulator)");
    Ok(input.clone())
}

fn execute_parallel_state(state_def: &Value, input: &Value) -> Result<Value> {
    let branches = state_def["Branches"].as_array()
        .ok_or_else(|| EmulatorError::InvalidRequest("Parallel state missing Branches".into()))?;
    
    let mut results = Vec::new();
    
    for branch in branches {
        // Execute each branch
        let branch_def = serde_json::to_string(branch)
            .map_err(|e| EmulatorError::Internal(e.to_string()))?;
        let branch_output = StateMachineExecutor::execute(&branch_def, &input.to_string())?;
        let output_val: Value = serde_json::from_str(&branch_output)
            .map_err(|e| EmulatorError::Internal(e.to_string()))?;
        results.push(output_val);
    }
    
    Ok(json!(results))
}

fn execute_map_state(state_def: &Value, input: &Value) -> Result<Value> {
    let items = input.as_array()
        .ok_or_else(|| EmulatorError::InvalidArgument("Map state input must be an array".into()))?;
    
    let iterator = state_def.get("Iterator")
        .ok_or_else(|| EmulatorError::InvalidRequest("Map state missing Iterator".into()))?;
    
    let mut results = Vec::new();
    
    for item in items {
        let iterator_def = serde_json::to_string(iterator)
            .map_err(|e| EmulatorError::Internal(e.to_string()))?;
        let item_output = StateMachineExecutor::execute(&iterator_def, &item.to_string())?;
        let output_val: Value = serde_json::from_str(&item_output)
            .map_err(|e| EmulatorError::Internal(e.to_string()))?;
        results.push(output_val);
    }
    
    Ok(json!(results))
}

fn apply_parameters(parameters: &Value, _input: &Value) -> Result<Value> {
    // Simple parameter transformation - just use the parameters if provided
    // Full implementation would support JSONPath references like $.field
    Ok(parameters.clone())
}

fn apply_result_path(result: &Value, original_input: &Value, path: &str) -> Value {
    if path == "$" {
        // Replace entire input with result
        result.clone()
    } else if path.is_empty() || path == "null" {
        // Discard result, return original input
        original_input.clone()
    } else if let Some(field) = path.strip_prefix("$.") {
        // Merge result into input at specified path
        let mut output = original_input.clone();
        if let Some(obj) = output.as_object_mut() {
            obj.insert(field.to_string(), result.clone());
        }
        output
    } else {
        result.clone()
    }
}

fn apply_output_path(output: &Value, path: &str) -> Result<Value> {
    if path == "$" {
        Ok(output.clone())
    } else {
        Ok(json_path_get(output, path).cloned().unwrap_or(Value::Null))
    }
}

fn json_path_get<'a>(value: &'a Value, path: &str) -> Option<&'a Value> {
    if path == "$" {
        return Some(value);
    }
    
    let path = path.strip_prefix("$.")?;
    let parts: Vec<&str> = path.split('.').collect();
    
    let mut current = value;
    for part in parts {
        current = current.get(part)?;
    }
    
    Some(current)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pass_state() {
        let def = json!({
            "StartAt": "Pass",
            "States": {
                "Pass": {
                    "Type": "Pass",
                    "Result": {"foo": "bar"},
                    "End": true
                }
            }
        }).to_string();
        
        let output = StateMachineExecutor::execute(&def, "{}").unwrap();
        assert_eq!(output, r#"{"foo":"bar"}"#);
    }

    #[test]
    fn test_choice_state() {
        let def = json!({
            "StartAt": "Choice",
            "States": {
                "Choice": {
                    "Type": "Choice",
                    "Choices": [
                        {
                            "Variable": "$.val",
                            "NumericEquals": 1,
                            "Next": "One"
                        }
                    ],
                    "Default": "Two"
                },
                "One": {
                    "Type": "Pass",
                    "Result": "1",
                    "End": true
                },
                "Two": {
                    "Type": "Pass",
                    "Result": "2",
                    "End": true
                }
            }
        }).to_string();
        
        // Test case 1
        let output = StateMachineExecutor::execute(&def, r#"{"val": 1}"#).unwrap();
        assert_eq!(output, "\"1\"");
        
        // Test case 2
        let output = StateMachineExecutor::execute(&def, r#"{"val": 2}"#).unwrap();
        assert_eq!(output, "\"2\"");
    }

    #[test]
    fn test_map_state() {
        let def = json!({
            "StartAt": "Map",
            "States": {
                "Map": {
                    "Type": "Map",
                    "Iterator": {
                        "StartAt": "Pass",
                        "States": {
                            "Pass": {
                                "Type": "Pass",
                                "End": true
                            }
                        }
                    },
                    "End": true
                }
            }
        }).to_string();
        
        // Process array [1, 2, 3] -> each item runs pass -> [1, 2, 3]
        let output = StateMachineExecutor::execute(&def, "[1, 2, 3]").unwrap();
        assert_eq!(output, "[1,2,3]");
    }
}

