use {
	crate::state::{State, Theme},
	std::cell::RefCell,
	uing::{
		Anchor, FlexDirection, TextProps, UiContext, WidgetBuilder, WidgetDim, WidgetKey,
		WidgetKeyHash, WidgetLayout, WidgetPadding, WidgetReaction, WidgetSize,
		components::ButtonColors,
		glam::{Vec2, Vec4},
		wk,
	},
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
	pub fn build(self, key: WidgetKey, ui: &mut UiContext, theme: &Theme) -> WidgetReaction {
		let root = ui
			.build_widget(wk!([key]))
			.color(theme.navigation_background_color)
			.size(WidgetSize {
				w: WidgetDim::fill(),
				h: WidgetDim::hug(),
			})
			.layout(WidgetLayout::Flex {
				direction: FlexDirection::Horizontal,
				gap: 1.0,
				wrap: false,
			})
			.tag("TabBar")
			.build();

		for (idx, tab) in self.tabs.iter().enumerate() {
			let colors = match tab.id == *self.active {
				true => ButtonColors {
					regular: (theme.content_background_color, 0),
					hovered: (theme.content_background_color, 0),
					pressed: (theme.content_background_color, 0),
				},
				false => ButtonColors {
					regular: (theme.navigation_background_color, 0),
					hovered: (theme.navigation_hover_background_color, 0),
					pressed: (theme.navigation_hover_background_color, 0),
				},
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

pub struct TextInput<'a> {
	pub placeholder: &'a str,
	pub width: WidgetDim,
	pub password: bool,
}
impl<'a> TextInput<'a> {
	pub fn build<'ui>(
		self,
		key: WidgetKey,
		ui: &'ui mut UiContext,
		state: &State,
	) -> (WidgetReaction, WidgetKeyHash) {
		(
			ui.text_input(
				key,
				|container| {
					let container = container
						.border_width(Vec4::ONE)
						.border_color(state.theme.border_color)
						.border_radius(Vec4::splat(4.0))
						.pad_hv(3.0, 0.0)
						.size_wh(
							self.width,
							WidgetDim::Fixed(state.theme.body_single_line_text.font_size + 5.0),
						);

					if self.password {
						container.passwd_input()
					} else {
						container
					}
				},
				&state.theme.body_single_line_text,
				self.placeholder,
				state.theme.subtext_color,
			),
			key.hash(),
		)
	}
}

pub struct ColorButton<'a> {
	pub label: &'a str,
	pub width: WidgetDim,
	pub height: WidgetDim,
	pub padding: f32,
	pub anchor: Anchor,
}
impl Default for ColorButton<'_> {
	fn default() -> Self {
		Self {
			label: "",
			width: WidgetDim::hug(),
			height: WidgetDim::hug(),
			padding: 5.0,
			anchor: Anchor::CENTER,
		}
	}
}
impl<'a> ColorButton<'a> {
	pub fn build(self, key: WidgetKey, ui: &mut UiContext, state: &State) -> WidgetReaction {
		let btn = ui
			.btn_box(
				key,
				ButtonColors {
					regular: (state.theme.accent - 0x30, 0),
					hovered: (state.theme.accent - 0x10, 0),
					pressed: (state.theme.accent, 0),
				},
			)
			.pad_all(self.padding)
			.anchorigin(self.anchor)
			.size_wh(self.width, self.height)
			.border_radius(Vec4::splat(6.0))
			.tag("ColorButton")
			.build();
		let lbl = ui
			.text(wk!([key]), self.label, &state.theme.body_text)
			.center()
			.build();
		ui.add_child(btn, lbl);

		btn
	}
}

pub struct Table<'a> {
	pub column_names: &'a [&'a str],
	pub width: WidgetDim,
	pub height: WidgetDim,
}
impl<'a> Table<'a> {
	pub fn build(self, key: WidgetKey, ui: &mut UiContext, theme: &Theme) -> BuiltTable {
		let root = ui
			.build_widget(key)
			.flex_row(10.0)
			.pad_all(10.0)
			.size_wh(self.width, self.height)
			.border_width(Vec4::splat(1.0))
			.border_radius(Vec4::splat(8.0))
			.border_color(theme.border_color)
			.tag("Table")
			.build();

		let mut columns = Vec::new();

		for (idx, column_name) in self.column_names.iter().enumerate() {
			let column = ui
				.build_widget(wk!([key], idx))
				.flex_col(5.0)
				.size_wh(WidgetDim::fill(), WidgetDim::hug())
				.tag("Column")
				.build();
			ui.add_child(root, column);
			columns.push(column);

			let lbl = ui
				.text(wk!([key], idx), column_name, &theme.bold_text)
				.size_wh(WidgetDim::fill(), WidgetDim::hug())
				.padding(WidgetPadding::trbl(0.0, 0.0, 10.0, 0.0))
				.tag("Column Title")
				.build();
			ui.add_child(column, lbl);
		}

		BuiltTable {
			table: root,
			columns,
		}
	}
}

pub struct BuiltTable {
	pub table: WidgetReaction,
	columns: Vec<WidgetReaction>,
}
impl BuiltTable {
	pub fn add_row(&self, ui: &mut UiContext, values: &[WidgetReaction]) {
		for (column, entry) in self.columns.iter().zip(values) {
			ui.add_child(*column, *entry);
		}
	}
}

pub struct Popup<'a> {
	pub title: &'a str,
	pub dismissable: bool,
	pub show: &'a mut bool,
}
impl<'a> Popup<'a> {
	pub fn build(self, key: WidgetKey, ui: &mut UiContext, state: &State) -> WidgetReaction {
		let backdrop = ui
			.build_widget(wk!([key]))
			.size_fill()
			.hoverable()
			.clickable()
			.color(state.theme.shadow_color - 0x7f)
			.tag("Popup Backdrop")
			.build();
		ui.add_child(state.ui_state.window.unwrap(), backdrop);

		let root = ui
			.build_widget(wk!([key]))
			.center()
			.size_fixed(400.0, 400.0)
			.border_radius(Vec4::splat(6.0))
			.color(state.theme.popup_background_color)
			.box_shadow(state.theme.shadow_color, 5.0, Vec2::ZERO)
			.flex_col_wrap(5.0)
			.tag("Popup")
			.build();

		if self.dismissable {
			let close = ui
				.btn_box(
					wk!([key]),
					ButtonColors {
						regular: (state.theme.popup_background_color, 0),
						hovered: (state.theme.popup_background_color, 0),
						pressed: (state.theme.popup_background_color, 0),
					},
				)
				.top_right()
				.size_hug()
				.padding(WidgetPadding::hv(4.0, 2.0))
				.build();

			let x = ui
				.text(wk!([key]), "x", &state.theme.monospace_text)
				.center()
				.build();
			ui.add_child(close, x);
			ui.add_child(root, close);
		}

		let title = ui
			.text(wk!([key]), self.title, &state.theme.title_text)
			.top_center()
			.build();
		ui.add_child(root, title);

		ui.add_child(state.ui_state.window.unwrap(), root);
		root
	}
}
