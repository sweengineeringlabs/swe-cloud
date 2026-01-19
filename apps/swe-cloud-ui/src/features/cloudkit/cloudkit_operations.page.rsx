//! CloudKit Operations Page
//!
//! Shows running and recent operations.

use rustscript::prelude::*;
use crate::components::*;

#[page]
pub fn CloudkitOperations() -> Element {
    let (operations, set_operations) = use_state(Vec::new());
    let (loading, set_loading) = use_state(true);

    use_effect(move || {
        set_loading(true);
        spawn(async move {
            let data = fetch_operations().await;
            set_operations(data);
            set_loading(false);
        });
    }, []);

    rsx! {
        <div class="cloudkit-operations">
            <header class="page-header">
                <Breadcrumb items={vec![
                    ("CloudKit", "/cloudkit"),
                    ("Operations", ""),
                ]} />
                <h1>"Operations"</h1>
            </header>

            <section class="running-operations">
                <h2>"Running"</h2>
                {if loading {
                    rsx! { <Loading /> }
                } else {
                    let running: Vec<_> = operations.iter()
                        .filter(|op| op.status == "running")
                        .collect();

                    if running.is_empty() {
                        rsx! { <p class="text-muted">"No running operations"</p> }
                    } else {
                        rsx! {
                            <OperationsList operations={running} />
                        }
                    }
                }}
            </section>

            <section class="recent-operations">
                <h2>"Recent"</h2>
                {if loading {
                    rsx! { <Loading /> }
                } else {
                    let recent: Vec<_> = operations.iter()
                        .filter(|op| op.status != "running")
                        .take(20)
                        .collect();

                    rsx! {
                        <OperationsTable operations={recent} />
                    }
                }}
            </section>
        </div>
    }
}
