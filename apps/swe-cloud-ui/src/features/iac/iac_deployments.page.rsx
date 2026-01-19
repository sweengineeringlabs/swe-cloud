//! IAC Deployments Page
//!
//! List all deployments.

use rustscript::prelude::*;
use crate::components::*;

#[page]
pub fn IacDeployments() -> Element {
    let (deployments, set_deployments) = use_state(Vec::new());
    let (loading, set_loading) = use_state(true);
    let (filter, set_filter) = use_state(DeploymentFilter::default());

    use_effect(move || {
        set_loading(true);
        spawn(async move {
            let data = fetch_deployments(&filter).await;
            set_deployments(data);
            set_loading(false);
        });
    }, [filter.clone()]);

    rsx! {
        <div class="iac-deployments">
            <header class="page-header">
                <Breadcrumb items={vec![
                    ("IAC", "/iac"),
                    ("Deployments", ""),
                ]} />
                <div class="header-actions">
                    <h1>"Deployments"</h1>
                    <Link to="/iac_deploy" class="btn btn-primary">
                        "New Deployment"
                    </Link>
                </div>
            </header>

            <section class="deployment-filters">
                <DeploymentFilterBar
                    filter={filter.clone()}
                    on_change={set_filter.clone()}
                />
            </section>

            <section class="deployment-list">
                {if loading {
                    rsx! { <Loading /> }
                } else if deployments.is_empty() {
                    rsx! {
                        <EmptyState
                            title="No Deployments"
                            description="No deployments found."
                            action_label="Create Deployment"
                            action_href="/iac_deploy"
                        />
                    }
                } else {
                    rsx! {
                        <DeploymentsTable deployments={deployments.clone()} />
                    }
                }}
            </section>
        </div>
    }
}
