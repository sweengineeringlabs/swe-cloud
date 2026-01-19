//! CloudKit Resources Page
//!
//! Browse all resource types.

use rustscript::prelude::*;
use crate::components::*;

#[page]
pub fn CloudkitResources() -> Element {
    let resource_types = vec![
        ("buckets", "Buckets", "database"),
        ("tables", "Tables", "table"),
        ("functions", "Functions", "code"),
        ("queues", "Queues", "list"),
        ("topics", "Topics", "bell"),
        ("instances", "Instances", "server"),
    ];

    rsx! {
        <div class="cloudkit-resources">
            <header class="page-header">
                <Breadcrumb items={vec![
                    ("CloudKit", "/cloudkit"),
                    ("Resources", ""),
                ]} />
                <h1>"Resources"</h1>
            </header>

            <section class="resource-types">
                <div class="grid grid-cols-3 gap-4">
                    {resource_types.iter().map(|(id, name, icon)| rsx! {
                        <ResourceTypeCard
                            key={*id}
                            id={id.to_string()}
                            name={name.to_string()}
                            icon={icon.to_string()}
                            href={format!("/cloudkit-resources/{}", id)}
                        />
                    })}
                </div>
            </section>
        </div>
    }
}
