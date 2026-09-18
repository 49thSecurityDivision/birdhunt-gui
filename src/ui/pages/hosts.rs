use {
	crate::{app::RenderInfo, state::State, ui::theme},
	uing::{UiContext, WidgetDim, WidgetReaction, glam::Vec4, wk},
};

pub fn render(ui: &mut UiContext, _render_info: RenderInfo, state: &mut State) -> WidgetReaction {
	let ui_state = &mut state.ui_state;
	let root = ui.build_widget(wk!()).size_fill().build();

	let hostname_input = ui.text_input(
		wk!(),
		|container| {
			container
				.border_width(Vec4::ONE)
				.border_color(theme::OVERLAY0)
				.size_wh(WidgetDim::Fixed(100.0), WidgetDim::Fixed(20.0))
		},
		&ui_state.default_text_props,
		"Host Name",
		theme::SUBTEXT0,
	);
	ui.add_child(root, hostname_input);

	root
}
