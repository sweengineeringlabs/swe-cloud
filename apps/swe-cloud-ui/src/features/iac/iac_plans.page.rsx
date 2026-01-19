//! IAC Plans Page
//!
//! List execution plans.

use rustscript::prelude::*;
use crate::components::*;

#[page]
pub fn IacPlans() -> Element {
    let (plans, set_plans) = use_state(Vec::new());
    let (loading, set_loading) = use_state(true);

    use_effect(move || {
        set_loading(true);
        spawn(async move {
            let data = fetch_plans().await;
            set_plans(data);
            set_loading(false);
        });
    }, []);

    rsx! {
        <div class="iac-plans">
            <header class="page-header">
                <Breadcrumb items={vec![
                    ("IAC", "/iac"),
                    ("Plans", ""),
                ]} />
                <h1>"Execution Plans"</h1>
            </header>

            <section class="plans-list">
                {if loading {
                    rsx! { <Loading /> }
                } else if plans.is_empty() {
                    rsx! {
                        <EmptyState
                            title="No Plans"
                            description="No execution plans found."
                        />
                    }
                } else {
                    rsx! {
                        <PlansTable plans={plans.clone()} />
                    }
                }}
            </section>
        </div>
    }
}
