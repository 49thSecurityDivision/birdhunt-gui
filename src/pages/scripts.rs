use uing::{UiContext, WidgetReaction, wk};

use crate::app::RenderInfo;

pub fn render(ui: &mut UiContext, _render_info: RenderInfo) -> WidgetReaction {
	ui.build_widget(wk!()).build()
}
