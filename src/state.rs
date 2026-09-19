use {
	crate::ui::theme,
	ecs::World,
	uing::{TextProps, WidgetId},
};

pub struct HostsWorldMarker;

/// Birdhunt app state.
pub struct State {
	pub ui_state: UiState,
	pub theme: Theme,
	pub hosts: World<HostsWorldMarker>,
	pub default_username: String,
	pub default_password: String,
}
impl State {
	pub fn new() -> Self {
		Self {
			ui_state: UiState {
				window: None,
				active_tab: BirdHuntTab::Home,
			},
			theme: Theme {
				title_text: TextProps::default_sans(40.0),
				body_text: TextProps::default_sans(16.0).multiline(),
				monospace_text: TextProps::default_mono(16.0).line_height(1.0),
				text_color: theme::TEXT,
				shadow_color: theme::CRUST,
				navigation_background_color: theme::BASE,
				navigation_hover_background_color: theme::MANTLE,
				content_background_color: theme::CRUST,
				popup_background_color: theme::SURFACE0,
			},
			hosts: World::new(),
			default_username: String::new(),
			default_password: String::new(),
		}
	}
}

pub struct UiState {
	/// Widget ID of the "window", or absolute top-level, widget. Always `Some`,
	/// only optional since it has to be loaded after the state gets created.
	///
	/// Useful for items that need to render stacked on top of the app's
	/// content.
	pub window: Option<WidgetId>,
	pub active_tab: BirdHuntTab,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum BirdHuntTab {
	Home,
	Hosts,
	Scripts,
}

pub struct Theme {
	pub title_text: TextProps<'static>,
	pub body_text: TextProps<'static>,
	pub monospace_text: TextProps<'static>,
	pub text_color: u32,
	pub shadow_color: u32,
	pub navigation_background_color: u32,
	pub navigation_hover_background_color: u32,
	pub content_background_color: u32,
	pub popup_background_color: u32,
}
