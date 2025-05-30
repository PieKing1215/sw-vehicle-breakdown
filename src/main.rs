pub mod data;
mod error_views;
mod templates;

use perseus::prelude::*;

#[perseus::main_export]
pub fn main<G: Html>() -> PerseusApp<G> {
    PerseusApp::new()
        .template(crate::templates::index::get_template())
        // .template(crate::templates::about::get_template())
        .error_views(crate::error_views::get_error_views())
}
