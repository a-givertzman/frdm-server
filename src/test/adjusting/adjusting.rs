use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
use eframe::{EventLoopBuilder, UserEvent};
use sal_core::dbg::Dbg;
use crate::
    test::adjusting::ui_app::UiApp
;
///
/// Application entry point
#[test]
fn main() {
    DebugSession::init(LogLevel::Debug, Backtrace::Short);
    let dbg = Dbg::own("main");
    
    eframe::run_native(
        "Rpi-FFT-App", 
        eframe::NativeOptions {
            // fullscreen: true,
            // maximized: true,
            viewport: egui::ViewportBuilder::default()
                // .with_fullscreen(true),
                .with_inner_size([1920.0, 1100.0]),
            event_loop_builder: event_loop_builder(),
            ..Default::default()
        }, 
        Box::new(|cc| Ok(Box::new(
            UiApp::new(&dbg, "src/test/complex/testing_files/rope_0.jpeg", cc),
        )))
    ).unwrap();
}
///
/// Event buildeer for `eframe::run_native`
#[cfg(not(feature = "plot"))]
fn event_loop_builder() -> Option<Box<dyn FnOnce(&mut EventLoopBuilder<UserEvent>)>> {
    Some(Box::new(|event_loop_builder| {
        winit::platform::x11::EventLoopBuilderExtX11::with_any_thread(event_loop_builder, true);
    }))
}
#[cfg(feature = "plot")]
fn event_loop_builder() -> Option<Box<dyn FnOnce(&mut EventLoopBuilder<UserEvent>)>> {
    None
}
