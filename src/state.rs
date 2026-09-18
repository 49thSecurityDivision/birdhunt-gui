use ecs::World;

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
			},
			hosts: World::new(),
		}
	}
}

pub struct UiState {
	pub active_tab: BirdHuntTab,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum BirdHuntTab {
	Home,
	Hosts,
	Scripts,
}
