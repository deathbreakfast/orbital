use leptos::prelude::*;

use super::types::{MaterialCorners, MaterialElevation, MaterialVariant};

/// Headless material surface root — DOM contract only, no theme styling.
#[component]
pub fn BaseMaterial(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
    #[prop(default = MaterialVariant::Solid)] variant: MaterialVariant,
    #[prop(default = MaterialElevation::Resting)] elevation: MaterialElevation,
    #[prop(default = MaterialCorners::Rounded)] corners: MaterialCorners,
    #[prop(optional, into)] role: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            class=move || {
                let extra = class.get().unwrap_or_default();
                if extra.is_empty() {
                    "orbital-material".to_string()
                } else {
                    format!("orbital-material {extra}")
                }
            }
            style=move || style.get()
            data-material-variant=variant.as_data()
            data-material-elevation=elevation.as_data()
            data-material-corners=corners.as_data()
            role=move || role.get().unwrap_or_else(|| "group".to_string())
        >
            {children()}
        </div>
    }
}
