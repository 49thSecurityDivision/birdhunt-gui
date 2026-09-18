use {
	crate::{app::RenderInfo, state::State},
	uing::{TextProps, UiContext, WidgetReaction, wk},
};

pub fn render(ui: &mut UiContext, _render_info: RenderInfo, _state: &mut State) -> WidgetReaction {
	ui.text(wk!(), "Home", &TextProps::default()).build()
}
