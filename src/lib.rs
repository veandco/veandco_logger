use env_logger::fmt::Color;
use env_logger::Builder;
use log::Level;
use std::io::Write;

pub fn init() {
    let mut builder = Builder::from_default_env();

    builder
        .format(|buf, record| {
            let color = match record.level() {
                Level::Debug => Color::Blue,
                Level::Info => Color::Black,
                Level::Warn => Color::Yellow,
                Level::Error => Color::Red,
                Level::Trace => Color::Rgb(64, 64, 64),
            };

            let mut style = buf.style();
            style.set_color(color);

            writeln!(
                buf,
                "[{} {} {} ({}:{})] {}",
                buf.timestamp(),
                style.value(record.level()),
                record.module_path().unwrap(),
                record.file().unwrap(),
                record.line().unwrap(),
                record.args(),
            )
        })
        .init();
}
