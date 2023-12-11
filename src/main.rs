use env_logger::Env;

mod lib;

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    lib::core::xml::parse();
    // let mut application = lib::core::application::Application::new("demo/title", 1280, 720);
    //
    // // Run the app
    // while application.main_loop() {
    //
    // }
}