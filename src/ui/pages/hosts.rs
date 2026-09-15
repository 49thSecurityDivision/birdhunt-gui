use {
	crate::app::RenderInfo,
	uing::{UiContext, WidgetReaction, wk},
};

pub fn render(ui: &mut UiContext, _render_info: RenderInfo) -> WidgetReaction {
	ui.build_widget(wk!()).build()
}
