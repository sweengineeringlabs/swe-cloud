//! CloudEmu Overview Page
//!
//! Landing page for CloudEmu feature showing provider selection
//! and recent activity across all cloud emulators.

use rustscript::prelude::*;
use crate::components::*;

#[page]
pub fn CloudemuOverview() -> Element {
    let providers = vec!["aws", "azure", "gcp", "zero"];

    rsx! {
        <div class="cloudemu-overview">
            <header class="page-header">
                <h1>"CloudEmu"</h1>
                <p class="subtitle">"Cloud Service Emulation for Local Development"</p>
            </header>

            <section class="provider-grid">
                <h2>"Select Provider"</h2>
                <div class="grid grid-cols-4 gap-4">
                    {providers.iter().map(|provider| rsx! {
                        <ProviderCard
                            key={*provider}
                            provider={provider.to_string()}
                        />
                    })}
                </div>
            </section>

            <section class="recent-activity">
                <h2>"Recent Activity"</h2>
                <RecentRequestsTable limit={10} />
            </section>
        </div>
    }
}
