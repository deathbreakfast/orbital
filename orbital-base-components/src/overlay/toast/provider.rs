use leptos::prelude::*;

use super::toast_container::BaseToastContainer;
use crate::overlay::{feedback_intent::FeedbackIntent, themed_portal::ThemedPortal};

pub const DEFAULT_TOAST_TIMEOUT_MS: i32 = 8000;
pub const DEFAULT_TOAST_LIMIT: u32 = 4;

#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub enum ToastStackPosition {
    Top,
    TopStart,
    TopEnd,
    #[default]
    Bottom,
    BottomStart,
    BottomEnd,
}

impl ToastStackPosition {
    pub const ALL: [Self; 6] = [
        Self::TopStart,
        Self::TopEnd,
        Self::Top,
        Self::BottomStart,
        Self::BottomEnd,
        Self::Bottom,
    ];

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Top => "top",
            Self::TopStart => "top-start",
            Self::TopEnd => "top-end",
            Self::Bottom => "bottom",
            Self::BottomStart => "bottom-start",
            Self::BottomEnd => "bottom-end",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ToastOffset {
    pub horizontal: i32,
    pub vertical: i32,
}

impl Default for ToastOffset {
    fn default() -> Self {
        Self {
            horizontal: 20,
            vertical: 16,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ToasterConfig {
    pub default_position: ToastStackPosition,
    pub default_intent: FeedbackIntent,
    pub default_timeout: i32,
    pub default_pause_on_hover: bool,
    pub limit: u32,
    pub offset: ToastOffset,
    pub inline: bool,
}

impl Default for ToasterConfig {
    fn default() -> Self {
        Self {
            default_position: ToastStackPosition::BottomEnd,
            default_intent: FeedbackIntent::Info,
            default_timeout: DEFAULT_TOAST_TIMEOUT_MS,
            default_pause_on_hover: false,
            limit: DEFAULT_TOAST_LIMIT,
            offset: ToastOffset::default(),
            inline: false,
        }
    }
}

#[derive(Clone)]
pub struct ToastAction {
    pub label: String,
    pub dismiss: bool,
    pub on_click: Option<Callback<()>>,
}

#[derive(Clone)]
enum ToastContent {
    Text {
        title: String,
        body: Option<String>,
        footer_actions: Vec<ToastAction>,
    },
    View(StoredValue<Box<dyn Fn() -> AnyView + Send + Sync>>),
}

#[derive(Clone)]
pub struct ToastOptions {
    content: ToastContent,
    intent: Option<FeedbackIntent>,
    pub position: Option<ToastStackPosition>,
    pub id: Option<String>,
    pub timeout: Option<i32>,
    pub pause_on_hover: Option<bool>,
}

impl ToastOptions {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            content: ToastContent::Text {
                title: title.into(),
                body: None,
                footer_actions: Vec::new(),
            },
            intent: None,
            position: None,
            id: None,
            timeout: None,
            pause_on_hover: None,
        }
    }

    /// Dispatch a toast composed with title, body, and footer slot components.
    pub fn composed<V>(content: impl Fn() -> V + Send + Sync + 'static) -> Self
    where
        V: IntoView + Send + 'static,
    {
        Self {
            content: ToastContent::View(StoredValue::new(Box::new(move || content().into_any()))),
            intent: None,
            position: None,
            id: None,
            timeout: None,
            pause_on_hover: None,
        }
    }

    pub fn body(mut self, body: impl Into<String>) -> Self {
        if let ToastContent::Text { body: slot, .. } = &mut self.content {
            *slot = Some(body.into());
        }
        self
    }

    pub fn intent(mut self, intent: FeedbackIntent) -> Self {
        self.intent = Some(intent);
        self
    }

    pub fn position(mut self, position: ToastStackPosition) -> Self {
        self.position = Some(position);
        self
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn timeout(mut self, timeout: i32) -> Self {
        self.timeout = Some(timeout);
        self
    }

    pub fn pause_on_hover(mut self, pause_on_hover: bool) -> Self {
        self.pause_on_hover = Some(pause_on_hover);
        self
    }

    pub fn footer_action(self, label: impl Into<String>, dismiss: bool) -> Self {
        self.footer_action_callback(label, dismiss, None)
    }

    pub fn footer_action_callback(
        mut self,
        label: impl Into<String>,
        dismiss: bool,
        on_click: Option<Callback<()>>,
    ) -> Self {
        if let ToastContent::Text { footer_actions, .. } = &mut self.content {
            footer_actions.push(ToastAction {
                label: label.into(),
                dismiss,
                on_click,
            });
        }
        self
    }
}

pub type ToastId = String;

#[derive(Clone)]
pub(crate) enum ToastRecordContent {
    Text {
        title: String,
        body: Option<String>,
        footer_actions: Vec<ToastAction>,
    },
    View(StoredValue<Box<dyn Fn() -> AnyView + Send + Sync>>),
}

#[derive(Clone)]
pub(crate) struct ToastRecord {
    pub id: String,
    pub content: ToastRecordContent,
    pub intent: FeedbackIntent,
    pub position: ToastStackPosition,
    pub timeout: i32,
    pub pause_on_hover: bool,
    pub queued: bool,
}

#[derive(Clone, Copy)]
pub struct ToasterInjection {
    toasts: RwSignal<Vec<ToastRecord>>,
    config: StoredValue<ToasterConfig>,
}

impl ToasterInjection {
    pub fn expect_context() -> Self {
        expect_context::<Self>()
    }

    pub fn new(config: ToasterConfig) -> Self {
        Self {
            toasts: RwSignal::new(Vec::new()),
            config: StoredValue::new(config),
        }
    }

    pub fn config(&self) -> ToasterConfig {
        self.config.get_value()
    }

    pub fn show(&self, message: impl Into<String>) -> ToastId {
        self.dispatch(ToastOptions::new(message))
    }

    pub fn dispatch(&self, options: ToastOptions) -> ToastId {
        let config = self.config.get_value();
        let id = options
            .id
            .clone()
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

        if self.toasts.with(|list| list.iter().any(|t| t.id == id)) {
            return id;
        }

        let visible_count = self
            .toasts
            .with(|list| list.iter().filter(|t| !t.queued).count());
        let queued = visible_count >= config.limit as usize;

        let content = match options.content {
            ToastContent::Text {
                title,
                body,
                footer_actions,
            } => ToastRecordContent::Text {
                title,
                body,
                footer_actions,
            },
            ToastContent::View(view) => ToastRecordContent::View(view),
        };

        let record = ToastRecord {
            id: id.clone(),
            content,
            intent: options.intent.unwrap_or(config.default_intent),
            position: options.position.unwrap_or(config.default_position),
            timeout: options.timeout.unwrap_or(config.default_timeout),
            pause_on_hover: options
                .pause_on_hover
                .unwrap_or(config.default_pause_on_hover),
            queued,
        };

        self.toasts.update(|list| list.push(record));
        id
    }

    pub fn dismiss(&self, id: &str) {
        let config = self.config.get_value();
        self.toasts.update(|list| {
            list.retain(|t| t.id != id);
            let visible = list.iter().filter(|t| !t.queued).count();
            if visible < config.limit as usize {
                if let Some(next) = list.iter_mut().find(|t| t.queued) {
                    next.queued = false;
                }
            }
        });
    }

    /// Dismiss every visible and queued toast for this [`ToasterProvider`] only.
    ///
    /// This does not affect toasts from other toaster instances on the page.
    pub fn dismiss_all(&self) {
        self.toasts.set(Vec::new());
    }

    pub(crate) fn visible_by_position(&self, position: ToastStackPosition) -> Vec<ToastRecord> {
        self.toasts.with(|list| {
            list.iter()
                .filter(|t| !t.queued && t.position == position)
                .cloned()
                .collect()
        })
    }

    pub(crate) fn has_visible(&self) -> bool {
        self.toasts.with(|list| list.iter().any(|t| !t.queued))
    }

    fn dismiss_callback(injection: ToasterInjection) -> Callback<String> {
        Callback::new(move |id: String| injection.dismiss(&id))
    }
}

#[component]
fn ToastPositionStack(
    position: ToastStackPosition,
    injection: ToasterInjection,
    inline: bool,
    offset: ToastOffset,
) -> impl IntoView {
    let inline_class = if inline {
        " orbital-toast-stack--inline"
    } else {
        ""
    };
    let has_toasts = Signal::derive(move || !injection.visible_by_position(position).is_empty());
    let on_dismiss = ToasterInjection::dismiss_callback(injection);

    view! {
        <Show when=move || has_toasts.get()>
            <div
                class=move || {
                    format!(
                        "orbital-toast-stack orbital-toast-stack--{}{inline_class}",
                        position.as_str()
                    )
                }
                data-orbital-toast-position=move || position.as_str()
                style=move || {
                    format!(
                        "--orbital-toast-offset-x: {}px; --orbital-toast-offset-y: {}px;",
                        offset.horizontal, offset.vertical
                    )
                }
            >
                <For
                    each=move || injection.visible_by_position(position)
                    key=|record| record.id.clone()
                    let:record
                >
                    <BaseToastContainer record=record on_dismiss=on_dismiss />
                </For>
            </div>
        </Show>
    }
}

#[component]
fn ToastStackLayer(
    injection: ToasterInjection,
    inline: bool,
    offset: ToastOffset,
) -> impl IntoView {
    view! {
        {ToastStackPosition::ALL.into_iter().map(|position| {
            view! {
                <ToastPositionStack
                    position=position
                    injection=injection
                    inline=inline
                    offset=offset
                />
            }
        }).collect_view()}
    }
}

#[component]
pub fn BaseToastStack() -> impl IntoView {
    let injection = ToasterInjection::expect_context();
    let config = injection.config();
    let has_messages = Signal::derive(move || injection.has_visible());

    if config.inline {
        view! {
            <Show when=move || has_messages.get()>
                <ToastStackLayer injection=injection inline=config.inline offset=config.offset />
            </Show>
        }
        .into_any()
    } else {
        view! {
            <ThemedPortal immediate=has_messages>
                <ToastStackLayer injection=injection inline=config.inline offset=config.offset />
            </ThemedPortal>
        }
        .into_any()
    }
}

#[component]
pub fn BaseToasterProvider(toaster_config: ToasterConfig, children: Children) -> impl IntoView {
    let injection = ToasterInjection::new(toaster_config);

    view! {
        <leptos::context::Provider value=injection>
            {children()}
        </leptos::context::Provider>
    }
}
