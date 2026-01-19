//! CloudEmu Logs Page
//!
//! Shows request logs across all cloud emulators.

use rustscript::prelude::*;
use crate::components::*;

#[page]
pub fn CloudemuLogs() -> Element {
    let (logs, set_logs) = use_state(Vec::new());
    let (loading, set_loading) = use_state(true);
    let (filter, set_filter) = use_state(LogFilter::default());
    let provider_ctx = use_context::<ProviderContext>();

    use_effect(move || {
        set_loading(true);
        spawn(async move {
            let data = fetch_logs(&filter, provider_ctx.current()).await;
            set_logs(data);
            set_loading(false);
        });
    }, [filter.clone(), provider_ctx.current()]);

    rsx! {
        <div class="cloudemu-logs">
            <header class="page-header">
                <Breadcrumb items={vec![
                    ("CloudEmu", "/cloudemu"),
                    ("Logs", ""),
                ]} />
                <h1>"Request Logs"</h1>
            </header>

            <section class="log-filters">
                <LogFilterBar
                    filter={filter.clone()}
                    on_change={set_filter.clone()}
                />
            </section>

            <section class="log-list">
                {if loading {
                    rsx! { <Loading /> }
                } else if logs.is_empty() {
                    rsx! {
                        <EmptyState
                            title="No Logs"
                            description="No request logs found for the selected filters."
                        />
                    }
                } else {
                    rsx! {
                        <LogTable logs={logs.clone()} />
                    }
                }}
            </section>
        </div>
    }
}

#[derive(Clone, Default)]
struct LogFilter {
    provider: Option<String>,
    service: Option<String>,
    level: Option<String>,
    time_range: Option<TimeRange>,
}
