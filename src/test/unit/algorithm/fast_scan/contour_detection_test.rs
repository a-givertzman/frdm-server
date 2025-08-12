#[cfg(test)]
use crate::{algorithm::{AutoBrightnessAndContrast, AutoBrightnessAndContrastCtx, AutoGamma, AutoGammaCtx, Context, ContextWrite, DetectingContoursCvCtx, EdgeDetectionCtx, EvalResult, Initial, InitialCtx, Side}, domain::{Eval, Image}};
use std::{sync::Once, time::{Duration, Instant}};
use opencv::{core::{self, Mat, MatTrait, Vec3b, Vector, ROTATE_90_CLOCKWISE}, highgui, imgcodecs, imgproc};
use sal_sync::services::conf::ConfTree;
use testing::stuff::max_test_duration::TestDuration;
use debugging::session::debug_session::{
    DebugSession, 
    LogLevel, 
    Backtrace
};
use sal_core::dbg::Dbg;
use walkdir::WalkDir;
use crate::{
    algorithm::{
        ContextRead, 
        DetectingContoursCv, 
        EdgeDetection,
    }, 
    conf::Conf,
};
///
///
static INIT: Once = Once::new();
///
/// once called initialisation
fn init_once() {
    INIT.call_once(|| {
        // implement your initialisation code to be called only once for current test file
    })
}
///
/// returns:
///  - ...
fn init_each() -> () {}
///
/// Testing 'eval'
#[test]
fn eval() {
    DebugSession::init(LogLevel::Debug, Backtrace::Short);
    init_once();
    init_each();
    let dbg = Dbg::own("eval");
    log::debug!("\n{}", dbg);
    let test_duration = TestDuration::new(&dbg, Duration::from_secs(1000));
    test_duration.run().unwrap();
    let conf = ConfTree::new_root(
        serde_yaml::from_str(&format!(r#"
            contours:
                gamma:
                    factor: 70.0              # percent of influence of [AutoGamma] algorythm bigger the value more the effect of [AutoGamma] algorythm, %
                brightness-contrast:
                    histogram-clipping: 1     # optional histogram clipping, default = 0 %
                gausian:
                    blur-size:
                        width: 7
                        height: 7
                    sigma-x: 0.0
                    sigma-y: 0.0
                sobel:
                    kernel-size: 3
                    scale: 1.0
                    delta: 0.0
                overlay:
                    src1-weight: 0.5
                    src2-weight: 0.5
                    gamma: 0.0
            edge-detection:
                threshold: 50                        # 0...255
            fast-scan:
                geometry-defect-threshold: 1.2      # 1.1..1.3, absolute threshold to detect the geometry deffects
            fine-scan:
                no-params: not implemented yet
        "#)).unwrap(),
    );
    let conf = Conf::new(&dbg, conf);
    let scan_rope = 
        EdgeDetection::new(
            conf.edge_detection.threshold,
            DetectingContoursCv::new(
                conf.contours.clone(),
                AutoBrightnessAndContrast::new(
                    conf.contours.brightness_contrast.histogram_clipping,
                    AutoGamma::new(
                        conf.contours.gamma.factor,
                        Initial::new(
                            InitialCtx::new(),
                        ),
                    ),
                ),
            )
        );
    let winp = "Otsu";
    let wgamma = "Gamma";
    let wbright = "Bright";
    let wcontours = "Contours";
    let wedges = "Edges";
    if let Err(err) = opencv::highgui::named_window(winp, opencv::highgui::WINDOW_NORMAL) {
        log::warn!("{}.stream | Create Window Error: {}", "dbg", err);
    }
    if let Err(err) = opencv::highgui::named_window(wgamma, opencv::highgui::WINDOW_NORMAL) {
        log::warn!("{}.stream | Create Window Error: {}", "dbg", err);
    }
    if let Err(err) = opencv::highgui::named_window(wbright, opencv::highgui::WINDOW_NORMAL) {
        log::warn!("{}.stream | Create Window Error: {}", "dbg", err);
    }
    if let Err(err) = opencv::highgui::named_window(wcontours, opencv::highgui::WINDOW_NORMAL) {
        log::warn!("{}.stream | Create Window Error: {}", "dbg", err);
    }
    if let Err(err) = opencv::highgui::named_window(wedges, opencv::highgui::WINDOW_NORMAL) {
        log::warn!("{}.stream | Create Window Error: {}", "dbg", err);
    }

    let image_dir = "/home/ilyarizo/deffect_photos/exp_gradient_rope_2diod/exp110_rope/retrived"; 

    for path in std::fs::read_dir(image_dir).unwrap().into_iter()
        .filter_map(|e| {
            let path = e.unwrap().path();
            path.is_file().then(|| path)
        })
    {
        match path.extension() {
            Some(ext) if ext == "jpg" || ext == "png" || ext == "jpeg" => {
                let frame_mat = imgcodecs::imread(
                    path.to_str().unwrap(),
                imgcodecs::IMREAD_COLOR,
                ).unwrap();
                let inp = frame_mat.clone();
                let mut rotated = Mat::default();
                core::rotate(&inp, &mut rotated, ROTATE_90_CLOCKWISE).unwrap();
                let mut res = rotated.clone();
                let src_frame = Image::with(rotated);
                let time = Instant::now();
                let ctx = scan_rope.eval(src_frame).unwrap();
                log::warn!("{dbg}.eval | Elapsed: {:?}", time.elapsed());
                let gamma: &AutoGammaCtx = ctx.read();
                let bright: &AutoBrightnessAndContrastCtx = ctx.read();
                let contours: &DetectingContoursCvCtx = ctx.read();
                let edges: &EdgeDetectionCtx = ctx.read();
                let edges_cont = contours.result.mat.clone();
                let upper = edges.result.get(Side::Upper);
                let lower = edges.result.get(Side::Lower);
                for dot in upper {
                    if dot.x >= 0 && dot.y >= 0 {
                        let x = dot.x as i32;
                        let y = dot.y as i32;
                        *res.at_2d_mut::<Vec3b>(y, x).unwrap() = Vec3b::from_array([0, 0, 255]);
                    }
                }
                for dot in lower {
                    if dot.x >= 0 && dot.y >= 0 {
                        let x = dot.x as i32;
                        let y = dot.y as i32;
                        *res.at_2d_mut::<Vec3b>(y, x).unwrap() = Vec3b::from_array([0, 255, 0]);
                    }
                }
                
                let mut ada = Mat::default();
                imgproc::adaptive_threshold(&contours.result.mat, &mut ada, 255.0, imgproc::ADAPTIVE_THRESH_MEAN_C, imgproc::THRESH_BINARY, 201, -20.0).unwrap();

                let mut hist = Mat::default();
                let hist_size = 256 as i32;
                // opencv::imgproc::calc_hist(
                //             &contours.result.mat,
                //             &Vector::from_slice(&[0]),
                //             &Mat::default(),
                //             &mut hist,
                //             &Vector::from_slice(&[hist_size]),
                //             &Vector::from_slice(&[0.0 ,255.0]),
                //             false,
                //         ).unwrap();
                // let mut transposed = Mat::default();
                // core::transpose(&inp, &mut transposed).unwrap();


                // highgui::imshow(winp, &rotated).unwrap();
                highgui::imshow(wgamma, &ada).unwrap();
                highgui::imshow(wbright, &bright.result.mat).unwrap();
                highgui::imshow(wcontours, &contours.result.mat).unwrap();
                highgui::imshow(wedges, &res).unwrap();
                // highgui::imshow(winp, &otsu).unwrap();
                highgui::wait_key(0).unwrap();
                highgui::destroy_all_windows().unwrap();
            },
            _ => continue,
        }



    }
    test_duration.exit();
}
///
/// Fake implements `Eval` for testing [EdgeDetection]
struct FakePassImg {}
impl FakePassImg{
    pub fn new() -> Self {
        Self {}
    }
}
//
//
impl Eval<Image, EvalResult> for FakePassImg {
    fn eval(&self, frame: Image) -> EvalResult {
        let ctx = Context::new(
            InitialCtx::new()
        );
        ctx.write(AutoBrightnessAndContrastCtx { result: frame })
    }
}