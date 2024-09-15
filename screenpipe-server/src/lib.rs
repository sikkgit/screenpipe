pub mod chunking;
pub mod cli;
pub mod core;
mod db;
pub mod filtering;
pub mod logs;
mod pipe_manager;
mod plugin;
mod resource_monitor;
mod server;
mod video;
mod video_db;
mod video_utils;
pub use cli::Cli;
pub use core::start_continuous_recording;
pub use db::{ContentSource, ContentType, DatabaseManager, SearchResult};
pub use logs::MultiWriter;
pub use pipe_manager::PipeManager;
pub use resource_monitor::{ResourceMonitor, RestartSignal};
pub use server::create_router;
pub use server::health_check;
pub use server::AppState;
pub use server::ContentItem;
pub use server::HealthCheckResponse;
pub use server::PaginatedResponse;
pub use server::Server;
pub use video::VideoCapture;
