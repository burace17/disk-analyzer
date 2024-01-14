use iced::widget::button;

impl button::StyleSheet for ButtonStyle {
	fn active(&self) -> button::Style {
			if self.is_enabled {
					// Active state for enabled button
					button::Style::default()
			} else {
					// Active state for disabled button
					button::Style {
							background: Some(iced::Color::from_rgb(0.8, 0.8, 0.8)),
							text_color: iced::Color::from_rgb(0.5, 0.5, 0.5),
							..button::Style::default()
					}
			}
	}
}
