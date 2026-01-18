use rsc::prelude::*;

#[page(route = "/workflow/:workflow_id", title = "Workflow")]
pub fn WorkflowRunner() -> Element {
    rsx! { div(class: "workflow-runner") { h1 { "Workflow Runner" } } }
}
