//! Graph components: [`Graph`] draws a [`GraphLayout`](crate::GraphLayout) as
//! SVG and [`GraphLegend`] toggles sets of its nodes.

mod fill;
#[allow(clippy::module_inception)]
mod graph;
mod keyboard;
mod legend;
mod model;
mod styles;
pub mod viewport;

pub use fill::{FillTokens, GraphFill};
pub use graph::*;
pub use legend::*;
pub use model::{GraphNodeView, MAX_NODE_LINES};
