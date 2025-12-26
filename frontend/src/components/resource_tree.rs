use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ResourceNode {
    pub name: String,
    pub resource_type: ResourceType,
    pub children: Vec<ResourceNode>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum ResourceType {
    S3Bucket,
    DynamoDBTable,
    SQSQueue,
    SNSTopic,
    LambdaFunction,
}

#[component]
pub fn ResourceTree(resources: Vec<ResourceNode>, on_select: EventHandler<ResourceNode>) -> Element {
    rsx! {
        div { class: "resource-tree",
            for resource in resources {
                ResourceItem { resource, on_select }
            }
        }
    }
}

#[component]
fn ResourceItem(resource: ResourceNode, on_select: EventHandler<ResourceNode>) -> Element {
    let icon = match resource.resource_type {
        ResourceType::S3Bucket => "folder",
        ResourceType::DynamoDBTable => "table",
        ResourceType::SQSQueue => "queue",
        ResourceType::SNSTopic => "broadcast",
        ResourceType::LambdaFunction => "function",
    };

    let res = resource.clone();

    rsx! {
        div { class: "resource-item",
            div {
                class: "resource-header",
                onclick: move |_| on_select.call(res.clone()),
                span { class: "icon", "{icon}" }
                span { class: "name", "{resource.name}" }
            }
            if !resource.children.is_empty() {
                div { class: "resource-children",
                    for child in resource.children {
                        ResourceItem { resource: child, on_select }
                    }
                }
            }
        }
    }
}
