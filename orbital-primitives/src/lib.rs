//! Orbital primitives — native core components plus gap placeholders.
//!
//! Re-export this crate as `orbital::primitives` in application code.

// Shadow migrated native components (Flex, Button, Material, Card, Icon, Navigation).
#[cfg(feature = "preview")]
pub use orbital_core_components::ThemePreviewMarker;
pub use orbital_core_components::{
    Accordion, AccordionHeader, AccordionItem, Anchor, AnchorConfig, AnchorLink, AppBar,
    AppBarDensity, AppBarInset, AppBarLeading, AppBarMaterial, AppBarPosition, AppBarTrailing,
    AutoComplete, AutoCompleteAppearance, AutoCompleteBind, AutoCompleteEvents, AutoCompleteOption,
    AutoCompleteSize, Avatar, AvatarColor, AvatarConfig, AvatarGroup, AvatarGroupLayout,
    AvatarGroupSize, AvatarShape, BackToTop, BackToTopLabel, Backdrop, BackdropConfig, Badge,
    BadgeAppearance, BadgeColor, BadgeSize, Body1, Box, Breadcrumb, BreadcrumbButton,
    BreadcrumbDivider, BreadcrumbItem, Button, ButtonAppearance, ButtonGroup, ButtonRef,
    ButtonShape, ButtonSize, ButtonType, Calendar, CalendarAppearance, CalendarBind, Caption2,
    Card, CardButtonArea, CardContent, CardFooter, CardHeader, CardHeaderAction,
    CardHeaderDescription, CardMedia, CardPreview, Checkbox, CheckboxSize, Code, Color, ColorBind,
    ColorPicker, ColorPickerAppearance, ColorPickerBind, Combobox, ComboboxAppearance,
    ComboboxBind, ComboboxOption, ComboboxOptionGroup, ComboboxSize, CounterBadge, DatePicker,
    DatePickerAppearance, DatePickerBind, DatePickerRule, DatePickerRuleTrigger, DatetimeFormat,
    DatetimeTimezone, Dialog, DialogActions, DialogBody, DialogContent, DialogDismiss,
    DialogDismissConfig, DialogSurface, DialogTitle, Divider, Drawer, DrawerBody, DrawerHeader,
    DrawerHeaderTitle, DrawerHeaderTitleAction, DrawerModalType, DrawerPosition, DrawerSize,
    Dropdown, Field, FieldInjection, FieldOrientation, FieldValidationState, FileList, Flex,
    FlexAlign, FlexGap, FlexJustify, FlexWrap, FloatingActionsMenu, FloatingActionsMenuConfig,
    FloatingActionsMenuItem, FloatingButton, FloatingButtonColor, FloatingButtonConfig,
    FloatingButtonSize, FloatingButtonVariant, FormBind, Grid, GridConfig, GridItem,
    GridItemConfig, Icon, Image, ImageConfig, ImageFit, ImageShape, InfoLabel, InfoLabelInfo,
    InfoLabelSize, InfoLabelWeight, InlineDrawer, Input, InputAppearance, InputBind, InputEvents,
    InputPrefix, InputRef, InputRule, InputRuleTrigger, InputSize, InputSuffix, InputType, Label,
    LabelSize, LabelWeight, Layout, LayoutHeader, LayoutHeaderInset, LayoutMain, LayoutPosition,
    LayoutSidebar, LayoutSidebarOpen, LayoutSidebarToggle, Link, Material, MaterialCorners,
    MaterialElevation, MaterialVariant, Menu, MenuAppearance, MenuButton, MenuConfig, MenuItem,
    MenuPosition, MenuTrigger, MenuTriggerType, MessageBar, MessageBarActions, MessageBarBody,
    MessageBarIntent, MessageBarLayout, MessageBarTitle, Navigation, NavigationAppItem,
    NavigationBody, NavigationCategory, NavigationCategoryHeader, NavigationCollapseToggle,
    NavigationConfig, NavigationDensity, NavigationDivider, NavigationFooter, NavigationHeader,
    NavigationItem, NavigationItemConfig, NavigationMaterial, NavigationMode,
    NavigationSectionHeader, NavigationSubItem, NavigationSubItemGroup, NumericStepper,
    NumericStepperAppearance, NumericStepperBind, NumericStepperRule, NumericStepperRuleTrigger,
    NumericStepperSize, OffsetTarget, OpenBind, OptionBind, OverlayDrawer, Pagination,
    PaginationConfig, Popover, PopoverAppearance, PopoverConfig, PopoverLifecycle, PopoverPosition,
    PopoverSize, PopoverTrigger, PopoverTriggerType, PresenceBadge, PresenceBadgeSize,
    PresenceStatus, ProgressBar, ProgressBarColor, ProgressCircle, ProgressCircleColor, Radio,
    RadioGroup, RadioGroupBind, RadioGroupRule, RadioGroupRuleTrigger, SearchBox,
    SearchBoxAppearance, SearchBoxBind, SearchBoxEvents, Select, SelectAppearance, SelectBind,
    SelectRule, SelectRuleTrigger, SelectSize, Skeleton, SkeletonItem, Slider, SliderAppearance,
    SliderBind, SliderLabel, SliderRule, SliderRuleTrigger, Space, SpaceConfig, SpaceGap,
    SpacingHorizontal, SpacingInset, SpacingVertical, Spinner, SpinnerSize, Stack, StackConfig,
    Switch, SwitchBind, SwitchLabel, SwitchRule, SwitchRuleTrigger, Tab, TabList, Table, TableBody,
    TableCell, TableCellLayout, TableCellLayoutConfig, TableHeader, TableHeaderCell,
    TableHeaderCellConfig, TableRow, Text, TextAlign, TextFont, TextSize, TextTag, TextWeight,
    Textarea, TextareaAppearance, TextareaBind, TextareaEvents, TextareaRef, TextareaResize,
    TextareaRule, TextareaRuleTrigger, TextareaSize, TimePicker, TimePickerAppearance,
    TimePickerBind, Title1, Title3, Toast, ToastBody, ToastFooter, ToastIntent, ToastPosition,
    ToastTitle, ToasterInjection, ToasterProvider, ToggleButton, Tooltip, TooltipAppearance,
    TooltipConfig, TooltipPosition, TransferList, TransferListChange, TransferListConfig, Tree,
    TreeConfig, TreeItem, TreeItemConfig, TreeItemLayout, TreeItemType, TreeSize, Upload,
    UploadConfig, UploadDragger,
};
pub use orbital_theme::{
    BrandPalette, ColorTheme, CommonTheme, Density, Direction, ElevationScale,
    OrbitalThemeProvider, Theme, ThemeMode, ThemeOptions, ThemeOverrides, TypographyOverrides,
};

pub mod gap;
pub mod placeholder;

pub use gap::*;
pub use orbital_charts::*;
pub use orbital_datatable::*;
pub use orbital_date_pickers::*;
pub use orbital_discussion::*;
pub use orbital_history::*;
pub use orbital_scheduler::*;
pub use orbital_tree::*;
pub use placeholder::placeholder_label;

#[cfg(feature = "preview")]
pub mod preview;

#[cfg(feature = "preview")]
pub(crate) mod components {}
