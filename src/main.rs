mod domain;
mod infrostructure;
mod conf;
#[cfg(test)]
mod test;
use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
///
/// Appliacation entri point
fn main() {
    DebugSession::init(LogLevel::Debug, Backtrace::Short);
    println!("Hello, world!");
}
