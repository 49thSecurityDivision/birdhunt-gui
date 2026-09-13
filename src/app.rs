use uing::{
	UiContext, WidgetBuilder, WidgetReaction, frienderer::Renderer, glam::Vec2, windowing::UingApp,
};
use winit::event::KeyEvent;

#[allow(dead_code)]
pub struct RenderInfo<'a> {
	pub renderer: &'a mut Renderer,
	pub viewport: Vec2,
	pub scale_factor: f32,
}

pub struct App<R>
where
	for<'a> R: FnMut(&mut UiContext, RenderInfo<'a>) -> WidgetReaction,
{
	pub ui_ctx: UiContext,
	pub render_fn: R,
}
impl<R> UingApp<KeyEvent> for App<R>
where
	for<'a> R: FnMut(&mut UiContext, RenderInfo<'a>) -> WidgetReaction,
{
	fn ui_mut(&mut self) -> &mut UiContext {
		&mut self.ui_ctx
	}

	fn on_key_input(&mut self, _key_event: &KeyEvent) -> bool {
		false
	}

	fn on_redraw(&mut self, renderer: &mut Renderer, viewport: Vec2, scale_factor: f32) {
		let App { ui_ctx, render_fn } = self;

		ui_ctx.start_frame();
		ui_ctx.resize(viewport);

		ui_ctx.build_interface(renderer, |ui, renderer| {
			let render_info = RenderInfo {
				renderer,
				viewport,
				scale_factor,
			};
			(render_fn)(ui, render_info)
		});
		ui_ctx.override_styles(|_widget: WidgetBuilder| {});

		ui_ctx.solve_layout(scale_factor);
		ui_ctx.react();
		ui_ctx.draw_widgets(renderer, scale_factor);
		ui_ctx.make_renderer_draw(renderer);

		ui_ctx.end_frame();
	}
}
