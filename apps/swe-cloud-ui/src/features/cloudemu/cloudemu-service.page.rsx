//! CloudEmu Service Page
//!
//! Lists resources for a specific service (e.g., S3 buckets, DynamoDB tables).

use rustscript::prelude::*;
use crate::components::*;

#[derive(Props)]
pub struct CloudemuServiceProps {
    pub provider: String,
    pub service: String,
}

#[page]
pub fn CloudemuService(props: CloudemuServiceProps) -> Element {
    let provider = &props.provider;
    let service = &props.service;
    let (resources, set_resources) = use_state(Vec::new());
    let (loading, set_loading) = use_state(true);

    use_effect(move || {
        set_loading(true);
        spawn(async move {
            let data = fetch_resources(provider, service).await;
            set_resources(data);
            set_loading(false);
        });
    }, [provider.clone(), service.clone()]);

    rsx! {
        <div class="cloudemu-service">
            <header class="page-header">
                <Breadcrumb items={vec![
                    ("CloudEmu", "/cloudemu"),
                    (provider.to_uppercase().as_str(), &format!("/cloudemu/{}", provider)),
                    (service.to_uppercase().as_str(), ""),
                ]} />
                <div class="header-actions">
                    <h1>{format!("{} - {}", provider.to_uppercase(), service.to_uppercase())}</h1>
                    <Link to={format!("/cloudemu/{}/{}/new", provider, service)} class="btn btn-primary">
                        "Create New"
                    </Link>
                </div>
            </header>

            <section class="resources-list">
                {if loading {
                    rsx! { <Loading /> }
                } else if resources.is_empty() {
                    rsx! {
                        <EmptyState
                            title="No Resources"
                            description="Create your first resource to get started."
                            action_label="Create Resource"
                            action_href={format!("/cloudemu/{}/{}/new", provider, service)}
                        />
                    }
                } else {
                    rsx! {
                        <ResourceTable
                            resources={resources.clone()}
                            provider={provider.clone()}
                            service={service.clone()}
                        />
                    }
                }}
            </section>
        </div>
    }
}
