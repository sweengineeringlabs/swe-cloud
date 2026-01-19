//! CloudEmu Service New Page
//!
//! Form to create a new resource for a specific service.

use rustscript::prelude::*;
use crate::components::*;

#[derive(Props)]
pub struct CloudemuServiceNewProps {
    pub provider: String,
    pub service: String,
}

#[page]
pub fn CloudemuServiceNew(props: CloudemuServiceNewProps) -> Element {
    let provider = &props.provider;
    let service = &props.service;
    let navigate = use_navigate();
    let (submitting, set_submitting) = use_state(false);
    let (error, set_error) = use_state(None::<String>);

    let on_submit = move |form_data: FormData| {
        set_submitting(true);
        set_error(None);

        spawn(async move {
            match create_resource(provider, service, form_data).await {
                Ok(resource) => {
                    navigate(&format!("/cloudemu/{}/{}/{}", provider, service, resource.id));
                }
                Err(e) => {
                    set_error(Some(e.to_string()));
                    set_submitting(false);
                }
            }
        });
    };

    rsx! {
        <div class="cloudemu-service-new">
            <header class="page-header">
                <Breadcrumb items={vec![
                    ("CloudEmu", "/cloudemu"),
                    (provider.to_uppercase().as_str(), &format!("/cloudemu/{}", provider)),
                    (service.to_uppercase().as_str(), &format!("/cloudemu/{}/{}", provider, service)),
                    ("New", ""),
                ]} />
                <h1>{format!("Create New {}", service_display_name(service))}</h1>
            </header>

            {if let Some(err) = &error {
                rsx! {
                    <Alert variant="error" dismissible={true}>
                        {err.clone()}
                    </Alert>
                }
            }}

            <section class="create-form">
                <ResourceForm
                    provider={provider.clone()}
                    service={service.clone()}
                    on_submit={on_submit}
                    submitting={submitting}
                />
            </section>
        </div>
    }
}

fn service_display_name(service: &str) -> &str {
    match service {
        "s3" => "S3 Bucket",
        "dynamodb" => "DynamoDB Table",
        "lambda" => "Lambda Function",
        "sqs" => "SQS Queue",
        "sns" => "SNS Topic",
        "ec2" => "EC2 Instance",
        _ => service,
    }
}
