use {
	crate::{
		app::RenderInfo,
		state::{
			State, Theme,
			host::{ConnectionInfo, Enabled, HostName},
		},
		ui::{
			theme,
			widgets::{ColorButton, Table, TextInput},
		},
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

	let hosts_lbl = ui.text(wk!(), "Hosts", &state.theme.title_text).build();
	ui.add_child(root, hosts_lbl);

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
			.text(
				wk!(entity.as_raw()),
				&conn_info.ip.to_string(),
				&state.theme.body_text,
			)
			.build();

		table.add_row(ui, &[hostname, ip]);
	}

	let add_host_lbl = ui.text(wk!(), "Add Host", &state.theme.title_text).build();
	ui.add_child(root, add_host_lbl);

	let (ip_input, ip_input_key) = TextInput {
		placeholder: "IP Address",
		width: WidgetDim::Fixed(300.0),
		password: false,
	}
	.build(wk!(), ui, state);
	ui.add_child(root, ip_input);

	let (port_input, port_input_key) = TextInput {
		placeholder: "SSH Port",
		width: WidgetDim::Fixed(300.0),
		password: false,
	}
	.build(wk!(), ui, state);
	ui.add_child(root, port_input);

	let add_host = ColorButton {
		label: "Add",
		..Default::default()
	}
	.build(wk!(), ui, state);
	ui.add_child(root, add_host);

	if add_host.l_clicked() {
		let ip = ui.get_text_input_content(ip_input_key).unwrap();
		let port = ui.get_text_input_content(port_input_key).unwrap();
		if let Ok(ip) = ip.parse()
			&& let Ok(port) = port.parse()
		{
			state.task_add_host(ConnectionInfo { ip, port });
			ui.get_text_input_mut(ip_input_key)
				.unwrap()
				.editor
				.set_text("");
			ui.get_text_input_mut(port_input_key)
				.unwrap()
				.editor
				.set_text("");
		}
	}

	root
}
