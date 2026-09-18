use {ecs::World, uing::TextProps};

pub struct HostsWorldMarker;

/// Birdhunt app state.
pub struct State {
	pub ui_state: UiState,
	pub hosts: World<HostsWorldMarker>,
}
impl State {
	pub fn new() -> Self {
		Self {
			ui_state: UiState {
				active_tab: BirdHuntTab::Home,
				default_text_props: TextProps::default_sans(14.0),
			},
			hosts: World::new(),
		}
	}
}

pub struct UiState {
	pub active_tab: BirdHuntTab,
	pub default_text_props: TextProps<'static>,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum BirdHuntTab {
	Home,
	Hosts,
	Scripts,
}
