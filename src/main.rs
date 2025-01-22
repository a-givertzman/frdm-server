mod domain;
mod infrostructure;
use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
#[cfg(test)]
mod test;
///
/// Appliacation entri point
fn main() {
    DebugSession::init(LogLevel::Debug, Backtrace::Short);
    println!("Hello, world!");
}
