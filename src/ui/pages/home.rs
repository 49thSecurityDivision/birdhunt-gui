use {
	crate::app::RenderInfo,
	uing::{TextProps, UiContext, WidgetReaction, wk},
};

pub fn render(ui: &mut UiContext, _render_info: RenderInfo) -> WidgetReaction {
	ui.text(wk!(), "Home", &TextProps::default()).build()
}
