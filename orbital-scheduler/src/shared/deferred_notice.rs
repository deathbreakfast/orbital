//! Deferred feature notice for catalog doc-sections (SC-12, SC-13, SC-25, SC-26).

use leptos::prelude::*;
use orbital_core_components::{
    MessageBar, MessageBarBody, MessageBarIntent, MessageBarTitle, Text,
};

/// Info notice for features deferred per design charter.
#[component]
pub fn DeferredFeatureNotice(
    sc_id: &'static str,
    feature_name: &'static str,
    #[prop(optional, into)] hint: Option<String>,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let body = hint.unwrap_or_else(|| {
        "This capability is not available in the current Orbital scheduler release.".to_string()
    });
    let test_id = format!("scheduler-deferred-{sc_id}");
    view! {
        <div class=move || class.get().unwrap_or_default() data-testid=test_id>
            <MessageBar intent=MessageBarIntent::Info>
                <MessageBarBody>
                    <MessageBarTitle>{feature_name.to_string()}</MessageBarTitle>
                    <Text>{body}</Text>
                </MessageBarBody>
            </MessageBar>
        </div>
    }
}
