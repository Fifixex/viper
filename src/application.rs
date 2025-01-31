use super::error::Error;
use adw::prelude::*;
use gtk::glib;

#[derive(Clone)]
pub struct Application {
	gtk_app: adw::Application,
}

impl Application {
	pub fn new() -> Result<Application, Error> {
		let gtk_app = adw::Application::builder()
			.application_id("sh.viper.terminal")
			.build();

		if let Err(err) = gtk_app.register(None as Option<&gtk::gio::Cancellable>) {
			return Err(Error::Error(err));
		}

		gtk_app.connect_activate(|_app| {
			tracing::info!("gtk: Activated application");
		});

		Ok(Application { gtk_app })
	}

	#[inline]
	pub fn gtk_app(&self) -> &adw::Application {
		&self.gtk_app
	}

	pub fn run(&self) -> glib::ExitCode {
		self.gtk_app.run()
	}
}
