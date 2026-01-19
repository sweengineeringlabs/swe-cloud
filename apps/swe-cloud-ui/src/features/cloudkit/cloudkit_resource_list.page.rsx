//! CloudKit Resource List Page
//!
//! Lists resources of a specific type.

use rustscript::prelude::*;
use crate::components::*;

#[derive(Props)]
pub struct CloudkitResourceListProps {
    #[prop(name = "type")]
    pub resource_type: String,
}

#[page]
pub fn CloudkitResourceList(props: CloudkitResourceListProps) -> Element {
    let resource_type = &props.resource_type;
    let (resources, set_resources) = use_state(Vec::new());
    let (loading, set_loading) = use_state(true);

    use_effect(move || {
        set_loading(true);
        spawn(async move {
            let data = fetch_resources_by_type(resource_type).await;
            set_resources(data);
            set_loading(false);
        });
    }, [resource_type.clone()]);

    rsx! {
        <div class="cloudkit-resource-list">
            <header class="page-header">
                <Breadcrumb items={vec![
                    ("CloudKit", "/cloudkit"),
                    ("Resources", "/cloudkit_resources"),
                    (resource_type.as_str(), ""),
                ]} />
                <h1>{format!("{}", capitalize(resource_type))}</h1>
            </header>

            <section class="resource-list">
                {if loading {
                    rsx! { <Loading /> }
                } else if resources.is_empty() {
                    rsx! {
                        <EmptyState
                            title={format!("No {}", resource_type)}
                            description="No resources found of this type."
                        />
                    }
                } else {
                    rsx! {
                        <ResourceTable
                            resources={resources.clone()}
                            resource_type={resource_type.clone()}
                        />
                    }
                }}
            </section>
        </div>
    }
}
