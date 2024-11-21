use env_logger::Builder;
use std::io::Write;

pub fn init() -> () {
    Builder::from_default_env()
        .format(|buf, record| writeln!(buf, "{}: {}", record.level(), record.args()))
        .init()
}
