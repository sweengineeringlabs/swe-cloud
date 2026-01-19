//! CloudKit Overview Page
//!
//! Landing page for CloudKit feature.

use rustscript::prelude::*;
use crate::components::*;

#[page]
pub fn CloudkitOverview() -> Element {
    rsx! {
        <div class="cloudkit-overview">
            <header class="page-header">
                <h1>"CloudKit"</h1>
                <p class="subtitle">"Cloud Resource Management Toolkit"</p>
            </header>

            <section class="quick-actions">
                <h2>"Quick Actions"</h2>
                <div class="grid grid-cols-3 gap-4">
                    <ActionCard
                        title="Resources"
                        description="Browse and manage cloud resources"
                        icon="database"
                        href="/cloudkit_resources"
                    />
                    <ActionCard
                        title="Operations"
                        description="View running operations"
                        icon="activity"
                        href="/cloudkit_operations"
                    />
                    <ActionCard
                        title="API Explorer"
                        description="Explore cloud APIs"
                        icon="compass"
                        href="/cloudkit_explorer"
                    />
                </div>
            </section>

            <section class="recent-resources">
                <h2>"Recent Resources"</h2>
                <RecentResourcesList limit={5} />
            </section>
        </div>
    }
}
