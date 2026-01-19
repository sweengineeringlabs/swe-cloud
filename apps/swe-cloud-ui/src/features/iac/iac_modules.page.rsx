//! IAC Modules Page
//!
//! Browse available infrastructure modules.

use rustscript::prelude::*;
use crate::components::*;

#[page]
pub fn IacModules() -> Element {
    let (modules, set_modules) = use_state(Vec::new());
    let (loading, set_loading) = use_state(true);
    let (search, set_search) = use_state(String::new());

    use_effect(move || {
        set_loading(true);
        spawn(async move {
            let data = fetch_modules().await;
            set_modules(data);
            set_loading(false);
        });
    }, []);

    let filtered_modules: Vec<_> = modules.iter()
        .filter(|m| search.is_empty() || m.name.contains(&search))
        .collect();

    rsx! {
        <div class="iac-modules">
            <header class="page-header">
                <Breadcrumb items={vec![
                    ("IAC", "/iac"),
                    ("Modules", ""),
                ]} />
                <h1>"Modules"</h1>
            </header>

            <section class="module-search">
                <SearchInput
                    value={search.clone()}
                    on_change={set_search.clone()}
                    placeholder="Search modules..."
                />
            </section>

            <section class="module-list">
                {if loading {
                    rsx! { <Loading /> }
                } else if filtered_modules.is_empty() {
                    rsx! {
                        <EmptyState
                            title="No Modules"
                            description="No modules found matching your search."
                        />
                    }
                } else {
                    rsx! {
                        <div class="grid grid-cols-3 gap-4">
                            {filtered_modules.iter().map(|module| rsx! {
                                <ModuleCard
                                    key={module.id.clone()}
                                    module={(*module).clone()}
                                />
                            })}
                        </div>
                    }
                }}
            </section>
        </div>
    }
}
