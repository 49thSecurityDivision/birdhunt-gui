use {
	crate::state::State,
	uing::{UiContext, WidgetBuilder, frienderer::Renderer, glam::Vec2, windowing::UingApp},
	winit::event::KeyEvent,
};

#[allow(dead_code)]
pub struct RenderInfo<'a> {
	pub renderer: &'a mut Renderer,
	pub viewport: Vec2,
	pub scale_factor: f32,
}

pub struct App {
	pub ui_ctx: UiContext,
	pub state: State,
}
impl UingApp<KeyEvent> for App {
	fn ui_mut(&mut self) -> &mut UiContext {
		&mut self.ui_ctx
	}

	fn on_key_input(&mut self, _key_event: &KeyEvent) -> bool {
		false
	}

	fn on_redraw(&mut self, renderer: &mut Renderer, viewport: Vec2, scale_factor: f32) {
		let App { ui_ctx, state } = self;

		ui_ctx.start_frame();
		ui_ctx.resize(viewport);

		ui_ctx.build_interface(renderer, |ui, renderer| {
			let render_info = RenderInfo {
				renderer,
				viewport,
				scale_factor,
			};
			crate::ui::render(ui, render_info, state)
		});
		ui_ctx.override_styles(|_widget: WidgetBuilder| {});

		ui_ctx.solve_layout(scale_factor);
		ui_ctx.react();
		ui_ctx.draw_widgets(renderer, scale_factor);
		ui_ctx.make_renderer_draw(renderer);

		ui_ctx.end_frame();
	}
}
