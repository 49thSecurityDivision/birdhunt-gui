use {
	crate::{app::App, state::State},
	tracing::Level,
	tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt},
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
	let filter = tracing_subscriber::filter::Targets::new()
		.with_target("uing", Level::DEBUG)
		.with_target("winit", Level::WARN)
		.with_default(Level::INFO);

	tracing_subscriber::registry()
		.with(tracing_subscriber::fmt::layer())
		.with(filter)
		.init();
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
