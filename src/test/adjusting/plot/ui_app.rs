use eframe::CreationContext;
use opencv::core::{MatTrait, MatTraitConst};
use sal_core::dbg::Dbg;
use sal_sync::collections::FxIndexMap;
use testing::entities::test_value::Value;
use std::{str::FromStr, sync::{Arc, Once}, time::{Duration, Instant}};
use egui::{
    Align2, Color32, ColorImage, FontFamily, FontId, RichText, TextStyle, TextureHandle, TextureOptions, TopBottomPanel 
};
use crate::{algorithm::{AutoBrightnessAndContrast, AutoGamma, ContextRead, Cropping, CroppingConf, DetectingContoursCv, DetectingContoursCvCtx, EdgeDetection, EdgeDetectionCtx, Initial, InitialCtx, Side, Threshold}, conf::{BrightnessContrastConf, Conf, DetectingContoursConf, EdgeDetectionConf, FastScanConf, FineScanConf, GammaConf, GausianConf, OverlayConf, SobelConf}, domain::{Dot, Eval, Image}};

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
    rotate: bool,
    show_images: bool,
    conf: Vec<Param>,
    params: FxIndexMap<String, (String, Value)>,
    zoom: f32,
    start_pos: egui::Pos2,
    end_pos: egui::Pos2,
    origin: Image,
    frame: Image,
    contour_frame: Option<Image>,
    result_frame: Option<Image>,
    is_changed: usize,
    alg_err: Option<String>,
    elapsed: Option<Duration>,
}
//
//
impl UiApp {
    pub fn new(
        parent: impl Into<String>,
        path: impl  Into<String>,
        cc: &CreationContext,
        // renderDelay: Duration,
    ) -> Self {
        let dbg = Dbg::new(parent, "UiApp");
        Self::setup_custom_fonts(&cc.egui_ctx);
        Self::configure_text_styles(&cc.egui_ctx);
        let path = path.into();
        let rotate= true;
        let (origin, frame, is_changed) = match Image::load(&path) {
            Ok(frame) => {
                match rotate {
                    true => {
                        let mut rotated = opencv::core::Mat::default();
                        opencv::core::rotate(&frame.mat, &mut rotated, opencv::core::ROTATE_90_CLOCKWISE).unwrap();
                        (frame, Image::with(rotated), 3)
                    }
                    false => (frame.clone(), frame, 3),
                }
            }
            Err(err) => {
                log::error!("{dbg}.new | Read path '{}' error: {:?}", path, err);
                (Image::with(opencv::core::Mat::default()), Image::with(opencv::core::Mat::default()), 0)
            }
        };
        Self {
            dbg,
            path,
            rotate,
            show_images: false,
            conf: vec![
                Param::new("Contours.cropping.x",                           ParamVal::IRange(0..6000),      Value::Int(0)),
                Param::new("Contours.cropping.width",                       ParamVal::IRange(0..6000),      Value::Int(1900)),
                Param::new("Contours.cropping.y",                           ParamVal::IRange(0..6000),      Value::Int(0)),
                Param::new("Contours.cropping.height",                      ParamVal::IRange(0..6000),      Value::Int(1200)),

                Param::new("BrightnessContrast.histogram_clipping",         ParamVal::IRange(0..100),       Value::Int(1)),

                Param::new("Contours.gamma.factor",                         ParamVal::FRange(1.1..100.0),   Value::Double(95.0)),

                Param::new("Contours.gausian.blur_w",                       ParamVal::IRange(0..100),       Value::Int(7)),
                Param::new("Contours.gausian.blur_h",                       ParamVal::IRange(0..100),       Value::Int(7)),
                Param::new("Contours.gausian.sigma_x",                      ParamVal::FRange(0.0..100.0),   Value::Double(0.0)),
                Param::new("Contours.gausian.sigma_y",                      ParamVal::FRange(0.0..100.0),   Value::Double(0.0)),
                
                Param::new("Contours.sobel.kernel_size",                    ParamVal::IRange(0..100),   Value::Int(3)),
                Param::new("Contours.sobel.scale",                          ParamVal::FRange(0.0..100.0),   Value::Double(11.0)),
                Param::new("Contours.sobel.delta",                          ParamVal::FRange(0.0..100.0),   Value::Double(0.0)),
                
                Param::new("Contours.overlay.src1_weight",                  ParamVal::FRange(0.0..100.0),   Value::Double(0.5)),
                Param::new("Contours.overlay.src2_weight",                  ParamVal::FRange(0.0..100.0),   Value::Double(0.5)),
                Param::new("Contours.overlay.gamma",                        ParamVal::FRange(0.0..100.0),   Value::Double(0.0)),

                Param::new("EdgeDetection.threshold",                       ParamVal::IRange(0..255),       Value::Int(20)),
                Param::new("FastScan.threshold",                            ParamVal::FRange(0.0..100.0),   Value::Double(1.2)),
            ],
            params: FxIndexMap::default(),
            zoom: 1.0,
            start_pos: egui::pos2(0.0, 0.0),
            end_pos: egui::pos2(100.0, 100.0),
            origin,
            frame,
            contour_frame: None,
            result_frame: None,
            is_changed,
            alg_err: None,
            elapsed: None,
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
        opencv::highgui::wait_key(1).unwrap();

        // std::thread::spawn(|| {
        //     opencv::highgui::wait_key(0).unwrap();
        // });
    }
    ///
    /// Adds an Image to Ui
    fn display_image_window(&mut self, ctx: &egui::Context, title: impl Into<String>, size: impl Into<egui::Vec2>, pos: impl Into<egui::Pos2>, frame: &Image) {
        if self.show_images {
            let title = title.into();
            egui::Window::new(format!("Image {title}"))
                .default_pos(pos)
                .default_size(size)
                .scroll(true)
                .show(ctx, |ui| {
                    let zoom_delta = ui.input(|i| i.zoom_delta());
                    if zoom_delta != 1.0 {
                        if zoom_delta > 1.0 {
                            self.zoom = self.zoom * 1.02;
                        } else {
                            self.zoom = self.zoom * 0.98;
                        }
                    }
                    // log::debug!("display_image_window | {title}: {},  delta: {zoom_delta}", self.zoom);
                    let texture_handle: TextureHandle = ui.ctx().load_texture(title, image(&frame), TextureOptions::LINEAR);
                    let mut scene_rect = ctx.input(|x| {
                        x.viewport().inner_rect.unwrap_or(egui::Rect::ZERO)
                    });
                    let scale_factor = 1.0 / ctx.zoom_factor();
                    let image = egui::Image::new(&texture_handle)
                        .fit_to_exact_size([(frame.width as f32) * self.zoom, (frame.height as f32) * self.zoom].into());
                        // .shrink_to_fit()
                        // .sense(egui::Sense::all());
                        // .fit_to_fraction(egui::Vec2::new(1.0, 1.0))
                    ui.add(
                        image
                    );
                    // let rect = egui::Rect::from_two_pos(self.start_pos, self.end_pos);
                    // let rect = egui::Rect::from_min_size(
                    //         egui::pos2(rect.min.x * scale_factor, rect.min.y * scale_factor),
                    //         egui::vec2(
                    //             rect.width() * scale_factor,
                    //             rect.height() * scale_factor,
                    //         ),
                    //     );
                    //     egui::Scene::new().sense(egui::Sense::all()).show(ui, &mut rect, |ui| {
                    //     let image = egui::Image::new(&texture_handle);
                    //     // .fit_to_exact_size([(frame.width as f32) * self.zoom, (frame.height as f32) * self.zoom].into())
                    //         // .shrink_to_fit()
                    //         // .sense(egui::Sense::all());
                    //         // .fit_to_fraction(egui::Vec2::new(1.0, 1.0))
                    //     ui.add(
                    //         image
                    //     );
                    // });
                    ui.set_width(scene_rect.width());
                    ui.set_height(scene_rect.height());
                });
        }
    }
    ///
    /// Returns Image with array of dots
    fn image_plot(frame: &Image, dots: Vec<Dot<usize>>, color: [u8; 3], cropping: &CroppingConf) -> Image {
        let mut res = frame.clone();
        for dot in dots {
            *res.mat.at_2d_mut::<opencv::core::Vec3b>(dot.y as i32 + cropping.y, dot.x as i32 + cropping.x).unwrap() = opencv::core::Vec3b::from_array(color);
        }
        res
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
        if let Some(vp_size) = ctx.input(|i| i.viewport().inner_rect) {
            let head_hight = 34.0;
            let mut path_error = None;
            egui::TopBottomPanel::bottom("StatusBar").exact_height(32.0).show(ctx, |ui| ui.horizontal(|ui| {
                ui.add(egui::Label::new(format!("Image: {} x {}", self.frame.width, self.frame.height)));
                ui.separator();
                match self.elapsed {
                    Some(elapsed) => ui.add(egui::Label::new(format!("Elapse: {:?}", elapsed))),
                    None => ui.add(egui::Label::new(format!("Elapse: ---"))),
                };
            }));
            egui::SidePanel::left("Parameters")
                .default_width(700.0)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.add(egui::Label::new(format!("Image↕ ")));
                        ui.separator();
                        if ui.add_sized([ui.available_width() - 4.0, 24.0], egui::TextEdit::singleline(&mut self.path)).changed() {
                            match Image::load(&self.path) {
                                Ok(frame) => {
                                    self.origin = frame.clone();
                                    match self.rotate {
                                        true => {
                                            let mut rotated = opencv::core::Mat::default();
                                            opencv::core::rotate(&self.origin.mat, &mut rotated, opencv::core::ROTATE_90_CLOCKWISE).unwrap();
                                            self.frame = Image::with(rotated);
                                        }
                                        false => self.frame = frame,
                                    }
                                    self.is_changed = 2;
                                }
                                Err(err) => {
                                    log::error!("Read path '{}' error: {:?}", self.path, err);
                                    path_error = Some(format!("Read path '{}' error: {:?}", self.path, err));
                                }
                            }
                        };
                    });
                    ui.horizontal(|ui| {
                        if ui.add(egui::Checkbox::new(&mut self.rotate, "Rotate")).changed() {
                            match self.rotate {
                                true => {
                                    let mut rotated = opencv::core::Mat::default();
                                    opencv::core::rotate(&self.origin.mat, &mut rotated, opencv::core::ROTATE_90_CLOCKWISE).unwrap();
                                    self.frame = Image::with(rotated);
                                }
                                false => self.frame = self.origin.clone(),
                            }
                            self.is_changed = 2;
                        };
                        ui.separator();
                        if ui.add(egui::Checkbox::new(&mut self.show_images, "Show images")).changed() {
                            self.is_changed = 2;
                        };
                    });
                    if let Some(path_err) = path_error {
                        ui.horizontal(|ui| ui.add(egui::Label::new(RichText::new(path_err).color(Color32::ORANGE_ACCENT))));
                    }
                    if let Some(alg_err) = &self.alg_err {
                        ui.horizontal(|ui| ui.add(egui::Label::new(RichText::new(alg_err).color(Color32::ORANGE_ACCENT))));
                    }
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
                                    if ui.add(egui::TextEdit::singleline(text)).changed() {
                                        match &param.val {
                                            ParamVal::IRange(_) => *value = Value::Int(Self::parse(&self.dbg, &param.key, text, param.default.as_int())),
                                            ParamVal::FRange(_) => *value = Value::Double(Self::parse(&self.dbg, &param.key, text, param.default.as_double())),
                                        }
                                        self.is_changed = 2;
                                    };                          
                                });
                            }
                        });
                });
            if self.is_changed > 0 {
                self.is_changed -= 1;
                // self.display_image_window(ctx, window_origin, [0.45 * vp_size.width(), 0.45 * vp_size.height() - head_hight], [10.0, 10.0], &frame);
                let cropping_x = self.params.get("Contours.cropping.x").unwrap().1.as_int() as i32;
                let cropping_width = self.params.get("Contours.cropping.width").unwrap().1.as_int() as i32;
                let cropping_y = self.params.get("Contours.cropping.y").unwrap().1.as_int() as i32;
                let cropping_height = self.params.get("Contours.cropping.height").unwrap().1.as_int() as i32;
                let conf = Conf {
                    contours: DetectingContoursConf {
                        cropping: CroppingConf {
                            x: cropping_x,
                            width: if cropping_x + cropping_width <= self.frame.width as i32 {cropping_width} else {self.frame.width as i32 - cropping_x},
                            y: cropping_y,
                            height: if cropping_y + cropping_height <= self.frame.height as i32 {cropping_height} else {self.frame.height as i32 - cropping_y},
                        },
                        gamma: GammaConf {
                            factor: self.params.get("Contours.gamma.factor").unwrap().1.as_double(),
                        },
                        brightness_contrast: BrightnessContrastConf {
                            histogram_clipping: self.params.get("BrightnessContrast.histogram_clipping").unwrap().1.as_int() as i32,
                        },
                        gausian: GausianConf {
                            blur_w: self.params.get("Contours.gausian.blur_w").unwrap().1.as_int() as i32,
                            blur_h: self.params.get("Contours.gausian.blur_h").unwrap().1.as_int() as i32,
                            sigma_x: self.params.get("Contours.gausian.sigma_x").unwrap().1.as_double(),
                            sigma_y: self.params.get("Contours.gausian.sigma_y").unwrap().1.as_double(),
                        },
                        sobel: SobelConf {
                            kernel_size: self.params.get("Contours.sobel.kernel_size").unwrap().1.as_int() as i32,
                            scale: self.params.get("Contours.sobel.scale").unwrap().1.as_double(),
                            delta: self.params.get("Contours.sobel.delta").unwrap().1.as_double(),
                        },
                        overlay: OverlayConf {
                            src1_weight: self.params.get("Contours.overlay.src1_weight").unwrap().1.as_double(),
                            src2_weight: self.params.get("Contours.overlay.src2_weight").unwrap().1.as_double(),
                            gamma: self.params.get("Contours.overlay.gamma").unwrap().1.as_double(),
                        },
                    },
                    edge_detection: EdgeDetectionConf {
                        threshold: self.params.get("EdgeDetection.threshold").unwrap().1.as_int() as u8,
                    },
                    fast_scan: FastScanConf {
                        geometry_defect_threshold: Threshold(self.params.get("FastScan.threshold").unwrap().1.as_double()),
                    },
                    fine_scan: FineScanConf::default(),
                };
                let t = Instant::now();
                let result_ctx = EdgeDetection::new(
                    conf.edge_detection.threshold,
                    DetectingContoursCv::new(
                        conf.contours.clone(),
                        AutoBrightnessAndContrast::new(
                            conf.contours.brightness_contrast.histogram_clipping,
                            AutoGamma::new(
                                conf.contours.gamma.factor,
                                Cropping::new(
                                    conf.contours.cropping.x,
                                    conf.contours.cropping.width,
                                    conf.contours.cropping.y,
                                    conf.contours.cropping.height,
                                    Initial::new(
                                        InitialCtx::new(),
                                    ),
                                ),
                            ),
                        ),
                    ),
                ).eval(self.frame.clone());
                match result_ctx {
                    Ok(result_ctx) => {
                        self.elapsed = Some(t.elapsed());
                        self.alg_err = None;
                        let contours_ctx: &DetectingContoursCvCtx = result_ctx.read();
                        self.contour_frame = Some(contours_ctx.result.clone());
                        let edges: &EdgeDetectionCtx = result_ctx.read();
                        let upper = edges.result.get(Side::Upper);
                        let result_img = Self::image_plot(&self.frame, upper, [0, 0, 255], &conf.contours.cropping);
                        let lower = edges.result.get(Side::Lower);
                        let result_img = Self::image_plot(&result_img, lower, [0, 255, 0], &conf.contours.cropping);
                        self.result_frame = Some(result_img)
                    }
                    Err(err) => {
                        self.alg_err = Some(format!("Error in the algorithms: {err}"));
                        self.elapsed = None;
                    }
                }
                
            }
            if let Some(frame) = self.contour_frame.clone() {
                self.display_image_window(ctx, window_contours, [0.45 * vp_size.width(), 0.45 * vp_size.height() - head_hight], [10.0, 0.5 * vp_size.height()], &frame);
                opencv::highgui::imshow(window_contours, &frame.mat).unwrap();
                opencv::highgui::wait_key(1).unwrap();
            }
            if let Some(frame) = self.result_frame.clone() {
                self.display_image_window(ctx, window_result, [0.70 * vp_size.width(), 0.70 * vp_size.height() - head_hight], [10.0, 10.0], &frame);
                opencv::highgui::imshow(window_result, &frame.mat).unwrap();
                opencv::highgui::wait_key(1).unwrap();
            }
        }
        ctx.request_repaint();
        // std::thread::sleep(Duration::from_millis(500));
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
/// Returns egui `Image` from `opencv::Mat`
fn image(frame: &Image) -> ColorImage {
    let mut pixels: Vec<u8> = Vec::with_capacity(frame.width * frame.height * 4); // For RGBA
    // Iterate over Mat pixels and convert BGR to RGBA
    // This is a simplified example; error handling and different Mat types need consideration.
    if frame.mat.channels() == 3 {
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
    } else if frame.mat.channels() == 1 {
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
    } else {
        log::warn!("image | Unsupported image format {} with {} channels", frame.mat.typ(), frame.mat.channels());
        ColorImage::from_rgba_unmultiplied([frame.width, frame.height], &pixels)
    }
}
