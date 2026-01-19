//! IAC Module Detail Page
//!
//! Shows details for a specific infrastructure module.

use rustscript::prelude::*;
use crate::components::*;

#[derive(Props)]
pub struct IacModuleDetailProps {
    pub module: String,
}

#[page]
pub fn IacModuleDetail(props: IacModuleDetailProps) -> Element {
    let module_id = &props.module;
    let (module, set_module) = use_state(None::<Module>);
    let (loading, set_loading) = use_state(true);
    let navigate = use_navigate();

    use_effect(move || {
        set_loading(true);
        spawn(async move {
            let data = fetch_module(module_id).await;
            set_module(data);
            set_loading(false);
        });
    }, [module_id.clone()]);

    let deploy_module = move |_| {
        navigate(&format!("/iac_deploy?module={}", module_id));
    };

    rsx! {
        <div class="iac-module-detail">
            <header class="page-header">
                <Breadcrumb items={vec![
                    ("IAC", "/iac"),
                    ("Modules", "/iac_modules"),
                    (module_id.as_str(), ""),
                ]} />
                <div class="header-actions">
                    <h1>{module_id.clone()}</h1>
                    <Button onclick={deploy_module} variant="primary">
                        "Deploy Module"
                    </Button>
                </div>
            </header>

            {if loading {
                rsx! { <Loading /> }
            } else if let Some(m) = &module {
                rsx! {
                    <div class="module-content">
                        <section class="module-info">
                            <h2>"Overview"</h2>
                            <p>{m.description.clone()}</p>
                            <ModuleMetadata module={m.clone()} />
                        </section>

                        <section class="module-inputs">
                            <h2>"Inputs"</h2>
                            <VariablesList variables={m.inputs.clone()} />
                        </section>

                        <section class="module-outputs">
                            <h2>"Outputs"</h2>
                            <VariablesList variables={m.outputs.clone()} />
                        </section>

                        <section class="module-readme">
                            <h2>"Documentation"</h2>
                            <Markdown content={m.readme.clone()} />
                        </section>
                    </div>
                }
            } else {
                rsx! {
                    <NotFound message="Module not found" />
                }
            }}
        </div>
    }
}
