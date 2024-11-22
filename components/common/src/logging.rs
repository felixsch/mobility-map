use env_logger::{Builder, Env};
use std::io::Write;

pub fn init() -> () {
    let env = Env::default().filter_or("LOG_LEVEL", "info");

    Builder::from_env(env)
        .format(|buf, record| writeln!(buf, "{}: {}", record.level(), record.args()))
        .init()
}
