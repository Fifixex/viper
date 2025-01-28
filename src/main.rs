use viper::application::Application;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let app = Application::new()?;
	app.run();

	Ok(())
}
