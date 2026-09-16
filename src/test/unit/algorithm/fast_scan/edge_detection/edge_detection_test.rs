#[cfg(test)]

use std::{sync::Once, time::Duration};
use opencv::{core::{Mat, MatTrait, Vec3b}, highgui, imgcodecs, imgproc};
use sal_core::{dbg::Dbg, error::Error};
use testing::stuff::max_test_duration::TestDuration;
use debugging::session::{DebugSession, LogLevel};
use crate::{algorithm::{Context, ContextRead, ContextWrite, FastContoursCtx, FastEdges, FastEdgesCtx, InitialCtx, Edges, Side}, domain::{Dot, Eval, Image}};
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
/// Visualize upper and lower edges on test image
#[allow(unused)]
fn edge_visualization_img() {
    let path = "src/test/unit/scan/edge_detection/test_photo2.png";
    let img = imgcodecs::imread(
        path,
        imgcodecs::IMREAD_GRAYSCALE,
    ).unwrap();
    let ctx = FastEdges::new(None, Some(1), None, FakePassImg::new()).eval(Image::from(img.clone(), 0)).unwrap();
    let edges: &FastEdgesCtx = ctx.read();
    let mut img_of_edges = imgcodecs::imread(
        path,
        imgcodecs::IMREAD_COLOR,
    ).unwrap();
    for dot in edges.edges.get(Side::Upper) {
        if dot.x as i32 >= 0 && dot.y as i32 >= 0 {
            let x = dot.x as i32;
            let y = dot.y as i32;
            *img_of_edges.at_2d_mut::<Vec3b>(y, x).unwrap() = Vec3b::from_array([0, 0, 255]);
        }
    }
    for dot in edges.edges.get(Side::Lower) {
        if dot.x as i32 >= 0 && dot.y as i32 >= 0 {
            let x = dot.x as i32;
            let y = dot.y as i32;
            *img_of_edges.at_2d_mut::<Vec3b>(y, x).unwrap() = Vec3b::from_array([0, 255, 0]);
        }
    }
    highgui::named_window("img", highgui::WINDOW_AUTOSIZE).unwrap();
    highgui::imshow("img", &img_of_edges).unwrap();
    highgui::wait_key(0).unwrap();
    highgui::destroy_all_windows().unwrap();
}
///
/// Visualize upper and lower edges on test matrix
fn edge_visualization_matrix(matrix: [[u8; 6]; 6]) {
    let matrix: Vec<Vec<u8>> = matrix.iter()
    .map(|row| row.iter().map(|&x| x * 255).collect())
    .collect();
    let img = Mat::from_slice_2d(&matrix).unwrap();
    let mut img_of_edges = Mat::default();
    imgproc::cvt_color(&img, &mut img_of_edges, imgproc::COLOR_GRAY2BGR, 0).unwrap();
    let ctx = FastEdges::new(None, Some(1), None, FakePassImg::new()).eval(Image::from(img, 0)).unwrap();
    let edges: &FastEdgesCtx = ctx.read();
    for dot in edges.edges.get(Side::Upper) {
        if dot.x as i32 >= 0 && dot.y as i32 >= 0 {
            let x = dot.x as i32;
            let y = dot.y as i32;
            *img_of_edges.at_2d_mut::<Vec3b>(y, x).unwrap() = Vec3b::from_array([0, 0, 255]);
        }
    }
    for dot in edges.edges.get(Side::Lower) {
        if dot.x as i32 >= 0 && dot.y as i32 >= 0 {
            let x = dot.x as i32;
            let y = dot.y as i32;
            *img_of_edges.at_2d_mut::<Vec3b>(y, x).unwrap() = Vec3b::from_array([0, 255, 0]);
        }
    }
    highgui::named_window("img", highgui::WINDOW_AUTOSIZE).unwrap();
    highgui::imshow("img", &img_of_edges).unwrap();
    highgui::wait_key(0).unwrap();
    highgui::destroy_all_windows().unwrap();
}
///
/// Testing FastEdges.eval
#[test]
fn edge_detection() {
    DebugSession::new().filter(LogLevel::Debug).init().unwrap();
    //
    // to visualize matrix use:
    let visualize_matrix = false;
    init_once();
    init_each();
    let dbg = Dbg::own("test");
    log::debug!("\n{}", dbg);
    let test_duration = TestDuration::new(dbg, Duration::from_secs(100));
    test_duration.run().unwrap();
    fn into_dots(dots: &[usize]) -> Vec<Dot<usize>> {
        dots.chunks(2).map(|d| {
            let dot: &[usize; 2] = d.try_into().unwrap();
            Dot::from(dot)
        }).collect()
    }
    let test_data: [(i32, Image, Result<FastEdgesCtx, Error>); 2] = [
        (
            1,
            Image::from( Mat::from_slice_2d(&MATRIX1).unwrap(), 0),
            Ok(FastEdgesCtx {
                edges: Edges::new(
                    into_dots(&[0,1, 1,0, 2,0, 3,1, 4,0, 5,0]),
                    into_dots(&[0,5, 1,4, 2,5, 3,5, 4,5, 5,4]),
                )
            }),
        ),
        (
            2,
            Image::from( Mat::from_slice_2d(&MATRIX2).unwrap(), 0),
            Ok(FastEdgesCtx {
                edges: Edges::new(
                    into_dots(&[0,2, 1,1, 2,0, 3,1, 4,0, 5,1]),
                    into_dots(&[0,3, 1,4, 2,4, 3,5, 4,4, 5,3]),
                )
            }),
        )
    ];
    for (step, img, target) in test_data {
        let result = FastEdges::new(
            None,
            Some(1),
            None,
            FakePassImg::new()
        )
        .eval(img)
        .map(|ctx| {
            let result: &FastEdgesCtx = ctx.read();
            result.to_owned()
        });
        match (result, target) {
            (Ok(result), Ok(target)) => {
                assert!(result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
            }
            (Ok(result), Err(target)) => panic!("step {} \nresult: {:?}\ntarget: {:?}", step, result, target),
            (Err(result), Ok(target)) => panic!("step {} \nresult: {:?}\ntarget: {:?}", step, result, target),
            (Err(_), Err(_)) => {},
        }
    }
    test_duration.exit();
    if visualize_matrix {
        edge_visualization_matrix(MATRIX2);
    }
    static MATRIX1: [[u8; 6]; 6] = [
        [0, 1, 1, 0, 1, 1],
        [1, 0, 0, 1, 0, 0],
        [0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0],
        [0, 1, 0, 0, 0, 1],
        [1, 0, 1, 1, 1, 0],
    ];
    static MATRIX2: [[u8; 6]; 6] = [
        [0, 0, 1, 0, 1, 0],
        [0, 1, 1, 1, 1, 1],
        [1, 1, 1, 1, 1, 1],
        [1, 1, 1, 1, 1, 1],
        [0, 1, 1, 1, 1, 0],
        [0, 0, 0, 1, 0, 0],
    ];
}
///
/// Fake implements `Eval` for testing [FastEdges]
struct FakePassImg {}
impl FakePassImg{
    pub fn new() -> Self {
        Self {}
    }
}
//
//
impl Eval<Image, Result<Context, Error>> for FakePassImg {
    fn eval(&self, frame: Image) -> Result<Context, Error> {
        let ctx = Context::new(
            InitialCtx::new(),
        );
        ctx.write(FastContoursCtx { result: frame })
    }
}
