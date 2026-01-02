use dioxus::prelude::*;
use crate::components::resource_tree::*;

#[component]
pub fn ExplorerPage() -> Element {
    let mut selected = use_signal(|| None::<ResourceNode>);

    let resources = vec![
        ResourceNode {
            name: "S3".into(),
            resource_type: ResourceType::S3Bucket,
            children: vec![
                ResourceNode {
                    name: "my-app-data".into(),
                    resource_type: ResourceType::S3Bucket,
                    children: vec![],
                },
                ResourceNode {
                    name: "backups".into(),
                    resource_type: ResourceType::S3Bucket,
                    children: vec![],
                },
            ],
        },
        ResourceNode {
            name: "DynamoDB".into(),
            resource_type: ResourceType::DynamoDBTable,
            children: vec![
                ResourceNode {
                    name: "users".into(),
                    resource_type: ResourceType::DynamoDBTable,
                    children: vec![],
                },
                ResourceNode {
                    name: "sessions".into(),
                    resource_type: ResourceType::DynamoDBTable,
                    children: vec![],
                },
            ],
        },
        ResourceNode {
            name: "SQS".into(),
            resource_type: ResourceType::SQSQueue,
            children: vec![
                ResourceNode {
                    name: "task-queue".into(),
                    resource_type: ResourceType::SQSQueue,
                    children: vec![],
                },
            ],
        },
    ];

    rsx! {
        div { class: "page explorer",
            header { class: "page-header",
                h1 { "Resource Explorer" }
                div { class: "controls",
                    select { class: "provider-select",
                        option { "CloudEmu" }
                        option { "AWS" }
                        option { "Azure" }
                    }
                    button { class: "btn-refresh", "Refresh" }
                }
            }

            div { class: "explorer-layout",
                aside { class: "resource-sidebar",
                    ResourceTree {
                        resources,
                        on_select: move |node| selected.set(Some(node)),
                    }
                }

                main { class: "resource-content",
                    match selected.read().as_ref() {
                        Some(node) => rsx! {
                            h2 { "{node.name}" }
                            p { "Resource type: {node.resource_type:?}" }
                            div { class: "resource-actions",
                                button { "Upload" }
                                button { "Create Folder" }
                                button { "Delete" }
                            }
                        },
                        None => rsx! {
                            p { class: "placeholder", "Select a resource" }
                        },
                    }
                }
            }
        }
    }
}
