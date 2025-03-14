pub mod batches;
pub mod database;
pub mod distances;
pub mod timer;

pub use anyhow::anyhow;
pub use anyhow::Context;
pub use anyhow::Error;
pub use anyhow::Result;

pub use std::error::Error as StdError;
pub use std::result::Result as StdResult;

pub type BoxDynError = Box<dyn StdError + 'static + Send + Sync>;

pub use timer::Timer;

pub use distances::DISTANCES;
