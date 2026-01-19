//! CloudEmu Service Detail Page
//!
//! Shows details for a specific resource instance.

use rustscript::prelude::*;
use crate::components::*;

#[derive(Props)]
pub struct CloudemuServiceDetailProps {
    pub provider: String,
    pub service: String,
    pub id: String,
}

#[page]
pub fn CloudemuServiceDetail(props: CloudemuServiceDetailProps) -> Element {
    let provider = &props.provider;
    let service = &props.service;
    let id = &props.id;
    let (resource, set_resource) = use_state(None::<Resource>);
    let (loading, set_loading) = use_state(true);
    let (tab, set_tab) = use_state("overview".to_string());

    use_effect(move || {
        set_loading(true);
        spawn(async move {
            let data = fetch_resource(provider, service, id).await;
            set_resource(data);
            set_loading(false);
        });
    }, [provider.clone(), service.clone(), id.clone()]);

    rsx! {
        <div class="cloudemu-service-detail">
            <header class="page-header">
                <Breadcrumb items={vec![
                    ("CloudEmu", "/cloudemu"),
                    (provider.to_uppercase().as_str(), &format!("/cloudemu/{}", provider)),
                    (service.to_uppercase().as_str(), &format!("/cloudemu/{}/{}", provider, service)),
                    (id.as_str(), ""),
                ]} />
                <div class="header-actions">
                    <h1>{id.clone()}</h1>
                    <div class="actions">
                        <Button variant="secondary" icon="edit">"Edit"</Button>
                        <Button variant="danger" icon="trash">"Delete"</Button>
                    </div>
                </div>
            </header>

            {if loading {
                rsx! { <Loading /> }
            } else if let Some(res) = &resource {
                rsx! {
                    <Tabs active={tab.clone()} on_change={set_tab.clone()}>
                        <Tab id="overview" label="Overview">
                            <ResourceOverview resource={res.clone()} />
                        </Tab>
                        <Tab id="config" label="Configuration">
                            <ResourceConfig resource={res.clone()} />
                        </Tab>
                        <Tab id="logs" label="Logs">
                            <ResourceLogs
                                provider={provider.clone()}
                                service={service.clone()}
                                resource_id={id.clone()}
                            />
                        </Tab>
                        <Tab id="metrics" label="Metrics">
                            <ResourceMetrics
                                provider={provider.clone()}
                                service={service.clone()}
                                resource_id={id.clone()}
                            />
                        </Tab>
                    </Tabs>
                }
            } else {
                rsx! {
                    <NotFound message="Resource not found" />
                }
            }}
        </div>
    }
}
