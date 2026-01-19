//! IAC State Page
//!
//! Shows current infrastructure state.

use rustscript::prelude::*;
use crate::components::*;

#[page]
pub fn IacState() -> Element {
    let (state, set_state) = use_state(None::<InfraState>);
    let (loading, set_loading) = use_state(true);

    use_effect(move || {
        set_loading(true);
        spawn(async move {
            let data = fetch_state().await;
            set_state(Some(data));
            set_loading(false);
        });
    }, []);

    let refresh_state = move |_| {
        set_loading(true);
        spawn(async move {
            let data = refresh_infra_state().await;
            set_state(Some(data));
            set_loading(false);
        });
    };

    rsx! {
        <div class="iac-state">
            <header class="page-header">
                <Breadcrumb items={vec![
                    ("IAC", "/iac"),
                    ("State", ""),
                ]} />
                <div class="header-actions">
                    <h1>"Infrastructure State"</h1>
                    <Button onclick={refresh_state} variant="secondary" icon="refresh">
                        "Refresh"
                    </Button>
                </div>
            </header>

            {if loading {
                rsx! { <Loading /> }
            } else if let Some(s) = &state {
                rsx! {
                    <div class="state-content">
                        <section class="state-summary">
                            <h2>"Summary"</h2>
                            <StateSummary state={s.clone()} />
                        </section>

                        <section class="state-resources">
                            <h2>"Resources"</h2>
                            <StateResourcesTree resources={s.resources.clone()} />
                        </section>

                        <section class="state-raw">
                            <h2>"Raw State"</h2>
                            <CodeViewer
                                code={serde_json::to_string_pretty(&s).unwrap()}
                                language="json"
                            />
                        </section>
                    </div>
                }
            } else {
                rsx! {
                    <EmptyState
                        title="No State"
                        description="No infrastructure state found."
                    />
                }
            }}
        </div>
    }
}
