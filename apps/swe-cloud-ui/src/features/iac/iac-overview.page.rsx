//! IAC Overview Page
//!
//! Landing page for Infrastructure as Code feature.

use rustscript::prelude::*;
use crate::components::*;

#[page]
pub fn IacOverview() -> Element {
    rsx! {
        <div class="iac-overview">
            <header class="page-header">
                <h1>"Infrastructure as Code"</h1>
                <p class="subtitle">"Deploy and manage cloud infrastructure"</p>
            </header>

            <section class="quick-actions">
                <h2>"Quick Actions"</h2>
                <div class="grid grid-cols-4 gap-4">
                    <ActionCard
                        title="Modules"
                        description="Browse infrastructure modules"
                        icon="box"
                        href="/iac-modules"
                    />
                    <ActionCard
                        title="Deploy"
                        description="Deploy infrastructure"
                        icon="upload-cloud"
                        href="/iac-deploy"
                    />
                    <ActionCard
                        title="State"
                        description="View current state"
                        icon="file-text"
                        href="/iac-state"
                    />
                    <ActionCard
                        title="Plans"
                        description="Execution plans"
                        icon="clipboard"
                        href="/iac-plans"
                    />
                </div>
            </section>

            <section class="recent-deployments">
                <h2>"Recent Deployments"</h2>
                <RecentDeploymentsList limit={5} />
            </section>
        </div>
    }
}
