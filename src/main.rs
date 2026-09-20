use {
	crate::{app::App, state::State},
	uing::{
		UiContext,
		windowing::winit::{WinitPlatformInteractions, WinitUingApp},
	},
	winit::{
		event_loop::{ControlFlow, EventLoop},
		window::{Theme, WindowAttributes},
	},
};

mod app;
mod state;
mod ui;

fn main() {
	unsafe { crux::init() };
	dioxus_devtools::connect_subsecond();

	let event_loop = EventLoop::new().unwrap();
	event_loop.set_control_flow(ControlFlow::Wait);

	let mut app = WinitUingApp::new(
		WindowAttributes::default()
			.with_active(true)
			.with_theme(Some(Theme::Dark))
			.with_title("Birdhunt")
			.with_resizable(true),
		|window, scale_factor, viewport_size| {
			Box::new(App {
				ui_ctx: UiContext::new(
					Box::new(WinitPlatformInteractions::new(window)),
					scale_factor,
					viewport_size,
				),
				state: State::new(),
			})
		},
	);

	event_loop.run_app(&mut app).unwrap();
}
