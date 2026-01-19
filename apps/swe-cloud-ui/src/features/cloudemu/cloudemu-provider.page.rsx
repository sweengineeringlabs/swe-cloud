//! CloudEmu Provider Page
//!
//! Shows services available for a specific cloud provider.

use rustscript::prelude::*;
use crate::components::*;

#[derive(Props)]
pub struct CloudemuProviderProps {
    pub provider: String,
}

#[page]
pub fn CloudemuProvider(props: CloudemuProviderProps) -> Element {
    let provider = &props.provider;
    let services = get_services_for_provider(provider);

    rsx! {
        <div class="cloudemu-provider">
            <header class="page-header">
                <Breadcrumb items={vec![
                    ("CloudEmu", "/cloudemu"),
                    (provider.to_uppercase().as_str(), ""),
                ]} />
                <h1>{provider.to_uppercase()}</h1>
            </header>

            <section class="services-grid">
                <h2>"Available Services"</h2>
                <div class="grid grid-cols-3 gap-4">
                    {services.iter().map(|service| rsx! {
                        <ServiceCard
                            key={service.id.clone()}
                            provider={provider.clone()}
                            service={service.clone()}
                        />
                    })}
                </div>
            </section>

            <section class="provider-stats">
                <h2>"Statistics"</h2>
                <ProviderStats provider={provider.clone()} />
            </section>
        </div>
    }
}

fn get_services_for_provider(provider: &str) -> Vec<Service> {
    match provider {
        "aws" => vec![
            Service::new("s3", "S3", "Object Storage"),
            Service::new("dynamodb", "DynamoDB", "NoSQL Database"),
            Service::new("lambda", "Lambda", "Serverless Functions"),
            Service::new("sqs", "SQS", "Message Queue"),
            Service::new("sns", "SNS", "Notifications"),
            Service::new("ec2", "EC2", "Virtual Machines"),
        ],
        "azure" => vec![
            Service::new("blobs", "Blob Storage", "Object Storage"),
            Service::new("keyvault", "Key Vault", "Secrets Management"),
            Service::new("functions", "Functions", "Serverless Functions"),
        ],
        "gcp" => vec![
            Service::new("storage", "Cloud Storage", "Object Storage"),
            Service::new("pubsub", "Pub/Sub", "Messaging"),
            Service::new("compute", "Compute Engine", "Virtual Machines"),
        ],
        "zero" => vec![
            Service::new("queue", "Zero Queue", "Message Queue"),
        ],
        _ => vec![],
    }
}
