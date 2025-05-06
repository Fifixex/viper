use adw::ApplicationWindow;
use gtk::gdk::Display;
use gtk::glib::object::IsA;

use super::css;

pub struct WindowBuilder {
	app:   adw::Application,
	title: String,
}

impl WindowBuilder {
	pub fn new(app: adw::Application) -> WindowBuilder {
		WindowBuilder {
			app,
			title: String::new(),
		}
	}

	pub fn set_title(&mut self, title: impl Into<String>) {
		self.title = title.into();
	}

	pub fn show(&mut self, content: &impl IsA<gtk::Widget>) -> ApplicationWindow {
		let provider = css::load_css().unwrap();
		let display = Display::default().unwrap();
		gtk::style_context_add_provider_for_display(
			&display,
			&provider,
			gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
		);

		ApplicationWindow::builder()
			.application(&self.app)
			.default_width(800)
			.default_height(600)
			.title(&self.title)
			.content(content)
			.build()
	}
}
