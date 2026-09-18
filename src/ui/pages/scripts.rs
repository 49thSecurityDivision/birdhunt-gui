use {
	crate::{app::RenderInfo, state::State},
	uing::{UiContext, WidgetReaction, wk},
};

pub fn render(ui: &mut UiContext, _render_info: RenderInfo, _state: &mut State) -> WidgetReaction {
	ui.build_widget(wk!()).build()
}
