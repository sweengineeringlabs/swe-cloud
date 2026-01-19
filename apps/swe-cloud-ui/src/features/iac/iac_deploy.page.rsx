//! IAC Deploy Page
//!
//! Deploy infrastructure using selected module.

use rustscript::prelude::*;
use crate::components::*;

#[page]
pub fn IacDeploy() -> Element {
    let query = use_query();
    let preselected_module = query.get("module");

    let (step, set_step) = use_state(1);
    let (selected_module, set_selected_module) = use_state(preselected_module);
    let (config, set_config) = use_state(DeployConfig::default());
    let (plan, set_plan) = use_state(None::<Plan>);
    let (deploying, set_deploying) = use_state(false);

    let generate_plan = move |_| {
        spawn(async move {
            let result = create_plan(&selected_module.unwrap(), &config).await;
            set_plan(Some(result));
            set_step(3);
        });
    };

    let execute_deploy = move |_| {
        set_deploying(true);
        spawn(async move {
            let result = execute_deployment(&plan.unwrap()).await;
            // Navigate to deployment detail
        });
    };

    rsx! {
        <div class="iac-deploy">
            <header class="page-header">
                <Breadcrumb items={vec![
                    ("IAC", "/iac"),
                    ("Deploy", ""),
                ]} />
                <h1>"Deploy Infrastructure"</h1>
            </header>

            <Stepper current={step}>
                <Step number={1} title="Select Module" />
                <Step number={2} title="Configure" />
                <Step number={3} title="Review Plan" />
                <Step number={4} title="Deploy" />
            </Stepper>

            <section class="deploy-content">
                {match step {
                    1 => rsx! {
                        <ModuleSelector
                            selected={selected_module.clone()}
                            on_select={move |m| {
                                set_selected_module(Some(m));
                                set_step(2);
                            }}
                        />
                    },
                    2 => rsx! {
                        <ConfigureModule
                            module={selected_module.clone().unwrap()}
                            config={config.clone()}
                            on_change={set_config.clone()}
                            on_back={move |_| set_step(1)}
                            on_next={generate_plan}
                        />
                    },
                    3 => rsx! {
                        <PlanReview
                            plan={plan.clone().unwrap()}
                            on_back={move |_| set_step(2)}
                            on_approve={move |_| set_step(4)}
                        />
                    },
                    4 => rsx! {
                        <DeployExecution
                            plan={plan.clone().unwrap()}
                            deploying={deploying}
                            on_deploy={execute_deploy}
                        />
                    },
                    _ => rsx! { <div /> },
                }}
            </section>
        </div>
    }
}
