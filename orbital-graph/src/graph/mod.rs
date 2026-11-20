//! Graph components: [`Graph`] draws a [`GraphLayout`](crate::GraphLayout) as
//! SVG, [`GraphToolbar`] and [`GraphMinimap`] control its view and
//! [`GraphLegend`] toggles sets of its nodes.

mod fill;
#[allow(clippy::module_inception)]
mod graph;
mod keyboard;
mod legend;
mod minimap;
mod model;
mod styles;
mod toolbar;
pub mod viewport;

pub use fill::{FillTokens, GraphFill};
pub use graph::*;
pub use legend::*;
pub use minimap::*;
pub use model::{GraphNodeView, MAX_NODE_LINES};
pub use toolbar::*;
