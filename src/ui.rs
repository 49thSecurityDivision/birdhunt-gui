pub mod theme;

mod pages;
mod widgets;

use {
	crate::{
		app::RenderInfo,
		state::{BirdHuntTab, State},
		ui::widgets::{ColorButton, Popup, TextInput},
	},
	uing::{
		Anchor, FlexDirection, UiContext, WidgetDim, WidgetKey, WidgetLayout, WidgetReaction,
		components::ButtonColors, wk,
	},
};

pub type RenderFn = fn(&mut UiContext, RenderInfo, &mut State) -> WidgetReaction;

#[inline(always)]
fn window_key() -> WidgetKey {
	wk!()
}

/// Render the Birdhunt app.
pub fn render(ui: &mut UiContext, render_info: RenderInfo, state: &mut State) -> WidgetReaction {
	let ui_state = &mut state.ui_state;

	let window = ui
		.build_widget(window_key())
		.size_fill()
		.layout(WidgetLayout::Stacked)
		.build();
	ui_state.window = Some(window.id());

	let root = ui
		.build_widget(wk!())
		.size_fill()
		.color(state.theme.content_background_color)
		.layout(WidgetLayout::Flex {
			direction: FlexDirection::Vertical,
			gap: 5.0,
			wrap: true,
		})
		.center()
		.build();
	ui.add_child(window, root);

	let tab_bar = widgets::TabBar {
		tabs: &[
			widgets::Tab {
				id: BirdHuntTab::Home,
				label: "Home",
			},
			widgets::Tab {
				id: BirdHuntTab::Hosts,
				label: "Hosts",
			},
			widgets::Tab {
				id: BirdHuntTab::Scripts,
				label: "Scripts",
			},
		],
		active_colors: ButtonColors {
			regular: (theme::BASE, 0),
			hovered: (theme::BASE, 0),
			pressed: (theme::BASE, 0),
		},
		inactive_colors: ButtonColors {
			regular: (theme::CRUST, 0),
			hovered: (theme::MANTLE, 0),
			pressed: (theme::BASE, 0),
		},
		active: &mut ui_state.active_tab,
		text_props: &state.theme.body_text,
	}
	.build(wk!(), ui, &state.theme);
	ui.add_child(root, tab_bar);

	let page_to_render: RenderFn = match ui_state.active_tab {
		BirdHuntTab::Home => pages::home::render,
		BirdHuntTab::Scripts => pages::scripts::render,
		BirdHuntTab::Hosts => pages::hosts::render,
	};
	let page = (page_to_render)(ui, render_info, state);
	ui.add_child(root, page);

	if state.default_password.is_empty() || state.default_username.is_empty() {
		let popup = Popup {
			title: "Initial Setup",
			dismissable: false,
			show: &mut true,
		}
		.build(wk!(), ui, state);

		let explanation = ui
			.text(
				wk!(),
				"Welcome to BirdHunt! Please enter default credentials to manage new servers with.",
				&state.theme.body_text,
			)
			.size_wh(WidgetDim::fill(), WidgetDim::hug())
			.top_center()
			.build();
		ui.add_child(popup, explanation);

		let form = ui
			.build_widget(wk!())
			.pad_hv(0.0, 15.0)
			.flex_col(15.0)
			.top_center()
			.size_wh(WidgetDim::Fixed(300.0), WidgetDim::hug())
			.build();
		ui.add_child(popup, form);

		let (default_username, default_username_key) = TextInput {
			placeholder: "Username",
			width: WidgetDim::fill(),
			password: false,
		}
		.build(wk!(), ui, state);
		ui.add_child(form, default_username);

		let (default_password, default_password_key) = TextInput {
			placeholder: "Password",
			width: WidgetDim::fill(),
			password: true,
		}
		.build(wk!(), ui, state);
		ui.add_child(form, default_password);

		let save = ColorButton {
			label: "Save",
			width: WidgetDim::Fixed(200.0),
			height: WidgetDim::hug(),
			padding: 5.0,
			anchor: Anchor::TOP_CENTER,
		}
		.build(wk!(), ui, state);
		ui.add_child(form, save);

		if save.l_clicked() {
			state.default_password = ui
				.get_text_input_content(default_password_key)
				.unwrap()
				.to_string();
			state.default_username = ui
				.get_text_input_content(default_username_key)
				.unwrap()
				.to_string();
		}
	}

	window
}
