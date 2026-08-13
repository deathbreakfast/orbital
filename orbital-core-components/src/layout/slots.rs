use leptos::prelude::*;

/// Optional header region — typically a Fixed or Sticky [`AppBar`](crate::AppBar).
#[slot]
pub struct LayoutHeader {
    pub children: Children,
}

/// Side navigation or utility column.
#[slot]
pub struct LayoutSidebar {
    pub children: ChildrenFn,
}

/// Primary scrollable content region.
#[slot]
pub struct LayoutMain {
    pub children: Children,
}
