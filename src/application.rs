use super::css::load_css;
use super::error::Error;
use gtk::gdk::Display;
use gtk::gio::prelude::ApplicationExtManual;
use gtk::gio::{
	ApplicationFlags,
	Cancellable,
};
use gtk::prelude::*;
use gtk::Application as GtkApplication;

#[derive(Clone)]
pub struct Application {
	gtk_app: GtkApplication,
}

impl Application {
	pub fn new() -> Result<Application, Error> {
		let gtk_app = GtkApplication::new(
			Some("sh.viper.terminal"),
			ApplicationFlags::NON_UNIQUE,
		);

		gtk_app.connect_activate(Self::on_activate);

		if let Err(err) = gtk_app.register(None as Option<&Cancellable>) {
			return Err(Error::Error(err));
		}

		Ok(Application { gtk_app })
	}

	fn on_activate(app: &GtkApplication) {
		tracing::info!("gtk: Activated application");

		if let Err(e) = Self::load_and_apply_css() {
			tracing::error!("Failed to load CSS: {}", e);
		}

		let window = Self::create_main_window(app);
		window.present();
	}

	fn load_and_apply_css() -> Result<(), Error> {
		let provider = load_css().unwrap();
		let display = Display::default().ok_or_else(|| {
			tracing::error!("Failed to connect to display");
			Error::DisplayConnectionFailed
		})?;

		gtk::style_context_add_provider_for_display(
			&display,
			&provider,
			gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
		);

		Ok(())
	}

	fn create_main_window(app: &GtkApplication) -> gtk::ApplicationWindow {
		gtk::ApplicationWindow::builder()
			.application(app)
			.default_width(800)
			.default_height(600)
			.title("Viper Terminal")
			.build()
	}

	#[inline]
	pub fn gtk_app(&self) -> &GtkApplication {
		&self.gtk_app
	}

	pub fn run(&self) -> gtk::glib::ExitCode {
		self.gtk_app.run()
	}
}
