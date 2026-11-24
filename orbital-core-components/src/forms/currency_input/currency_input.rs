use std::sync::atomic::{AtomicUsize, Ordering};

use leptos::prelude::*;
use orbital_base_components::{FieldInjection, Handler};
use orbital_macros::component_doc;

use super::format::{
    edit_text, format_amount, parse_amount, round_canonical, CurrencyCode, SymbolSide,
};
use super::types::{CurrencyInputAppearance, CurrencyInputBind};
use crate::flex::{Flex, FlexAlign};
use crate::forms::input::{
    Input, InputAppearance, InputBind, InputEvents, InputPrefix, InputSuffix,
};
use crate::forms::select::{Select, SelectAppearance, SelectBind};

static NEXT_SELECT_ID: AtomicUsize = AtomicUsize::new(0);

/// CurrencyInput edits a money amount — prices, budgets, invoice lines — with the currency
/// symbol on the side the locale puts it, digits grouped and the decimal mark placed the
/// locale's way, and typed or pasted text cleaned up.
///
/// While the field has focus it shows the plain number with the locale's decimal mark, so
/// the caret never jumps over a group separator. On blur it formats the amount, padded to
/// the currency's minor digits. An edit it can't read is undone and the last good text stays.
///
/// # API notes
///
/// - Bind a `String` via [`CurrencyInputBind`]. The value is always canonical: an optional
///   `-`, digits and a `.` decimal point (`"1234.56"`), whatever the locale. Parse it with
///   your decimal type; it is empty while the field has no number.
/// - Set the locale in [`CurrencyInputAppearance`]; `currency` is an `RwSignal<CurrencyCode>`
///   so the optional currency select can change it. Both set the symbol, its side and the
///   decimals allowed (JPY has none).
/// - Pass `currencies` to show a currency [`Select`](crate::Select) beside the amount.
///   Changing the code rounds the amount to the new currency's minor digits.
/// - Pasted symbols, ISO codes, spaces and the locale's group separator are dropped.
/// - Built-in locales: en-US, en-GB, de-DE, fr-FR, de-CH, pt-BR, ja-JP and en-IN.
///   [`CurrencyLocale::from_tag`] falls back to en-US for anything else.
///
/// # When to use
///
/// - Price, budget and invoice amount fields
/// - Forms that take amounts in more than one currency or locale
///
/// # Usage
///
/// 1. Bind a `String` signal via [`CurrencyInputBind`].
/// 2. Set `currency` and, in [`CurrencyInputAppearance`], the `locale`.
/// 3. Wrap in [`Field`](crate::Field) when the control needs a label or hint.
///
/// # Best Practices
///
/// ## Do's
///
/// * Keep the stored value canonical and format only for display
/// * Wrap in [`Field`](crate::Field) for an accessible label
///
/// ## Don'ts
///
/// * Do not use for plain counts — use [`NumericStepper`](crate::NumericStepper)
/// * Do not store the formatted text; it changes with the locale
/// * Do not put `data-testid` on the component — wrap with a native element
///
/// # Examples
///
/// ## Basic
/// en-US, USD: symbol first, comma groups, dot decimals.
/// <!-- preview -->
/// ```rust
/// use crate::{CurrencyCode, CurrencyInput, Field};
/// let price = RwSignal::new("1234.56".to_string());
/// view! {
///     <div data-testid="currency-input-basic">
///         <Field label="Price">
///             <CurrencyInput bind=price currency=RwSignal::new(CurrencyCode::USD) />
///         </Field>
///     </div>
/// }
/// ```
///
/// ## Locale
/// de-DE, EUR: symbol after the number, dot groups, comma decimals.
/// <!-- preview -->
/// ```rust
/// use crate::{CurrencyCode, CurrencyInput, CurrencyInputAppearance, CurrencyLocale, Field};
/// let amount = RwSignal::new("1234.56".to_string());
/// view! {
///     <div data-testid="currency-input-locale">
///         <Field label="Betrag">
///             <CurrencyInput
///                 bind=amount
///                 currency=RwSignal::new(CurrencyCode::EUR)
///                 appearance=CurrencyInputAppearance {
///                     locale: Signal::from(CurrencyLocale::DeDe),
///                     ..Default::default()
///                 }
///             />
///         </Field>
///     </div>
/// }
/// ```
///
/// ## Multi-currency
/// Set `currencies` to show a currency select; the symbol and decimals follow it.
/// <!-- preview -->
/// ```rust
/// use crate::{CurrencyCode, CurrencyInput, Field};
/// let amount = RwSignal::new("1234.56".to_string());
/// let currency = RwSignal::new(CurrencyCode::USD);
/// view! {
///     <div data-testid="currency-input-multi">
///         <Field label="Amount">
///             <CurrencyInput
///                 bind=amount
///                 currency=currency
///                 currencies=CurrencyCode::ALL.to_vec()
///             />
///         </Field>
///     </div>
/// }
/// ```
///
/// ## Negative amounts
/// Accept a leading minus for refunds and adjustments.
/// <!-- preview -->
/// ```rust
/// use crate::{CurrencyCode, CurrencyInput, CurrencyInputAppearance, Field};
/// let amount = RwSignal::new("-25".to_string());
/// view! {
///     <div data-testid="currency-input-negative">
///         <Field label="Adjustment">
///             <CurrencyInput
///                 bind=amount
///                 currency=RwSignal::new(CurrencyCode::USD)
///                 appearance=CurrencyInputAppearance {
///                     allow_negative: true,
///                     ..Default::default()
///                 }
///             />
///         </Field>
///     </div>
/// }
/// ```
///
/// ## Disabled
/// Shows the formatted amount without allowing edits.
/// <!-- preview -->
/// ```rust
/// use crate::{CurrencyCode, CurrencyInput, CurrencyInputAppearance, Field};
/// let amount = RwSignal::new("99".to_string());
/// view! {
///     <div data-testid="currency-input-disabled">
///         <Field label="Total">
///             <CurrencyInput
///                 bind=amount
///                 currency=RwSignal::new(CurrencyCode::USD)
///                 appearance=CurrencyInputAppearance {
///                     disabled: Signal::from(true),
///                     ..Default::default()
///                 }
///             />
///         </Field>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Inputs",
    preview_slug = "currency-input",
    preview_label = "Currency Input",
    preview_icon = icondata::AiDollarOutlined,
)]
#[component]
pub fn CurrencyInput(
    /// Canonical amount binding, field identity.
    #[prop(optional, into)]
    bind: CurrencyInputBind,
    /// Locale, minus sign, placeholder, visual size and disabled state.
    #[prop(optional, into)]
    appearance: CurrencyInputAppearance,
    /// Currency of the amount; the currency select writes to it.
    #[prop(optional, into)]
    currency: RwSignal<CurrencyCode>,
    /// When set, shows a currency select beside the amount offering these codes.
    #[prop(optional, into)]
    currencies: Option<Vec<CurrencyCode>>,
    /// Extra CSS class names merged onto the input wrapper.
    #[prop(optional, into)]
    class: MaybeProp<String>,
) -> impl IntoView {
    let CurrencyInputBind { value, id, name } = bind;
    let CurrencyInputAppearance {
        locale,
        allow_negative,
        placeholder,
        size,
        disabled,
    } = appearance;

    let (id, name) = FieldInjection::use_id_and_name(id, name);
    let value = StoredValue::new(value);

    let blurred_text = move |canonical: &str| {
        format_amount(canonical, locale.get_untracked(), currency.get_untracked())
    };
    let text = RwSignal::new(blurred_text(&value.get_value().get_untracked()));
    let focused = RwSignal::new(false);
    // Edit text of the last accepted edit, restored when an edit is rejected.
    let last_edit = StoredValue::new(edit_text(
        &value.get_value().get_untracked(),
        locale.get_untracked(),
    ));

    // Typed or pasted text: normalise it, or put back the last good text.
    Effect::new(move |_| {
        let raw = text.get();
        if !focused.get_untracked() {
            return;
        }
        match parse_amount(
            &raw,
            locale.get_untracked(),
            currency.get_untracked(),
            allow_negative,
        ) {
            Some(parsed) => {
                last_edit.set_value(parsed.edit.clone());
                if parsed.edit != raw {
                    text.set(parsed.edit);
                }
                if value.get_value().get_untracked() != parsed.canonical {
                    value.get_value().set(parsed.canonical);
                }
            }
            None => {
                let prev = last_edit.get_value();
                if prev != raw {
                    text.set(prev);
                }
            }
        }
    });

    // Outside changes: a new bound value, currency or locale.
    Effect::new(move |_| {
        let loc = locale.get();
        let cur = currency.get();
        let canonical = value.get_value().get();
        let rounded = round_canonical(&canonical, cur);
        if rounded != canonical {
            value.get_value().set(rounded);
            return;
        }
        if focused.get_untracked() {
            let same = parse_amount(&text.get_untracked(), loc, cur, allow_negative)
                .is_some_and(|p| p.canonical == canonical);
            if !same {
                let edit = edit_text(&canonical, loc);
                last_edit.set_value(edit.clone());
                text.set(edit);
            }
        } else {
            let shown = format_amount(&canonical, loc, cur);
            if text.get_untracked() != shown {
                text.set(shown);
            }
        }
    });

    let on_focus = move |_| {
        focused.set(true);
        let edit = edit_text(&value.get_value().get_untracked(), locale.get_untracked());
        last_edit.set_value(edit.clone());
        text.set(edit);
    };
    let on_blur = move |_| {
        focused.set(false);
        let shown = blurred_text(&value.get_value().get_untracked());
        text.set(shown);
    };

    let placeholder = Signal::derive(move || {
        placeholder
            .get()
            .unwrap_or_else(|| format_amount("0", locale.get(), currency.get()))
    });
    let side = Memo::new(move |_| locale.get().symbol_side());
    let symbol = move || currency.get().symbol();

    let field = move || {
        let events = InputEvents {
            on_focus: Some(Handler::on(on_focus)),
            on_blur: Some(Handler::on(on_blur)),
            ..Default::default()
        };
        let input_appearance = InputAppearance {
            placeholder: MaybeProp::derive(move || Some(placeholder.get())),
            size,
            disabled,
            ..Default::default()
        };
        let bind = InputBind {
            value: text.into(),
            id: id.into(),
            name: name.into(),
            ..Default::default()
        };
        match side.get() {
            SymbolSide::Prefix => view! {
                <Input bind=bind appearance=input_appearance events=events class=class>
                    <InputPrefix slot>{symbol}</InputPrefix>
                </Input>
            }
            .into_any(),
            SymbolSide::Suffix => view! {
                <Input bind=bind appearance=input_appearance events=events class=class>
                    <InputSuffix slot>{symbol}</InputSuffix>
                </Input>
            }
            .into_any(),
        }
    };

    let select = currencies.map(|codes| {
        let code_text = RwSignal::new(currency.get_untracked().code().to_string());
        Effect::new(move |_| {
            if let Some(code) = CurrencyCode::from_code(&code_text.get()) {
                if currency.get_untracked() != code {
                    currency.set(code);
                }
            }
        });
        Effect::new(move |_| {
            let code = currency.get().code().to_string();
            if code_text.get_untracked() != code {
                code_text.set(code);
            }
        });
        let select_id = format!(
            "orbital-currency-input-select-{}",
            NEXT_SELECT_ID.fetch_add(1, Ordering::Relaxed)
        );
        view! {
            <Select
                bind=SelectBind {
                    value: code_text.into(),
                    id: MaybeProp::from(select_id),
                    ..Default::default()
                }
                appearance=SelectAppearance { disabled, ..Default::default() }
                aria_label="Currency"
            >
                {codes
                    .into_iter()
                    .map(|c| view! { <option value=c.code()>{c.code()}</option> })
                    .collect_view()}
            </Select>
        }
    });

    view! {
        <Flex align=FlexAlign::Center>
            {field}
            {select}
        </Flex>
    }
}
