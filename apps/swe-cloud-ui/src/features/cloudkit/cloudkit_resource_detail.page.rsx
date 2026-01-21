//! CloudKit Resource Detail Page
//!
//! Shows details for a specific resource.

use rustscript::prelude::*;
use crate::components::*;

#[derive(Props)]
pub struct CloudkitResourceDetailProps {
    pub resource_type: String,
    pub id: String,
}

#[page]
pub fn CloudkitResourceDetail(props: CloudkitResourceDetailProps) -> Element {
    let resource_type = &props.resource_type;
    let id = &props.id;
    let (resource, set_resource) = use_state(None::<Resource>);
    let (loading, set_loading) = use_state(true);

    use_effect(move || {
        set_loading(true);
        spawn(async move {
            let data = fetch_resource_detail(resource_type, id).await;
            set_resource(data);
            set_loading(false);
        });
    }, [resource_type.clone(), id.clone()]);

    rsx! {
        <div class="cloudkit-resource-detail">
            <header class="page-header">
                <Breadcrumb items={vec![
                    ("CloudKit", "/cloudkit"),
                    ("Resources", "/cloudkit_resources"),
                    (resource_type.as_str(), &format!("/cloudkit_resources/{}", resource_type)),
                    (id.as_str(), ""),
                ]} />
                <h1>{id.clone()}</h1>
            </header>

            {if loading {
                rsx! { <Loading /> }
            } else if let Some(res) = &resource {
                rsx! {
                    <ResourceDetailView resource={res.clone()} />
                }
            } else {
                rsx! {
                    <NotFound message="Resource not found" />
                }
            }}
        </div>
    }
}
