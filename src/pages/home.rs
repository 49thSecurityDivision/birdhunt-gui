use uing::{TextProps, UiContext, WidgetReaction, wk};

use crate::app::RenderInfo;

pub fn render(ui: &mut UiContext, _render_info: RenderInfo) -> WidgetReaction {
	ui.text(wk!(), "Home", &TextProps::default()).build()
}
