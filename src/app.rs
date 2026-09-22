use {
	crate::state::State,
	uing::{
		TextInputAction, UiContext, WidgetBuilder, frienderer::Renderer, glam::Vec2,
		windowing::UingApp,
	},
	winit::{
		event::{ElementState, KeyEvent},
		keyboard::{Key, NamedKey},
	},
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

	fn on_key_input(&mut self, key_event: &KeyEvent) -> bool {
		let ui = &mut self.ui_ctx;

		'text_input: {
			let text = key_event
				.text
				.as_ref()
				.map(|s| s.as_str())
				.unwrap_or_default();

			let action = match (key_event.state, key_event.logical_key.as_ref()) {
				(ElementState::Pressed, Key::Named(NamedKey::Backspace)) => {
					match ui.modifier_ctrl() {
						true => TextInputAction::DeleteLeftWord,
						false => TextInputAction::DeleteLeftChar,
					}
				}
				(ElementState::Pressed, Key::Named(NamedKey::Delete)) => match ui.modifier_ctrl() {
					true => TextInputAction::DeleteRightWord,
					false => TextInputAction::DeleteRightChar,
				},
				(ElementState::Pressed, Key::Named(NamedKey::ArrowLeft)) => {
					match ui.modifier_ctrl() {
						true => TextInputAction::MoveLeftWord,
						false => TextInputAction::MoveLeftChar,
					}
				}
				(ElementState::Pressed, Key::Named(NamedKey::ArrowRight)) => {
					match ui.modifier_ctrl() {
						true => TextInputAction::MoveRightWord,
						false => TextInputAction::MoveRightChar,
					}
				}
				(ElementState::Pressed, Key::Named(NamedKey::ArrowUp)) => {
					match ui.modifier_ctrl() {
						true => TextInputAction::MoveUpParagraph,
						false => TextInputAction::MoveUpLine,
					}
				}
				(ElementState::Pressed, Key::Named(NamedKey::ArrowDown)) => {
					match ui.modifier_ctrl() {
						true => TextInputAction::MoveDownParagraph,
						false => TextInputAction::MoveDownLine,
					}
				}
				(ElementState::Pressed, Key::Named(NamedKey::Enter)) if ui.modifier_shift() => {
					TextInputAction::NewLine
				}
				(ElementState::Pressed, Key::Named(NamedKey::Enter)) => {
					TextInputAction::SubmitPress
				}
				(ElementState::Released, Key::Named(NamedKey::Enter)) => {
					TextInputAction::SubmitRelease
				}
				(ElementState::Pressed, Key::Named(NamedKey::Tab)) => break 'text_input,
				(ElementState::Pressed, key) => match ui.modifier_ctrl() {
					true => match key {
						Key::Character("a") => TextInputAction::SelectAll,
						Key::Character("c") => TextInputAction::Copy,
						Key::Character("x") => TextInputAction::Cut,
						Key::Character("v") => TextInputAction::Paste,
						_ => break 'text_input,
					},
					false if text.is_empty() => break 'text_input,
					false => TextInputAction::Commit,
				},

				_ => break 'text_input,
			};

			if ui.on_text_input_action(action, ui.modifier_shift(), text, None) {
				return true;
			}
		};

		match (key_event.state, key_event.logical_key.as_ref()) {
			(ElementState::Pressed, Key::Character("=")) if ui.modifier_ctrl() => {
				ui.set_scale_factor(2.0);
			}
			(ElementState::Pressed, Key::Character("-")) if ui.modifier_ctrl() => {
				ui.set_scale_factor(1.0);
			}
			(ElementState::Released, Key::Named(NamedKey::Tab)) => {
				if ui.modifier_shift() {
					ui.focus_on_prev();
				} else {
					ui.focus_on_next();
				}
			}
			(state, Key::Named(NamedKey::Enter | NamedKey::Space)) => {
				ui.set_focus_pressed(state == ElementState::Pressed);
			}
			(ElementState::Released, Key::Named(NamedKey::Escape)) => {
				ui.focus_on(None);
			}
			(ElementState::Released, Key::Character("c")) if ui.modifier_ctrl() => {
				ui.copy_global_selected_text_to_clipboard();
			}
			(ElementState::Released, Key::Named(NamedKey::F3 | NamedKey::F12)) => {
				ui.inspector.toggle();
			}
			_ => return false,
		}

		true
	}

	fn on_redraw(&mut self, renderer: &mut Renderer, viewport: Vec2, scale_factor: f32) {
		let App { ui_ctx, state } = self;

		state.check_tasks();

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
