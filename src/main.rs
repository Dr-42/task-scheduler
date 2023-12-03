mod app;
mod duration;
mod task;
mod time;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() {}
