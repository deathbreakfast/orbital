use leptos::{children::ToChildren, ev, prelude::*};
use palette::{FromColor, Hsl, Hsv, IntoColor, RgbHue, Srgb};
use wasm_bindgen::JsCast;

use crate::{
    form::{input_event_value, FieldInjection, FormBind, OptionBind},
    overlay::{positioning::AnchorWidth, AnchoredPanel, AnchoredPositioner, Placement},
};

/// Canonical color payload for Orbital color pickers.
#[derive(Clone, Debug)]
pub enum Color {
    RGB(Srgb),
    HSV(Hsv),
    HSL(Hsl),
}

impl Default for Color {
    fn default() -> Self {
        Self::RGB(Srgb::new(0.0, 0.0, 0.0))
    }
}

impl Color {
    pub fn to_hsv(&self) -> Hsv {
        match self {
            Self::RGB(rgb) => (*rgb).into_color(),
            Self::HSV(hsv) => *hsv,
            Self::HSL(hsl) => (*hsl).into_color(),
        }
    }

    pub fn update_from_hsv(&mut self, hsv: Hsv) {
        match self {
            Self::RGB(rgb) => *rgb = hsv.into_color(),
            Self::HSV(current) => *current = hsv,
            Self::HSL(hsl) => *hsl = hsv.into_color(),
        }
    }

    pub fn to_rgb_u8(&self) -> Srgb<u8> {
        let rgb: Srgb = self.to_hsv().into_color();
        Srgb::<u8>::from_format(rgb)
    }

    pub fn to_css_rgb(&self) -> String {
        let rgb = self.to_rgb_u8();
        format!("rgb({}, {}, {})", rgb.red, rgb.green, rgb.blue)
    }
}

impl From<Srgb> for Color {
    fn from(value: Srgb) -> Self {
        Self::RGB(value)
    }
}

impl From<Hsv> for Color {
    fn from(value: Hsv) -> Self {
        Self::HSV(value)
    }
}

impl From<Hsl> for Color {
    fn from(value: Hsl) -> Self {
        Self::HSL(value)
    }
}

#[derive(Clone)]
pub enum ColorBind {
    Optional(OptionBind<Color>),
    Required(FormBind<Color>),
}

impl ColorBind {
    pub fn get(&self) -> Option<Color> {
        match self {
            Self::Optional(bind) => bind.get(),
            Self::Required(bind) => Some(bind.get()),
        }
    }

    pub fn get_untracked(&self) -> Option<Color> {
        match self {
            Self::Optional(bind) => bind.get_untracked(),
            Self::Required(bind) => Some(bind.get_untracked()),
        }
    }

    pub fn set(&self, value: Color) {
        match self {
            Self::Optional(bind) => bind.set(Some(value)),
            Self::Required(bind) => bind.set(value),
        }
    }
}

impl Default for ColorBind {
    fn default() -> Self {
        Self::Required(FormBind::from(Color::default()))
    }
}

impl From<OptionBind<Color>> for ColorBind {
    fn from(value: OptionBind<Color>) -> Self {
        Self::Optional(value)
    }
}

impl From<FormBind<Color>> for ColorBind {
    fn from(value: FormBind<Color>) -> Self {
        Self::Required(value)
    }
}

impl From<RwSignal<Option<Color>>> for ColorBind {
    fn from(value: RwSignal<Option<Color>>) -> Self {
        Self::Optional(value.into())
    }
}

impl From<RwSignal<Color>> for ColorBind {
    fn from(value: RwSignal<Color>) -> Self {
        Self::Required(value.into())
    }
}

/// Headless color picker with SV plane and hue slider.
#[component]
pub fn BaseColorPicker(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] id: MaybeProp<String>,
    #[prop(optional, into)] name: MaybeProp<String>,
    #[prop(optional, into)] value: ColorBind,
    #[prop(optional, into)] disabled: Signal<bool>,
) -> impl IntoView {
    let (id, name) = FieldInjection::use_id_and_name(id, name);
    let value = StoredValue::new(value);
    let open = RwSignal::new(false);
    let dragging_sv = RwSignal::new(false);
    let hsv = RwSignal::new(
        value
            .get_value()
            .get_untracked()
            .map(|c| c.to_hsv())
            .unwrap_or_default(),
    );

    Effect::new(move |_| {
        if open.get() {
            return;
        }
        if let Some(color) = value.get_value().get() {
            hsv.set(color.to_hsv());
        }
    });

    let update_model = move |next: Hsv| {
        hsv.set(next);
        let mut current = value
            .get_value()
            .get_untracked()
            .unwrap_or_else(|| Color::from(Srgb::new(0.0, 0.0, 0.0)));
        current.update_from_hsv(next);
        value.get_value().set(current);
    };

    let on_sv_interaction = move |event: ev::MouseEvent| {
        let Some(target) = event.current_target() else {
            return;
        };
        let Ok(element) = target.dyn_into::<web_sys::Element>() else {
            return;
        };
        let rect = element.get_bounding_client_rect();
        if rect.width() <= 0.0 || rect.height() <= 0.0 {
            return;
        }
        let x = (f64::from(event.client_x()) - rect.left()).clamp(0.0, rect.width());
        let y = (f64::from(event.client_y()) - rect.top()).clamp(0.0, rect.height());
        let saturation = (x / rect.width()) as f32;
        let value_level = (1.0 - y / rect.height()) as f32;
        let mut next = hsv.get_untracked();
        next.saturation = saturation;
        next.value = value_level;
        update_model(next);
    };

    let open_panel = move |_| {
        if disabled.get_untracked() {
            return;
        }
        open.set(true);
    };

    view! {
        <AnchoredPositioner panel=AnchoredPanel {
            show: open.read_only().into(),
            width: Some(AnchorWidth::Target),
            placement: Placement::BottomStart,
            auto_height: false,
            arrow: None,
            motion: None,
            children: ToChildren::to_children(move || {
                let sv_cursor_style = Signal::derive(move || {
                    let current = hsv.get();
                    format!(
                        "left: {}%; top: {}%;",
                        f64::from(current.saturation) * 100.0,
                        (1.0 - f64::from(current.value)) * 100.0
                    )
                });
                let panel_bg = Signal::derive(move || {
                    let hue = hsv.get().hue;
                    let rgb = Srgb::from_color(Hsv::new(hue, 1.0, 1.0)).into_format::<u8>();
                    format!("rgb({}, {}, {})", rgb.red, rgb.green, rgb.blue)
                });

                view! {
                    <div class="orbital-color-picker-panel" role="dialog" aria-label="Color picker">
                        <div
                            class="orbital-color-picker-panel__sv"
                            style:background-color=move || panel_bg.get()
                            on:mousedown=move |ev| {
                                dragging_sv.set(true);
                                on_sv_interaction(ev);
                            }
                            on:mousemove=move |ev| {
                                if dragging_sv.get() {
                                    on_sv_interaction(ev);
                                }
                            }
                            on:mouseup=move |_| dragging_sv.set(false)
                            on:mouseleave=move |_| dragging_sv.set(false)
                        >
                            <div class="orbital-color-picker-panel__sv-white"></div>
                            <div class="orbital-color-picker-panel__sv-black"></div>
                            <span class="orbital-color-picker-panel__sv-cursor" style=move || sv_cursor_style.get()></span>
                        </div>
                        <input
                            type="range"
                            min="0"
                            max="360"
                            step="1"
                            class="orbital-color-picker-panel__hue"
                            on:input=move |ev| {
                                if let Some(raw) = input_event_value(&ev) {
                                    if let Ok(next_hue) = raw.parse::<f32>() {
                                        let mut next = hsv.get_untracked();
                                        next.hue = RgbHue::from_degrees(next_hue);
                                        if next.saturation == 0.0 && next.value == 0.0 {
                                            next.saturation = 1.0;
                                            next.value = 1.0;
                                        }
                                        update_model(next);
                                    }
                                }
                            }
                            on:change=move |ev| {
                                if let Some(raw) = input_event_value(&ev) {
                                    if let Ok(next_hue) = raw.parse::<f32>() {
                                        let mut next = hsv.get_untracked();
                                        next.hue = RgbHue::from_degrees(next_hue);
                                        if next.saturation == 0.0 && next.value == 0.0 {
                                            next.saturation = 1.0;
                                            next.value = 1.0;
                                        }
                                        update_model(next);
                                    }
                                }
                            }
                        />
                        <div class="orbital-color-picker-panel__actions">
                            <button type="button" class="orbital-color-picker-panel__action" on:click=move |_| open.set(false)>
                                "Close"
                            </button>
                        </div>
                    </div>
                }
            }),
        }>
            <div class=move || {
                let mut parts = vec!["orbital-color-picker".to_string()];
                if disabled.get() {
                    parts.push("orbital-color-picker--disabled".to_string());
                }
                if let Some(extra) = class.get() {
                    if !extra.is_empty() {
                        parts.push(extra);
                    }
                }
                parts.join(" ")
            }>
                <button
                    id=id
                    type="button"
                    class="orbital-color-picker__trigger"
                    aria-haspopup="dialog"
                    aria-expanded=move || open.get().to_string()
                    disabled=move || disabled.get().then_some("")
                    on:click=open_panel
                >
                    <span
                        class="orbital-color-picker__swatch"
                        style:background-color=move || {
                            value
                                .get_value()
                                .get()
                                .unwrap_or_default()
                                .to_css_rgb()
                        }
                    ></span>
                    <span class="orbital-color-picker__label">
                        {move || {
                            let rgb = value
                                .get_value()
                                .get()
                                .unwrap_or_default()
                                .to_rgb_u8();
                            format!("#{:02X}{:02X}{:02X}", rgb.red, rgb.green, rgb.blue)
                        }}
                    </span>
                </button>
                <input
                    type="hidden"
                    name=move || name.get()
                    value=move || {
                        let rgb = value
                            .get_value()
                            .get()
                            .unwrap_or_default()
                            .to_rgb_u8();
                        format!("#{:02X}{:02X}{:02X}", rgb.red, rgb.green, rgb.blue)
                    }
                />
            </div>
        </AnchoredPositioner>
    }
}
