pub mod config;
pub mod current_location;
pub mod prelude;

pub use config::{
    Config, CONFIG_FILE_NAME, CONFIG_MIN_VERSION, CONFIG_VERSION, FileView, FullView, load_config,
    save_config,
};
pub use config::{ContestConfig, ContestDayConfig};
pub use config::problem::*;
pub use config::{contest, contestday, lang, migrate, msgs, problem};
pub use current_location::CurrentLocation;
