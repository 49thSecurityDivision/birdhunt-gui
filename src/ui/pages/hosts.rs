use {
	crate::{app::RenderInfo, state::State, ui::theme},
	uing::{UiContext, WidgetDim, WidgetReaction, glam::Vec4, wk},
};

pub fn render(ui: &mut UiContext, _render_info: RenderInfo, state: &mut State) -> WidgetReaction {
	let root = ui.build_widget(wk!()).size_fill().flex_col(5.0).build();

	let hostname_input = wk!();
	let hostname_input_widget = ui.text_input(
		hostname_input,
		|container| {
			container
				.border_width(Vec4::ONE)
				.border_color(theme::OVERLAY0)
				.size_wh(WidgetDim::Fixed(100.0), WidgetDim::Fixed(20.0))
		},
		&state.theme.body_text,
		"Host Name",
		theme::SUBTEXT0,
	);
	ui.add_child(root, hostname_input_widget);

	let ip_input = wk!();
	let ip_input_widget = ui.text_input(
		ip_input,
		|container| {
			container
				.border_width(Vec4::ONE)
				.border_color(theme::OVERLAY0)
				.size_wh(WidgetDim::Fixed(100.0), WidgetDim::Fixed(20.0))
		},
		&state.theme.body_text,
		"IP",
		theme::SUBTEXT0,
	);
	ui.add_child(root, ip_input_widget);

	let username_input = wk!();
	let username_input_widget = ui.text_input(
		username_input,
		|container| {
			container
				.border_width(Vec4::ONE)
				.border_color(theme::OVERLAY0)
				.size_wh(WidgetDim::Fixed(100.0), WidgetDim::Fixed(20.0))
		},
		&state.theme.body_text,
		"Username",
		theme::SUBTEXT0,
	);
	ui.add_child(root, username_input_widget);

	root
}
