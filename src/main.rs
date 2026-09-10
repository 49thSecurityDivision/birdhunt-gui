mod app;
mod tab_bar;

use uing::{
	TextProps, UiContext, WidgetReaction,
	windowing::winit::{WinitPlatformInteractions, WinitUingApp},
	wk,
};
use winit::{
	event_loop::{ControlFlow, EventLoop},
	window::{Theme, WindowAttributes},
};

use crate::app::{App, RenderInfo};

struct AppState {}

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
				render_fn: mk_render_fn(),
			})
		},
	);

	event_loop.run_app(&mut app).unwrap();
}

fn mk_render_fn() -> impl for<'a> FnMut(&mut UiContext, RenderInfo<'a>) -> WidgetReaction {
	let mut state = AppState {};

	move |ui, _| ui.text(wk!(), "Hello, world!", &TextProps::new()).build()
}
