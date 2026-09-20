use {
	crate::{
		app::RenderInfo,
		state::{
			State, Theme,
			host::{ConnectionInfo, Enabled, HostName},
		},
		ui::{theme, widgets::Table},
	},
	ecs::query::Select,
	uing::{UiContext, WidgetDim, WidgetReaction, glam::Vec4, wk},
};

pub fn render(ui: &mut UiContext, _render_info: RenderInfo, state: &mut State) -> WidgetReaction {
	let root = ui
		.build_widget(wk!())
		.size_fill()
		.scrollable()
		.flex_col(5.0)
		.pad_all(5.0)
		.build();

	let query = state
		.hosts
		.query::<Select<(&HostName, &ConnectionInfo)>>()
		.filter(|e| e.0.has::<Enabled>(&state.hosts));
	let table = Table {
		column_names: &["Hostname", "IP"],
		width: WidgetDim::fill(),
		height: WidgetDim::fill(),
	}
	.build(wk!(), ui, &state.theme);
	ui.add_child(root, table.table);

	for (entity, hostname, conn_info) in query {
		let hostname = ui
			.text(wk!(entity.as_raw()), &hostname.0, &state.theme.body_text)
			.build();
		let ip = ui
			.text(wk!(entity.as_raw()), &conn_info.ip, &state.theme.body_text)
			.build();

		table.add_row(ui, &[hostname, ip]);
	}

	root
}

fn add_host_form(ui: &mut UiContext, theme: &Theme) -> WidgetReaction {
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
		&theme.body_text,
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
		&theme.body_text,
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
		&theme.body_text,
		"Username",
		theme::SUBTEXT0,
	);
	ui.add_child(root, username_input_widget);

	root
}
