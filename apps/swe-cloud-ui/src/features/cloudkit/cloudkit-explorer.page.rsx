//! CloudKit API Explorer Page
//!
//! Interactive API explorer for cloud services.

use rustscript::prelude::*;
use crate::components::*;

#[page]
pub fn CloudkitExplorer() -> Element {
    let provider_ctx = use_context::<ProviderContext>();
    let (selected_api, set_selected_api) = use_state(None::<ApiEndpoint>);
    let (request, set_request) = use_state(ApiRequest::default());
    let (response, set_response) = use_state(None::<ApiResponse>);
    let (loading, set_loading) = use_state(false);

    let execute_request = move |_| {
        set_loading(true);
        set_response(None);

        spawn(async move {
            let result = execute_api_request(&request).await;
            set_response(Some(result));
            set_loading(false);
        });
    };

    rsx! {
        <div class="cloudkit-explorer">
            <header class="page-header">
                <Breadcrumb items={vec![
                    ("CloudKit", "/cloudkit"),
                    ("API Explorer", ""),
                ]} />
                <h1>"API Explorer"</h1>
            </header>

            <div class="explorer-layout">
                <aside class="api-sidebar">
                    <h3>"APIs"</h3>
                    <ApiTree
                        provider={provider_ctx.current()}
                        selected={selected_api.clone()}
                        on_select={set_selected_api.clone()}
                    />
                </aside>

                <main class="explorer-main">
                    <section class="request-builder">
                        <h3>"Request"</h3>
                        <RequestBuilder
                            endpoint={selected_api.clone()}
                            request={request.clone()}
                            on_change={set_request.clone()}
                        />
                        <Button
                            onclick={execute_request}
                            loading={loading}
                            disabled={selected_api.is_none()}
                        >
                            "Execute"
                        </Button>
                    </section>

                    <section class="response-viewer">
                        <h3>"Response"</h3>
                        {if loading {
                            rsx! { <Loading /> }
                        } else if let Some(res) = &response {
                            rsx! { <ResponseViewer response={res.clone()} /> }
                        } else {
                            rsx! { <p class="text-muted">"Execute a request to see the response"</p> }
                        }}
                    </section>
                </main>
            </div>
        </div>
    }
}
