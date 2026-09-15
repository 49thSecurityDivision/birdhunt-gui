use uing::{
	FlexDirection, TextProps, UiContext, WidgetDim, WidgetKey, WidgetLayout, WidgetReaction,
	WidgetSize, components::ButtonColors, glam::Vec4, wk,
};

pub struct Tab<'a, T: Eq + Copy> {
	pub id: T,
	pub label: &'a str,
}

pub struct TabBar<'a, T: Eq + Copy> {
	/// The list of tabs in this tab bar.
	pub tabs: &'a [Tab<'a, T>],
	/// The button colors for the active tab.
	pub active_colors: ButtonColors,
	/// The button colors for the inactive tabs.
	pub inactive_colors: ButtonColors,
	/// The ID to the active tab. This will be updated if the tab is switched.
	pub active: &'a mut T,
	/// Text props for the labels on the tabs.
	pub text_props: &'a TextProps<'a>,
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
				gap: 1.0,
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
				.pad_all(5.0)
				.border(
					0,
					Vec4::from_array([0.0; 4]),
					Vec4::new(12.0, 0.0, 0.0, 12.0),
				)
				.build();

			let tab_lbl = ui.text(wk!([key], idx), tab.label, self.text_props).build();
			ui.add_child(tab_btn, tab_lbl);

			if tab_btn.l_clicked() {
				*self.active = tab.id;
			}

			ui.add_child(root, tab_btn);
		}

		root
	}
}
