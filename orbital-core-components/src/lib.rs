#![allow(clippy::needless_update, unused_doc_comments)]
#![recursion_limit = "512"]
//! Themed Orbital UI components for Leptos applications.
//!
//! Builds on [`orbital_base_components`] with Orbital theme tokens, motion, and
//! styled surfaces (buttons, forms, overlays, navigation, and more).

pub mod app_bar;
pub mod button;
pub mod code;
pub mod data;
pub mod divider;
pub mod extensions;
pub mod feedback;
pub mod flex;
pub mod floating_button;
pub mod forms;
pub mod grid;
pub mod icon;
pub mod layout;
pub mod layout_box;
pub mod material;
pub mod rating;
pub mod scroll_area;
pub mod space;
pub mod stack;
pub mod tag;
pub mod tree;

pub mod card;
pub mod navigation;
pub mod overlay;
pub mod pagination;
pub mod tab_list;
pub mod text;

pub use app_bar::{
    AppBar, AppBarDensity, AppBarLeading, AppBarMaterial, AppBarPosition, AppBarTrailing,
};
pub use card::{
    Card, CardButtonArea, CardContent, CardFooter, CardHeader, CardHeaderAction,
    CardHeaderDescription, CardMedia, CardPreview, CardSectionBorder,
};
pub use text::*;

pub use button::{Button, ButtonAppearance, ButtonRef, ButtonShape, ButtonSize, ButtonType};
pub use code::Code;
pub use data::{
    link_styles, Avatar, AvatarColor, AvatarConfig, AvatarGroup, AvatarGroupLayout,
    AvatarGroupSize, AvatarShape, Breadcrumb, BreadcrumbButton, BreadcrumbDivider, BreadcrumbItem,
    Image, ImageConfig, ImageFit, ImageShape, Link, List, ListItem, ListNavigationMode,
    ListSelectionMode, Persona, PersonaConfig, PersonaPrimaryText, PersonaQuaternaryText,
    PersonaSecondaryText, PersonaSize, PersonaTertiaryText, PersonaTextAlignment,
    PersonaTextPosition, Table, TableBody, TableCell, TableCellLayout, TableCellLayoutConfig,
    TableHeader, TableHeaderCell, TableHeaderCellConfig, TableRow,
};
pub use divider::Divider;
pub use extensions::ThemeDensityStepper;
#[cfg(feature = "preview")]
pub use extensions::ThemePreviewMarker;
pub use extensions::{LoadingBarInjection, LoadingBarProvider};
pub use flex::{
    Flex, FlexAlign, FlexGap, FlexJustify, FlexWrap, SpacingHorizontal, SpacingInset,
    SpacingVertical,
};
pub use floating_button::{
    FloatingButton, FloatingButtonColor, FloatingButtonConfig, FloatingButtonSize,
    FloatingButtonVariant,
};
pub use forms::{
    calendar_styles, default_calendar_day, default_calendar_month_button, input_styles,
    ActionMenuButton, ActionMenuItems, AutoComplete, AutoCompleteAppearance, AutoCompleteBind,
    AutoCompleteEvents, AutoCompleteOption, AutoCompleteSize, ButtonGroup, Calendar,
    CalendarAppearance, CalendarBind, CalendarChromeLabels, CalendarDayProps, CalendarDayRenderer,
    CalendarMonthButtonProps, CalendarMonthButtonRenderer, CalendarWeekdayHeader, Checkbox,
    CheckboxSize, Color, ColorBind, ColorPicker, ColorPickerAppearance, ColorPickerBind, Combobox,
    ComboboxAppearance, ComboboxBind, ComboboxOption, ComboboxOptionGroup, ComboboxSize,
    CompoundButton, CompoundButtonIconPosition, DatePicker, DatePickerAppearance, DatePickerBind,
    DatePickerRule, DatePickerRuleTrigger, DatetimeFormat, DatetimeTimezone, Dropdown, Field,
    FieldInjection, FieldOrientation, FieldValidationState, FileList, FormBind, InfoLabel,
    InfoLabelInfo, InfoLabelSize, InfoLabelWeight, Input, InputAppearance, InputBind, InputEvents,
    InputPrefix, InputRef, InputRule, InputRuleTrigger, InputSize, InputSuffix, InputType, Label,
    LabelSize, LabelWeight, MenuButton, NumericStepper, NumericStepperAppearance,
    NumericStepperBind, NumericStepperRule, NumericStepperRuleTrigger, NumericStepperSize,
    OptionBind, PickerShortcut, PickerShortcutsBar, Radio, RadioGroup, RadioGroupBind,
    RadioGroupLayout, RadioGroupRule, RadioGroupRuleTrigger, SearchBox, SearchBoxAppearance,
    SearchBoxBind, SearchBoxEvents, Select, SelectAppearance, SelectBind, SelectRule,
    SelectRuleTrigger, SelectSize, Slider, SliderAppearance, SliderBind, SliderLabel, SliderRule,
    SliderRuleTrigger, SwatchPicker, SwatchPickerItem, SwatchPickerLayout, SwatchPickerShape,
    SwatchPickerSize, Switch, SwitchBind, SwitchLabel, SwitchRule, SwitchRuleTrigger, Textarea,
    TextareaAppearance, TextareaBind, TextareaEvents, TextareaRef, TextareaResize, TextareaRule,
    TextareaRuleTrigger, TextareaSize, TimePicker, TimePickerAppearance, TimePickerBind,
    ToggleButton, TransferList, TransferListChange, TransferListConfig, Upload, UploadConfig,
    UploadDragger,
};
pub use grid::{Grid, GridConfig, GridItem, GridItemConfig};
pub use icon::Icon;
pub use layout_box::Box;
pub use orbital_base_components::DemoBox;
pub use orbital_base_components::{
    use_breakpoint, use_breakpoint_down, use_breakpoint_up, use_media_query,
};
pub use orbital_theme::Breakpoint;
pub use rating::{Rating, RatingColor, RatingDisplay, RatingRule, RatingRuleTrigger, RatingSize};
pub use scroll_area::ScrollArea;
pub use space::{Space, SpaceConfig, SpaceGap};
pub use stack::{Stack, StackConfig};
pub use tag::{
    InteractionTag, InteractionTagPrimary, SecondaryActionTag, Tag, TagAppearance, TagGroup,
    TagPicker, TagPickerBind, TagPickerControl, TagPickerGroup, TagPickerInput, TagPickerOption,
    TagPickerOptionGroup, TagPickerSize, TagSize,
};

pub use feedback::{
    Badge, BadgeAppearance, BadgeColor, BadgeSize, CounterBadge, Dialog, DialogActions, DialogBody,
    DialogContent, DialogDismiss, DialogDismissConfig, DialogSurface, DialogTitle, MessageBar,
    MessageBarActions, MessageBarBody, MessageBarIntent, MessageBarLayout, MessageBarTitle,
    OpenBind, PresenceBadge, PresenceBadgeSize, PresenceStatus, ProgressBar, ProgressBarColor,
    ProgressCircle, ProgressCircleColor, Skeleton, SkeletonItem, SkeletonItemShape,
    SkeletonItemSize, Spinner, SpinnerSize, Toast, ToastAction, ToastBody, ToastDefaultTimeoutMs,
    ToastFooter, ToastId, ToastIntent, ToastOffset, ToastOptions, ToastPosition, ToastTitle,
    ToastTrigger, ToasterInjection, ToasterProvider,
};
pub use layout::{
    AppBarInset, Aside, Content, ContentWithAside, Layout, LayoutHeader, LayoutHeaderInset,
    LayoutMain, LayoutPosition, LayoutSidebar, LayoutSidebarOpen, LayoutSidebarToggle,
    SidebarPresentation, DEFAULT_SIDEBAR_OVERLAY_BREAKPOINT,
};
pub use material::{Material, MaterialCorners, MaterialElevation, MaterialVariant};
pub use navigation::{
    Accordion, AccordionHeader, AccordionItem, AdaptiveMenu, AdaptiveMenuTrigger, Anchor,
    AnchorConfig, AnchorLink, BackToTop, BackToTopLabel, Carousel, CarouselSlide, CarouselState,
    CarouselStateInjection, CarouselStepper, CarouselStepperLayout, CarouselViewport, Drawer,
    DrawerBody, DrawerHeader, DrawerHeaderTitle, DrawerHeaderTitleAction, DrawerModalType,
    DrawerPosition, DrawerSize, FloatingActionsMenu, FloatingActionsMenuConfig,
    FloatingActionsMenuItem, InlineDrawer, Menu, MenuAppearance, MenuConfig, MenuItem, MenuPosition,
    MenuTrigger, MenuTriggerType, Navigation, NavigationAppItem, NavigationBody,
    NavigationCategory, NavigationCategoryHeader, NavigationCollapseToggle, NavigationConfig,
    NavigationDensity, NavigationDivider, NavigationFooter, NavigationHeader, NavigationItem,
    NavigationItemConfig, NavigationMaterial, NavigationMode, NavigationSectionHeader,
    NavigationSubItem, NavigationSubItemGroup, OffsetTarget, Overflow, OverflowAxes,
    OverflowChangeData, OverflowDirection, OverflowMenuItems, OverlayDrawer, Popover,
    PopoverAppearance, PopoverConfig, PopoverLifecycle, PopoverPosition, PopoverSize, PopoverTrigger,
    PopoverTriggerType, SpotlightActions, SpotlightBackdrop, SpotlightBody, SpotlightFooter,
    SpotlightHeader, SpotlightMedia, SpotlightPopover, SpotlightTip, SpotlightTour,
    SpotlightTourInjection, SpotlightTourState, SpotlightTourStep, SpotlightTrigger, Toolbar,
    ToolbarButton, ToolbarSize, Tooltip, TooltipAppearance, TooltipConfig, TooltipPosition,
    DEFAULT_ADAPTIVE_MENU_BREAKPOINT,
};
pub use orbital_base_components::{ExpansionTrigger, TreeSelectionMode};
pub use overlay::{Backdrop, BackdropConfig, OverlayLayerRoot, ThemedPortal};
pub use pagination::{Pagination, PaginationConfig};
#[cfg(feature = "preview")]
pub use scroll_area::{
    ScrollAreaPreview, SCROLLAREA_BEST_PRACTICES, SCROLLAREA_DESCRIPTION, SCROLLAREA_DOC,
    SCROLLAREA_PREVIEW_REGISTRATION, SCROLLAREA_PROPS,
};
pub use tab_list::{Tab, TabList};
pub use tree::{
    RichTree, RichTreeData, Tree, TreeApiRef, TreeAppearance, TreeBehavior, TreeConfig,
    TreeExpansion, TreeItem, TreeItemAside, TreeItemCheckbox, TreeItemCollapse, TreeItemConfig,
    TreeItemIconAfter, TreeItemIconBefore, TreeItemLabelInput, TreeItemLayout, TreeItemType,
    TreeSelection, TreeSize,
};

#[cfg(feature = "preview")]
pub mod preview;

#[cfg(feature = "preview")]
pub use data::{
    AVATAR_PREVIEW_REGISTRATION, BREADCRUMB_PREVIEW_REGISTRATION, IMAGE_PREVIEW_REGISTRATION,
    LINK_PREVIEW_REGISTRATION, LIST_PREVIEW_REGISTRATION, PERSONA_PREVIEW_REGISTRATION,
    TABLE_PREVIEW_REGISTRATION,
};
#[cfg(feature = "preview")]
pub use divider::DIVIDER_PREVIEW_REGISTRATION;
#[cfg(feature = "preview")]
pub use extensions::LOADINGBAR_PREVIEW_REGISTRATION;
#[cfg(feature = "preview")]
pub use feedback::{
    BADGE_PREVIEW_REGISTRATION, DIALOG_PREVIEW_REGISTRATION, MESSAGEBAR_PREVIEW_REGISTRATION,
    PROGRESSBAR_PREVIEW_REGISTRATION, SKELETON_PREVIEW_REGISTRATION, SPINNER_PREVIEW_REGISTRATION,
    TOAST_PREVIEW_REGISTRATION,
};
#[cfg(feature = "preview")]
pub use floating_button::FLOATINGBUTTON_PREVIEW_REGISTRATION;
#[cfg(feature = "preview")]
pub use forms::upload::UPLOAD_PREVIEW_REGISTRATION;
#[cfg(feature = "preview")]
pub use forms::{
    AUTOCOMPLETE_PREVIEW_REGISTRATION, BUTTONGROUP_PREVIEW_REGISTRATION,
    CALENDAR_PREVIEW_REGISTRATION, CHECKBOX_PREVIEW_REGISTRATION, COLORPICKER_PREVIEW_REGISTRATION,
    COMBOBOX_PREVIEW_REGISTRATION, DATEPICKER_PREVIEW_REGISTRATION, DROPDOWN_PREVIEW_REGISTRATION,
    FIELD_PREVIEW_REGISTRATION, INFOLABEL_PREVIEW_REGISTRATION, INPUT_PREVIEW_REGISTRATION,
    LABEL_PREVIEW_REGISTRATION, MENUBUTTON_PREVIEW_REGISTRATION,
    NUMERICSTEPPER_PREVIEW_REGISTRATION, RADIO_PREVIEW_REGISTRATION,
    SEARCHBOX_PREVIEW_REGISTRATION, SELECT_PREVIEW_REGISTRATION, SLIDER_PREVIEW_REGISTRATION,
    SWITCH_PREVIEW_REGISTRATION, TEXTAREA_PREVIEW_REGISTRATION, TIMEPICKER_PREVIEW_REGISTRATION,
    TOGGLEBUTTON_PREVIEW_REGISTRATION, TRANSFERLIST_PREVIEW_REGISTRATION,
};
#[cfg(feature = "preview")]
pub use grid::GRID_PREVIEW_REGISTRATION;
#[cfg(feature = "preview")]
pub use navigation::anchor::ANCHOR_PREVIEW_REGISTRATION;
#[cfg(feature = "preview")]
pub use navigation::back_to_top::BACKTOTOP_PREVIEW_REGISTRATION;
#[cfg(feature = "preview")]
pub use navigation::{
    ACCORDION_PREVIEW_REGISTRATION, CAROUSELSTEPPER_PREVIEW_REGISTRATION,
    CAROUSEL_PREVIEW_REGISTRATION, DRAWER_PREVIEW_REGISTRATION,
    FLOATINGACTIONSMENU_PREVIEW_REGISTRATION, MENU_PREVIEW_REGISTRATION,
    POPOVER_PREVIEW_REGISTRATION, SPOTLIGHTPOPOVER_PREVIEW_REGISTRATION,
    SPOTLIGHTTIP_PREVIEW_REGISTRATION, SPOTLIGHTTOUR_PREVIEW_REGISTRATION,
    TOOLTIP_PREVIEW_REGISTRATION,
};
#[cfg(feature = "preview")]
pub use rating::{RATINGDISPLAY_PREVIEW_REGISTRATION, RATING_PREVIEW_REGISTRATION};
#[cfg(feature = "preview")]
pub use space::SPACE_PREVIEW_REGISTRATION;
#[cfg(feature = "preview")]
pub use stack::STACK_PREVIEW_REGISTRATION;
#[cfg(feature = "preview")]
pub use tag::{
    INTERACTIONTAG_PREVIEW_REGISTRATION, TAGGROUP_PREVIEW_REGISTRATION,
    TAGPICKER_PREVIEW_REGISTRATION, TAG_PREVIEW_REGISTRATION,
};

#[cfg(feature = "preview")]
pub(crate) mod components {
    pub use crate::preview::components::*;
}
