use gtk::CssProvider;

#[cfg(debug_assertions)]
pub fn load_css() -> Result<CssProvider, gtk::glib::Error> {
	#[cfg(debug_assertions)]
	let css_path = "./css/style.css";

	let css = CssProvider::default();
	css.load_from_path(css_path);

	Ok(css)
}
