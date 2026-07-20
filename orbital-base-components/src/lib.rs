//! Headless base primitives for Orbital — structure, semantics, and a11y without theme.

#![recursion_limit = "512"]

mod callback;

pub mod accordion;
pub mod anchor;
pub mod app_bar;
pub mod avatar;
pub mod back_to_top;
pub mod button;
pub mod card;
pub mod code;
pub mod collection;
pub mod data;
pub mod demo_box;
pub mod divider;
pub mod drawer;
pub mod feedback;
pub mod flex;
pub mod floating_button;
pub mod form;
pub mod grid;
pub mod icon;
pub mod layout;
pub mod loading_bar;
pub mod material;
pub mod navigation;
pub mod overlay;
pub mod pagination;
pub mod preview_render_mode;
pub mod rating;
pub mod signals;
pub mod space;
pub mod spacing;
pub mod stack;
pub mod tab_list;
pub mod table;
pub mod tag;
pub mod tokens;
pub mod tree;
pub mod upload;

pub use accordion::{AccordionHeader, AccordionInjection, BaseAccordion, BaseAccordionItem};
pub use anchor::{AnchorInjection, BaseAnchor, BaseAnchorLink, OffsetTarget};
pub use app_bar::{AppBarDensity, AppBarPosition, BaseAppBar};
pub use avatar::{color_hash, initials_from_name, AvatarColor, AvatarShape, BaseAvatar};
pub use back_to_top::BaseBackToTop;
pub use button::{
    BaseButton, BaseCompoundButton, ButtonAppearance, ButtonRef, ButtonShape, ButtonSize,
    ButtonType, CompoundButtonIconPosition,
};
pub use callback::Handler;
pub use card::BaseCardButtonArea;
pub use code::BaseCode;
pub use collection::state::CollectionStateInjection;
pub use data::{
    AvatarGroupLayout, AvatarGroupSize, BaseAvatarGroup, BaseBreadcrumb, BaseBreadcrumbButton,
    BaseBreadcrumbDivider, BaseBreadcrumbItem, BaseImage, BaseLink, BaseList, BaseListItem,
    ImageFit, ImageShape, ListNavigationMode, ListSelectionMode,
};
pub use demo_box::DemoBox;
pub use divider::BaseDivider;
pub use drawer::{
    drawer_size_css, BaseDrawerBody, BaseDrawerHeader, BaseDrawerHeaderTitle, BaseInlineDrawer,
    BaseOverlayDrawer, DrawerHeaderTitleAction, DrawerModalType, DrawerPosition, DrawerSize,
};
pub use feedback::{BaseCounterBadge, BasePresenceBadge, PresenceBadgeSize, PresenceStatus};
pub use flex::{BaseFlex, FlexAlign, FlexGap, FlexJustify, FlexWrap};
pub use floating_button::BaseFloatingButton;
pub use floating_button::FloatingButtonVariant;
pub use form::{
    build_month_grid, compose_time_unix, event_html_element, format_datetime, format_time_value,
    format_unix, get_dropdown_action_from_key, input_event_value, is_datetime_out_of_range,
    is_day_disabled, listbox_keyboard_event, move_all, move_checked, new_field_id,
    normalize_reference_date, now_time, parse_datetime, parse_time_input, parse_to_unix,
    selectable_ids, selected_count, textarea_event_value, to_panel_time, toggle_all,
    use_active_descendant, ActiveDescendantController, AutoCompleteSize, BaseButtonGroup,
    BaseCalendar, BaseCheckbox, BaseColorPicker, BaseDatePicker, BaseField, BaseInfoLabel,
    BaseInput, BaseLabel, BaseListbox, BaseNumericStepper, BaseRadio, BaseRadioGroup, BaseSelect,
    BaseSlider, BaseSliderLabel, BaseSwatchPicker, BaseSwatchPickerItem, BaseSwitch, BaseTextarea,
    BaseTimePicker, CheckboxSize, Color, ColorBind, ComboboxSize, DatePickerRule,
    DatePickerRuleTrigger, DatetimeError, DatetimeFormat, DatetimeTimezone, DropdownAction,
    FieldInjection, FieldOrientation, FieldValidationState, FormBind, GridDay, GridDayKind,
    InfoLabelInfo, InputRef, InputRule, InputRuleTrigger, InputSize, InputType, LabelSize,
    LabelWeight, ListboxInjection, MonthButtonRenderProps, MonthButtonRenderer, NumericStepperRule,
    NumericStepperRuleTrigger, OptionBind, OrbitalDateTime, PickerShortcut, PickerShortcutsBar,
    RadioGroupRule, RadioGroupRuleTrigger, RatingRule, RatingRuleTrigger, Rule,
    RuleValueWithUntracked, SelectRule, SelectRuleTrigger, SelectSize, SliderRule,
    SliderRuleTrigger, SwatchPickerInjection, SwatchPickerLayout, SwatchPickerShape,
    SwatchPickerSize, SwitchRule, SwitchRuleTrigger, TextareaRef, TextareaResize, TextareaRule,
    TextareaRuleTrigger, TextareaSize, ToDataValue, ToIso8601, ToUnixSeconds, TransferListItem,
    TryFromDataValue, TryFromIso8601, TryFromUnixSeconds,
};
pub use grid::{use_grid, BaseGrid, BaseGridItem, GridInjection};
pub use icon::BaseIcon;
pub use layout::{
    AppBarInset, BaseLayout, BaseLayoutBody, BaseLayoutHeaderInset, BaseLayoutMain,
    BaseLayoutSidebar, LayoutPosition,
};
pub use loading_bar::{BaseLoadingBarProvider, LoadingBarInjection};
pub use material::{BaseMaterial, MaterialCorners, MaterialElevation, MaterialVariant};
pub use navigation::carousel::{
    BaseCarousel, BaseCarouselSlide, BaseCarouselStepper, BaseCarouselViewport, CarouselState,
    CarouselStateInjection, CarouselStepperLayout,
};
pub use navigation::overflow::{BaseOverflow, OverflowAxes, OverflowChangeData, OverflowDirection};
pub use navigation::toolbar::{BaseToolbar, ToolbarSize};
pub use navigation::{
    BaseFloatingActionsMenuItem, BaseNavigation, BaseNavigationItem, BaseNavigationItemConfig,
    FloatingActionsMenuDirection, FloatingActionsMenuInjection, FloatingActionsMenuTooltipSide,
    NavigationCategoryInjection, NavigationDensity, NavigationInjection, NavigationMode,
    NavigationSubItemGroupInjection,
};
pub use overlay::{
    arrow_style, build_anchor_arrow, AnchorWidth, AnchoredOverlay, AnchoredOverlayConfig,
    AnchoredPanel, AnchoredPositioner, AnchoredSurface, BackdropMode, BadgeAppearance, BadgeColor,
    BadgeSize, BaseBackdrop, BaseBadge, BaseDialog, BaseDialogActions, BaseDialogBody,
    BaseDialogContent, BaseDialogSurface, BaseDialogTitle, BaseMenu, BaseMenuItem, BaseMessageBar,
    BaseMessageBarActions, BaseMessageBarBody, BaseMessageBarTitle, BasePopover, BaseSkeleton,
    BaseSkeletonItem, BaseSpinner, BaseToast, BaseToastBody, BaseToastFooter, BaseToastStack,
    BaseToastTitle, BaseToastTrigger, BaseToasterProvider, BaseTooltip, DialogDismiss,
    DialogInjection, FeedbackIntent, FocusTrap, MenuInjection, MenuKeyboardRegion,
    MessageBarLayout, OpenBind, OverlayAppearance, OverlayArrowInjection, OverlayArrowMode,
    OverlayDismiss, OverlayLifecycle, OverlayPanelSize, OverlayPlacementInjection, OverlayTrigger,
    OverlayTriggerType, Placement, PopoverEvents, SkeletonInjection, SkeletonItemShape,
    SkeletonItemSize, SpinnerSize, ThemedPortal, ToastAction, ToastId, ToastItemContext,
    ToastOffset, ToastOptions, ToastStackPosition, ToasterConfig, ToasterInjection,
    UseOverlayVisibility, DEFAULT_TOAST_LIMIT, DEFAULT_TOAST_TIMEOUT_MS,
};
pub use pagination::{pagination_items, PaginationItem};
pub use preview_render_mode::PreviewRenderMode;
pub use rating::{filled_fraction, BaseRatingItem, RatingColor, RatingSize};
pub use space::BaseSpace;
pub use stack::BaseStack;
pub use table::{
    BaseTable, BaseTableBody, BaseTableCell, BaseTableCellLayout, BaseTableHeader,
    BaseTableHeaderCell, BaseTableRow,
};
pub use tag::{
    BaseInteractionTag, BaseInteractionTagPrimary, BaseSecondaryActionTag, BaseTag,
    BaseTagDismissButton, BaseTagGroup, InteractionTagInjection, TagAppearance, TagGroupInjection,
    TagSize,
};
pub use tree::{
    apply_sibling_dom_reorder, base_tree_item_layout, install_tree_drag_listeners, BaseSubtree,
    BaseTree, BaseTreeCollapseSlot, BaseTreeConfig, BaseTreeItem, BaseTreeItemAside,
    BaseTreeItemCheckbox, BaseTreeItemIconAfter, BaseTreeItemIconBefore, BaseTreeItemLayout,
    BaseTreeItemRow, BaseTreeRoot, ExpansionTrigger, SubtreeInjection, TreeDragListenerHandle,
    TreeDragState, TreeDropPosition, TreeExpansionState, TreeFocusState, TreeInjection,
    TreeItemDomRegistry, TreeItemInjection, TreeItemRegistry, TreeItemType, TreeRegistryEntry,
    TreeSelectionMode, TreeSelectionState, TreeSize, TreeState, TreeStateInjection,
};
pub use upload::{BaseUpload, BaseUploadDragger, UploadFileList};

/// One documented component prop for preview catalog Properties tabs.
#[derive(Clone, Copy, Debug)]
pub struct ComponentPropDoc {
    pub name: &'static str,
    pub type_name: &'static str,
    pub description: &'static str,
}
pub use signals::{ComponentRef, SignalModel, WebSysCallback};
pub use spacing::{SpacingHorizontal, SpacingInset, SpacingVertical};
pub use tab_list::{BaseTab, BaseTabList};
pub use tokens::{
    BorderRadius, FontFamily, FontSize, FontWeight, IconSize, LineHeight, MotionCurve,
    MotionDuration, Shadow, StrokeWidth, ThemeColor,
};
