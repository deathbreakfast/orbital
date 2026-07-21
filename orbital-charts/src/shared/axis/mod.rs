//! Axis and grid composition components.

mod grid;
mod ticks;
mod x_axis;
mod y_axis;

pub use grid::ChartGrid;
pub use ticks::{
    AXIS_TITLE_GAP, AXIS_TITLE_TICK_GAP, AXIS_TITLE_WIDTH, TICK_LABEL_HEIGHT, TICK_LABEL_OFFSET,
    TICK_SIZE,
};
pub use x_axis::XAxis;
pub use y_axis::YAxis;

#[cfg(feature = "preview")]
mod showcase;

#[cfg(feature = "preview")]
pub use showcase::{ChartsAxis, CHARTSAXIS_PREVIEW_REGISTRATION};
