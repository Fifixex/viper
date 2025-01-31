use std::cell::RefCell;
use std::rc::Rc;

use viper::application::Application;
use viper::window::WindowBuilder;

use adw::prelude::*;
use adw::{
	ActionRow,
	HeaderBar,
};
use gtk::{
	Align,
	Box as BoxContainer,
	Orientation,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let app = Application::new()?;
	let window = Rc::new(RefCell::new(WindowBuilder::new(app.gtk_app().clone())));

	app.gtk_app().connect_activate(move |_| {
		build_ui(Rc::clone(&window));
	});
	app.run();
	Ok(())
}

fn build_ui(window: Rc<RefCell<WindowBuilder>>) {
	let mut window = window.borrow_mut();

	window.set_title("Viper");

	let content = BoxContainer::builder()
		.orientation(Orientation::Vertical)
		.halign(Align::BaselineFill)
		.hexpand(true)
		.hexpand_set(true)
		.homogeneous(false)
		.css_name("main-box")
		.build();

	let row = ActionRow::builder()
		.activatable(true)
		.title("Click me")
		.build();

	content.append(&HeaderBar::new());
	content.append(&row);

	window.show(&content).present();
}
