//! IAC Plan Detail Page
//!
//! Shows details for a specific execution plan.

use rustscript::prelude::*;
use crate::components::*;

#[derive(Props)]
pub struct IacPlanDetailProps {
    pub id: String,
}

#[page]
pub fn IacPlanDetail(props: IacPlanDetailProps) -> Element {
    let plan_id = &props.id;
    let (plan, set_plan) = use_state(None::<Plan>);
    let (loading, set_loading) = use_state(true);
    let navigate = use_navigate();

    use_effect(move || {
        set_loading(true);
        spawn(async move {
            let data = fetch_plan(plan_id).await;
            set_plan(data);
            set_loading(false);
        });
    }, [plan_id.clone()]);

    let apply_plan = move |_| {
        spawn(async move {
            let deployment = apply_plan_execution(plan_id).await;
            navigate(&format!("/iac_deployments/{}", deployment.id));
        });
    };

    rsx! {
        <div class="iac-plan-detail">
            <header class="page-header">
                <Breadcrumb items={vec![
                    ("IAC", "/iac"),
                    ("Plans", "/iac_plans"),
                    (&format!("#{}", plan_id), ""),
                ]} />
                <div class="header-actions">
                    <h1>{format!("Plan #{}", plan_id)}</h1>
                    {if let Some(p) = &plan {
                        if p.status == "pending" {
                            rsx! {
                                <Button onclick={apply_plan} variant="primary">
                                    "Apply Plan"
                                </Button>
                            }
                        } else {
                            rsx! { <StatusBadge status={p.status.clone()} /> }
                        }
                    }}
                </div>
            </header>

            {if loading {
                rsx! { <Loading /> }
            } else if let Some(p) = &plan {
                rsx! {
                    <div class="plan-content">
                        <section class="plan-summary">
                            <h2>"Summary"</h2>
                            <PlanSummary plan={p.clone()} />
                        </section>

                        <section class="plan-changes">
                            <h2>"Changes"</h2>
                            <PlanChanges changes={p.changes.clone()} />
                        </section>

                        <section class="plan-diff">
                            <h2>"Diff"</h2>
                            <PlanDiff plan={p.clone()} />
                        </section>
                    </div>
                }
            } else {
                rsx! {
                    <NotFound message="Plan not found" />
                }
            }}
        </div>
    }
}
