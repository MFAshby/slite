use druid::{AppLauncher,WindowDesc,Widget};
use druid::widget::Label;
use slite::cfg;
use slite::db;
use slite::rtm_listener;
use std::thread::spawn;
use std::error::Error;

fn build_ui() -> impl Widget<()> {
    Label::new("Hello World!")
}

fn main() -> Result<(), Box<dyn Error>> {
    let c = cfg::load();
    let d = db::load(&c)?;
    spawn(move || {
        let _ = rtm_listener::start(&c, &d);
    });
    AppLauncher::with_window(WindowDesc::new(build_ui)).launch(()).map_err(Box::new)?;
    Ok(())
}
