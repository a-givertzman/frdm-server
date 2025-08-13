use eframe::CreationContext;
use opencv::core::{MatTraitConst, MatTraitConstManual};
use sal_core::dbg::Dbg;
use sal_sync::collections::FxIndexMap;
use testing::entities::test_value::Value;
use std::{str::FromStr, sync::{Arc, Once}, time::Duration};
use egui::{
    vec2, Align2, Color32, ColorImage, FontFamily, FontId, TextStyle, TextureHandle, TextureOptions, Widget 
};
use crate::{algorithm::{AutoBrightnessAndContrast, AutoGamma, ContextRead, DetectingContoursCv, DetectingContoursCvCtx, EdgeDetection, Initial, InitialCtx}, conf::DetectingContoursConf, domain::{Eval, Image}};

///
/// 
static START: Once = Once::new();
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
                Param::new("EdgeDetection.threshold", ParamVal::IRange(0..255), Value::Int(20)),
                Param::new("Contours.gamma.factor", ParamVal::FRange(0.0..100.0), Value::Double(95.0)),
                Param::new("Contours....", ParamVal::IRange(0..100), Value::Int(0)),
                Param::new("AutoBrightnessContrast.histogram_clipping", ParamVal::IRange(0..100), Value::Int(0)),
                
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
    ///
    /// Create opencv Ui windows
    fn setup_opencv_windows(dbg: &Dbg, keys: Vec<impl Into<String>>) {
        for key in keys {
            if let Err(err) = opencv::highgui::named_window(&key.into(), opencv::highgui::WINDOW_NORMAL) {
                log::warn!("{}.stream | Create Window Error: {}", dbg, err);
            }
        }
        std::thread::spawn(|| {
            opencv::highgui::wait_key(0).unwrap();
        });
    }
}
//
//
impl eframe::App for UiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let window_origin = "Orgin";
        let window_contours = "DetectingContoursCv";
        let window_result = "Result";
        START.call_once(|| {
            Self::setup_opencv_windows(&self.dbg, vec![window_origin, window_result]);
        });
        let vp_size = ctx.input(|i| i.viewport().inner_rect).unwrap();
        let head_hight = 34.0;
        egui::Window::new("Parameters")
            .anchor(Align2::RIGHT_BOTTOM, [0.0, 0.0])
            .default_size([0.4 * vp_size.width(), 0.5 * vp_size.height() - head_hight])
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
                        ui.horizontal(|ui| {
                            ui.add_sized(
                                [128.0, 16.0 * 2.0 + 6.0], 
                                egui::Label::new(format!("{:?}\t|\t{:?}", i, param.key)),
                            );
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
                        });

                    }
                });
            });
        match Image::load(&self.path) {
            Ok(frame) => {
                egui::Window::new(format!("Image {window_origin}"))
                    .default_pos([10.0, 10.0])
                    .default_size([0.7 * vp_size.width(), 0.7 * vp_size.height() - head_hight])
                    .show(ctx, |ui| {
                        let texture_handle: TextureHandle = ui.ctx().load_texture(window_origin, color_image(&frame), TextureOptions::LINEAR);
                        ui.add(
                            egui::Image::new(&texture_handle)
                                .fit_to_fraction(egui::Vec2::new(1.0, 1.0))
                        );
                    });
                // if let Err(err) = opencv::highgui::imshow(window_origin, &frame.mat) {
                //     log::warn!("{}.stream | Display img error: {:?}", self.dbg, err);
                // };
                let contours_result = EdgeDetection::new(
                    self.params.get("EdgeDetection.threshold").unwrap().1.as_int() as u8,
                    DetectingContoursCv::new(
                        DetectingContoursConf::default(),
                        AutoBrightnessAndContrast::new(
                            self.params.get("AutoBrightnessContrast.histogram_clipping").unwrap().1.as_int() as i32,
                            AutoGamma::new(
                                self.params.get("Contours.gamma.factor").unwrap().1.as_double(),
                                Initial::new(
                                    InitialCtx::new(),
                                ),
                            ),
                        ),
                    ),
                )
                    .eval(frame.clone()).unwrap();
                let contours_ctx = ContextRead::<DetectingContoursCvCtx>::read(&contours_result);
                // if let Err(e) = opencv::highgui::imshow(window_result, &contours_ctx.result.mat) {
                //     log::error!("Display error: {}", e);
                // }
                egui::Window::new(format!("Image {window_result}"))
                    .default_pos([0.5 * vp_size.width(), 10.0])
                    .default_size([0.7 * vp_size.width(), 0.7 * vp_size.height() - head_hight])
                    .show(ctx, |ui| {
                        let texture_handle: TextureHandle = ui.ctx().load_texture(window_result, gray_image(&contours_ctx.result), TextureOptions::LINEAR);
                        ui.add(
                            egui::Image::new(&texture_handle)
                                .fit_to_fraction(egui::Vec2::new(1.0, 1.0))
                        );
                    });

                // let edge_detection_ctx = ContextRead::<EdgeDetectionCtx>::read(&contours_result);
                // if let Err(e) = opencv::highgui::imshow(window_result, &edge_detection_ctx.result.mat) {
                //     log::error!("Display error: {}", e);
                // }
            }
            Err(err) => log::error!("Read path '{}' error: {:?}", self.path, err),
        }

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
        // std::thread::sleep(Duration::from_secs(3));
    }
}
///
/// 
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
///
/// Returns color egui `Image` from `opencv::Mat`
fn color_image(frame: &Image) -> ColorImage {
    let mut pixels: Vec<u8> = Vec::with_capacity(frame.width * frame.height * 4); // For RGBA
    // Iterate over Mat pixels and convert BGR to RGBA
    // This is a simplified example; error handling and different Mat types need consideration.
    for y in 0..frame.height {
        for x in 0..frame.width {
            let pixel = frame.mat.at_2d::<opencv::core::Vec3b>(y as i32, x as i32).unwrap();
            pixels.push(pixel[2]); // R
            pixels.push(pixel[1]); // G
            pixels.push(pixel[0]); // B
            pixels.push(255);       // A (fully opaque)
        }
    }
    ColorImage::from_rgba_unmultiplied([frame.width, frame.height], &pixels)
}
///
/// Returns color egui `Image` from `opencv::Mat`
fn gray_image(frame: &Image) -> ColorImage {
    let mut pixels: Vec<u8> = Vec::with_capacity(frame.width * frame.height * 4); // For RGBA
    // Iterate over Mat pixels and convert BGR to RGBA
    // This is a simplified example; error handling and different Mat types need consideration.
    for y in 0..frame.height {
        for x in 0..frame.width {
            let pixel = frame.mat.at_2d::<opencv::core::VecN<u8, 1>>(y as i32, x as i32).unwrap();
            // pixels.push(pixel[2]); // R
            // pixels.push(pixel[1]); // G
            pixels.push(pixel[0]); // B
            // pixels.push(255);       // A (fully opaque)
        }
    }
    ColorImage::from_gray([frame.width, frame.height], &pixels)//rgba_unmultiplied([frame.width, frame.height], &pixels)
}
