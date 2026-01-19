//! IAC Deployment Detail Page
//!
//! Shows details for a specific deployment.

use rustscript::prelude::*;
use crate::components::*;

#[derive(Props)]
pub struct IacDeploymentDetailProps {
    pub id: String,
}

#[page]
pub fn IacDeploymentDetail(props: IacDeploymentDetailProps) -> Element {
    let deployment_id = &props.id;
    let (deployment, set_deployment) = use_state(None::<Deployment>);
    let (loading, set_loading) = use_state(true);
    let (tab, set_tab) = use_state("overview".to_string());

    use_effect(move || {
        set_loading(true);
        spawn(async move {
            let data = fetch_deployment(deployment_id).await;
            set_deployment(data);
            set_loading(false);
        });
    }, [deployment_id.clone()]);

    rsx! {
        <div class="iac-deployment-detail">
            <header class="page-header">
                <Breadcrumb items={vec![
                    ("IAC", "/iac"),
                    ("Deployments", "/iac_deployments"),
                    (&format!("#{}", deployment_id), ""),
                ]} />
                <div class="header-actions">
                    <h1>{format!("Deployment #{}", deployment_id)}</h1>
                    {if let Some(d) = &deployment {
                        rsx! {
                            <StatusBadge status={d.status.clone()} />
                        }
                    }}
                </div>
            </header>

            {if loading {
                rsx! { <Loading /> }
            } else if let Some(d) = &deployment {
                rsx! {
                    <Tabs active={tab.clone()} on_change={set_tab.clone()}>
                        <Tab id="overview" label="Overview">
                            <DeploymentOverview deployment={d.clone()} />
                        </Tab>
                        <Tab id="plan" label="Plan">
                            <PlanViewer plan={d.plan.clone()} />
                        </Tab>
                        <Tab id="logs" label="Logs">
                            <DeploymentLogs deployment_id={deployment_id.clone()} />
                        </Tab>
                        <Tab id="resources" label="Resources">
                            <DeploymentResources deployment={d.clone()} />
                        </Tab>
                    </Tabs>
                }
            } else {
                rsx! {
                    <NotFound message="Deployment not found" />
                }
            }}
        </div>
    }
}
