use gtk::glib::Error as GLibError;

#[derive(Debug, Clone)]
pub enum Error {
	Error(GLibError),
	DisplayConnectionFailed,
}

impl std::fmt::Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Error::Error(err) => write!(f, "GTK Error: {err}"),
			Error::DisplayConnectionFailed => {
				write!(f, "Failed to connect to display")
			}
		}
	}
}

impl std::error::Error for Error {}
