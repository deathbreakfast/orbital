use leptos::prelude::*;
use orbital_base_components::SignalModel;
use orbital_core_components::{Pagination, PaginationConfig};

use crate::types::{DataTableSourceKind, DataTableTableState};

/// Pagination control wrapper (1-indexed UI, 0-indexed internal page / offset).
#[component]
pub fn DataTablePaginationBar(state: DataTableTableState) -> impl IntoView {
    let ui_page = RwSignal::new(1usize);

    if state.source_kind == DataTableSourceKind::Server {
        let limit = state.page_size.get_untracked() as u32;

        Effect::new(move |_| {
            let offset = state.server_offset.get();
            ui_page.set((offset / limit.max(1)) as usize + 1);
        });

        #[cfg(feature = "hydrate")]
        {
            Effect::new(move |_| {
                let current_page = ui_page.get();
                let derived_offset = (current_page.saturating_sub(1) as u32) * limit.max(1);
                if state.server_offset.get_untracked() != derived_offset {
                    state.server_offset.set(derived_offset);
                    state.notify_pagination();
                }
            });
        }

        #[cfg(not(feature = "hydrate"))]
        {
            Effect::new(move |_| {
                let current_page = ui_page.get();
                let derived_offset = (current_page.saturating_sub(1) as u32) * limit.max(1);
                if state.server_offset.get() != derived_offset {
                    state.server_offset.set(derived_offset);
                    state.notify_pagination();
                }
            });
        }
    } else {
        Effect::new(move |_| {
            ui_page.set(state.page.get() + 1);
        });

        Effect::new(move |_| {
            let internal = ui_page.get().saturating_sub(1);
            if state.page.get() != internal {
                state.page.set(internal);
                state.notify_pagination();
            }
        });
    }

    view! {
        <div data-testid="data-table-pagination">
            <Pagination config=PaginationConfig::new(
                SignalModel::from(ui_page),
                state.page_count.into(),
            ) />
        </div>
    }
}
