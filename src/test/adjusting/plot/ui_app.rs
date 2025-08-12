use eframe::CreationContext;
use egui_plot::{Line, Plot, Points};
use sal_core::dbg::Dbg;
use sal_sync::collections::FxIndexMap;
use testing::entities::test_value::Value;
use std::{str::FromStr, sync::Arc};
use egui::{
    vec2,
    Color32, Align2, FontFamily, TextStyle, FontId, 
};
///
/// Variant of parameter value
enum ParamVal {
    IRange(std::ops::Range<i64>),
    FRange(std::ops::Range<f64>),
}
struct Param {
    pub key: String,
    pub val: ParamVal,
    pub default: Value,
}
impl Param {
    pub fn new(key: impl Into<String>, val: ParamVal, default: Value) -> Self {
        Self {
            key: key.into(),
            val,
            default,
        }
    }
}
///
/// Ui application for adjusting parameters of the graphic algorithms
pub struct UiApp {
    dbg: Dbg,
    path: String,
    conf: Vec<Param>,
    params: FxIndexMap<String, (String, Value)>,
}

impl UiApp {
    pub fn new(
        parent: impl Into<String>,
        path: impl  Into<String>,
        cc: &CreationContext,
        // renderDelay: Duration,
    ) -> Self {
        Self::setup_custom_fonts(&cc.egui_ctx);
        Self::configure_text_styles(&cc.egui_ctx);
        Self {
            dbg: Dbg::new(parent, "UiApp"),
            path: path.into(),
            conf: vec![
                Param::new("EdgeDetection.threshold", ParamVal::FRange(0.0..100.0), Value::Double(0.0)),
                Param::new("Contours.gamma.factor", ParamVal::IRange(0..100), Value::Int(0)),
                
            ],
            params: FxIndexMap::default(),
        }
    }
    ///
    fn setup_custom_fonts(ctx: &egui::Context) {
        // Start with the default fonts (we will be adding to them rather than replacing them).
        let mut fonts = egui::FontDefinitions::default();

        // Install my own font (maybe supporting non-latin characters).
        // .ttf and .otf files supported.
        fonts.font_data.insert(
            "Icons".to_owned(),
            Arc::new(egui::FontData::from_static(include_bytes!(
                "../../../../assets/fonts/icons.ttf"
            ))),
        );

        // Put my font first (highest priority) for proportional text:
        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(0, "Icons".to_owned());

        // Put my font as last fallback for monospace:
        fonts
            .families
            .entry(egui::FontFamily::Monospace)
            .or_default()
            .push("Icons".to_owned());

        // Tell egui to use these fonts:
        ctx.set_fonts(fonts);
    }
    ///
    fn configure_text_styles(ctx: &egui::Context) {
        use FontFamily::{Monospace, Proportional};
        let mut style = (*ctx.style()).clone();
        style.text_styles = [
            (TextStyle::Heading, FontId::new(24.0, Proportional)),
            // (heading2(), FontId::new(22.0, Proportional)),
            // (heading3(), FontId::new(19.0, Proportional)),
            (TextStyle::Body, FontId::new(16.0, Proportional)),
            (TextStyle::Monospace, FontId::new(12.0, Monospace)),
            (TextStyle::Button, FontId::new(16.0, Proportional)),
            (TextStyle::Small, FontId::new(8.0, Proportional)),
        ].into();
        ctx.set_style(style);
    }
    ///
    /// Converts string into `T` if posible
    fn parse<T: FromStr>(dbg: &Dbg, key: impl std::fmt::Display, text: &str, default: T) -> T where <T as FromStr>::Err: std::fmt::Debug {
        match text.parse() {
            Ok(val) => val,
            Err(err) => {
                log::warn!("{}.update | Error parse param '{}': {:?}", dbg, key, err);
                default
            }
        }
    }
}
//
//
impl eframe::App for UiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let vp_size = ctx.input(|i| i.viewport().inner_rect).unwrap();
        let head_hight = 34.0;
        egui::Window::new("Events")
            .anchor(Align2::RIGHT_BOTTOM, [0.0, 0.0])
            .default_size(vec2(0.4 * vp_size.width(), 0.5 * vp_size.height() - head_hight))
            .show(ctx, |ui| {
                // let btn = Button::image_and_text(
                //     Te
                //     "text"
                // );
                // if ui.button("Restart").clicked() {
                //     self.events.push("New event".to_string());
                // }
                let mut path = self.path.clone();
                ui.horizontal(|ui| {
                    ui.add_sized(
                        [32.0, 16.0 * 2.0 + 6.0], 
                        egui::Label::new(format!("image↕ ")), //⇔⇕   ↔
                    );
                    ui.separator();
                    if ui.add_sized([64.0, 16.0], egui::TextEdit::singleline(&mut path)).changed() {
                        self.path = path;
                    };                          
                });
                egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .stick_to_bottom(true)
                .show(ui, |ui| {
                    for (i, param) in self.conf.iter().enumerate() {
                        ui.label(format!("{:?}\t|\t{:?}", i, param.key));
                        ui.separator();
                        let (text, value) = self.params.entry(param.key.clone()).or_insert(match &param.val {
                            ParamVal::IRange(_) => (param.default.to_string(), Value::Int(param.default.as_int())),
                            ParamVal::FRange(_) => (param.default.to_string(), Value::Double(param.default.as_double())),
                        });
                        if ui.add_sized([64.0, 16.0], egui::TextEdit::singleline(text)).changed() {
                            match &param.val {
                                ParamVal::IRange(_) => *value = Value::Int(Self::parse(&self.dbg, &param.key, text, param.default.as_int())),
                                ParamVal::FRange(_) => *value = Value::Double(Self::parse(&self.dbg, &param.key, text, param.default.as_double())),
                            }
                        };                          
                    }
                });
            });

        // egui::Window::new("AnalyzeFft input").show(ctx, |ui| {
        //     let analyzeFft = self.analyzeFft.lock().unwrap();
        //     ui.label(format!(" t: {:?}", analyzeFft.t));
        //     ui.label(format!("t length: {}", analyzeFft.tList.len()));
        //     ui.label(format!("xyPoints length: {}", analyzeFft.xyPoints.len()));
        //     // ui.end_row();
        //     if ui.button("just button").clicked() {
        //     }
        //     Plot::new("input").show(ui, |plotUi| {
        //         plotUi.points(
        //             Points::new(
        //                 analyzeFft.xyPoints.buffer().clone(),
        //             ),
        //         )
        //     });
        // });
        ctx.request_repaint();
    }
}


pub trait ExtendedColors {
    const ORANGE: Color32 = Color32::from_rgb(255, 152, 0);
    const ORANGE_ACCENT: Color32 = Color32::from_rgb(255, 152, 0);
    const LIGHT_GREEN10: Color32 = Color32::from_rgba_premultiplied(0x90, 0xEE, 0x90, 10);
    fn with_opacity(&self, opacity: u8) -> Self;
}

impl ExtendedColors for Color32 {
    fn with_opacity(&self, opacity: u8) -> Self {
        let [r, g, b, _] = self.to_array();
        Color32::from_rgba_premultiplied(r, g, b, opacity)
    }
}
