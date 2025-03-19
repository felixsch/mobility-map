use serde;
use sqlx;
use std;
use tracing;

// error handling
pub use std::error::Error;
pub type BoxDynError = Box<dyn Error + 'static + Send + Sync>;

#[macro_export]
macro_rules! boxed_error {
    ($fmt:expr, $($args:tt)*) => {
        Box::from(format!($fmt, $($args)*)) as Box<dyn std::error::Error + Send + Sync>
    };
}

// flow handling
pub type NoResult = Result<(), BoxDynError>;
pub use futures::future::BoxFuture;
pub use futures::future::FutureExt;

pub use std::sync::Arc;

// database
pub type Pool = sqlx::PgPool;

// logging
pub use tracing::{debug, error, info};

// io
pub use std::fs::File;
pub use std::io::{Read, Seek};
pub use std::path::Path;

// time
pub use std::time::Duration;

// serialize
pub use serde::{Deserialize, Serialize};
