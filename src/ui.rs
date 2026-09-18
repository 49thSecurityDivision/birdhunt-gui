mod pages;
mod theme;
mod widgets;

use {
	crate::{
		app::RenderInfo,
		state::{BirdHuntTab, State},
	},
	uing::{
		FlexDirection, TextProps, UiContext, WidgetLayout, WidgetReaction,
		components::ButtonColors, wk,
	},
};

pub type RenderFn = fn(&mut UiContext, RenderInfo, &mut State) -> WidgetReaction;

/// Render the Birdhunt app.
pub fn render(ui: &mut UiContext, render_info: RenderInfo, state: &mut State) -> WidgetReaction {
	let ui_state = &mut state.ui_state;

	let root = ui
		.build_widget(wk!())
		.size_fill()
		.color(theme::BASE)
		.layout(WidgetLayout::Flex {
			direction: FlexDirection::Vertical,
			gap: 5.0,
			wrap: true,
		})
		.center()
		.build();

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
		text_props: &ui_state.default_text_props,
	}
	.build(wk!(), ui);
	ui.add_child(root, tab_bar);

	let page_to_render: RenderFn = match ui_state.active_tab {
		BirdHuntTab::Home => pages::home::render,
		BirdHuntTab::Scripts => pages::scripts::render,
		BirdHuntTab::Hosts => pages::hosts::render,
	};
	let page = (page_to_render)(ui, render_info, state);
	ui.add_child(root, page);

	root
}
