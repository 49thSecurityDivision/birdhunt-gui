mod app;
mod pages;
mod theme;
mod widgets;

use uing::{
	FlexDirection, UiContext, WidgetLayout, WidgetReaction,
	components::ButtonColors,
	windowing::winit::{WinitPlatformInteractions, WinitUingApp},
	wk,
};
use winit::{
	event_loop::{ControlFlow, EventLoop},
	window::{Theme, WindowAttributes},
};

use crate::app::{App, RenderInfo};

fn main() {
	let event_loop = EventLoop::new().unwrap();
	event_loop.set_control_flow(ControlFlow::Wait);

	let mut app = WinitUingApp::new(
		WindowAttributes::default()
			.with_active(true)
			.with_theme(Some(Theme::Dark))
			.with_title("uing playground")
			.with_resizable(true),
		|window, scale_factor, viewport_size| {
			Box::new(App {
				ui_ctx: UiContext::new(
					Box::new(WinitPlatformInteractions::new(window)),
					scale_factor,
					viewport_size,
				),
				render_fn: render(),
			})
		},
	);

	event_loop.run_app(&mut app).unwrap();
}

struct BirdHuntState {
	tab: BirdHuntTab,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum BirdHuntTab {
	Home,
	Hosts,
	Scripts,
}

fn render() -> impl for<'a> FnMut(&mut UiContext, RenderInfo<'a>) -> WidgetReaction {
	let mut state = BirdHuntState {
		tab: BirdHuntTab::Home,
	};

	move |ui, render_info| {
		let root = ui
			.build_widget(wk!())
			.size_fill()
			.color(theme::BASE)
			.layout(WidgetLayout::Flex {
				direction: FlexDirection::Vertical,
				gap: 5.0,
				wrap: true,
			})
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
				regular: (theme::MANTLE, 0),
				hovered: (theme::MANTLE, 0),
				pressed: (theme::MANTLE, 0),
			},
			inactive_colors: ButtonColors {
				regular: (theme::BASE, 0),
				hovered: (theme::CRUST, 0),
				pressed: (theme::MANTLE, 0),
			},
			active: &mut state.tab,
		}
		.build(wk!(), ui);
		ui.add_child(root, tab_bar);

		let page_to_render = match state.tab {
			BirdHuntTab::Home => pages::home::render,
			BirdHuntTab::Scripts => pages::scripts::render,
			BirdHuntTab::Hosts => pages::hosts::render,
		};
		let page = (page_to_render)(ui, render_info);
		ui.add_child(root, page);

		root
	}
}
