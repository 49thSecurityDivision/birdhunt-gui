use uing::{
	FlexDirection, TextProps, UiContext, WidgetDim, WidgetKey, WidgetLayout, WidgetReaction,
	WidgetSize, components::ButtonColors, wk,
};

pub struct Tab<'a, T: Eq + Copy> {
	pub id: T,
	pub label: &'a str,
}

pub struct TabBar<'a, T: Eq + Copy> {
	pub tabs: &'a [Tab<'a, T>],
	pub active_colors: ButtonColors,
	pub inactive_colors: ButtonColors,
	pub active: &'a mut T,
}
impl<'a, T: Eq + Copy> TabBar<'a, T> {
	pub fn build(self, key: WidgetKey, ui: &mut UiContext) -> WidgetReaction {
		let root = ui
			.build_widget(wk!([key]))
			.size(WidgetSize {
				w: WidgetDim::fill(),
				h: WidgetDim::hug(),
			})
			.layout(WidgetLayout::Flex {
				direction: FlexDirection::Horizontal,
				gap: 5.0,
				wrap: false,
			})
			.build();

		for (idx, tab) in self.tabs.iter().enumerate() {
			let colors = match tab.id == *self.active {
				true => self.active_colors,
				false => self.inactive_colors,
			};

			let tab_btn = ui
				.btn_box(wk!([key], idx), colors)
				.size(WidgetSize::hug_round())
				.build();

			let tab_lbl = ui
				.text(wk!([key], idx), tab.label, &TextProps::default())
				.build();
			ui.add_child(tab_btn, tab_lbl);

			if tab_btn.l_clicked() {
				*self.active = tab.id;
			}

			ui.add_child(root, tab_btn);
		}

		root
	}
}
