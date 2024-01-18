use iced::{Element, widget::{Row, row, column, text, container}, Length};

use crate::{application::{GUI, ApplicationEvent}, logic::analyzer::{ViewColumn, self}};

pub fn directory_display_view(app: &GUI) -> Element<ApplicationEvent> {
	let file_list = ViewColumn::default_butt_title(String::from("root"));
	let file_columns = analyzer::create_analyzer_columns(file_list);
	let header_view: Row<'_, ApplicationEvent> = row![];
	let header_columns = file_columns
			.children
			.iter()
			.fold(header_view, |acc, (k, v)| acc.push(text(k)));
	let dir = &app.dir;
	let dir_clone = dir.clone();
	let directory_content = analyzer::fill_list_store(dir_clone);
	let directory_list = directory_content.iter().map(|dir_store| {
			let icon = "f";
			let percent =
					((dir_store.inner_size % dir_store.outer_size) * 100).to_string() + "%";

			let file_row: Row<'_, ApplicationEvent> = row![
					text(icon),
					text(dir_store.name.clone()),
					text(percent),
					text(dir_store.inner_size)
			];
			file_row
	});
	let directory_column =
			directory_list.fold(column![], |column, row| column.push(row));
	let directory_display = column![header_columns, directory_column];
	container(directory_display)
			.height(Length::Fill)
			.center_y()
			.into()
}