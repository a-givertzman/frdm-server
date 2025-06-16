#[cfg(test)]

mod geometry_defect {
    use std::{
        sync::Once, 
        time::Duration
    };
    use indexmap::IndexMap;
    use sal_core::dbg::Dbg;
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{
        DebugSession, 
        LogLevel, 
        Backtrace
    };
    use opencv::{
        core::{Point, Scalar, Vector, CV_8UC3},
        imgproc,
        prelude::*,
    };
    use crate::{
        algorithm::{
            geometry_defect::{
                GeometryDefect, GeometryDefectCtx, GeometryDefectType, Threshold
            }, mad::{
                Bond, 
                Mad
            }, width_emissions::WidthEmissions, Context, ContextRead, ContextWrite, EvalResult, InitialCtx, InitialPoints, Side
        }, 
        domain::{
            graham::dot::Dot, 
            Eval
        }, 
        infrostructure::arena::Image
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
    /// Visualizing result
    fn visualize_with_opencv(
        test_case: usize,
        initial_points: &InitialPoints<usize>,
        defects: &[GeometryDefectType],
    ) -> Result<(), Box<dyn std::error::Error>> {
        let width = 1200;
        let height = 800;
        let mut img = Mat::zeros(height, width, CV_8UC3)?.to_mat()?;
        img.set_to(&Scalar::all(255.0), &opencv::core::no_array())?;
        let scale_x = width as f32 / 120.0;
        let scale_y = height as f32 / 150.0;
        let blue = Scalar::new(255.0, 0.0, 0.0, 0.0);
        let black = Scalar::new(0.0, 0.0, 0.0, 0.0);        
        if let Some(upper) = initial_points.sides.get(&Side::Upper) {
            for dot in upper {
                let x = (dot.x as f32 * scale_x) as i32;
                let y = height - (dot.y as f32 * scale_y) as i32;
                imgproc::circle(
                    &mut img,
                    Point::new(x, y),
                    5,
                    black,
                    -1,
                    imgproc::LINE_8,
                    0,
                )?;
            }
        }
        if let Some(lower) = initial_points.sides.get(&Side::Lower) {
            for dot in lower {
                let x = (dot.x as f32 * scale_x) as i32;
                let y = height - (dot.y as f32 * scale_y) as i32;
                imgproc::circle(
                    &mut img,
                    Point::new(x, y),
                    5,
                    black,
                    -1,
                    imgproc::LINE_8,
                    0,
                )?;
            }
        }
        for defect in defects {
            let (x, y) = match defect {
                GeometryDefectType::Expansion(bond) => (bond.x, bond.y),
                GeometryDefectType::Contraction(bond) => (bond.x, bond.y),
                GeometryDefectType::Mound(bond) => (bond.x, bond.y),
                GeometryDefectType::Groove(bond) => (bond.x, bond.y),
            };
            let px = (x as f32 * scale_x) as i32;
            let py = height - (y as f32 * scale_y) as i32;
            imgproc::line(
                &mut img,
                Point::new(px-5, py-5),
                Point::new(px+5, py+5),
                blue,
                2,
                imgproc::LINE_8,
                0,
            )?;
            imgproc::line(
                &mut img,
                Point::new(px+5, py-5),
                Point::new(px-5, py+5),
                blue,
                2,
                imgproc::LINE_8,
                0,
            )?;
        }
        let output_path = format!("src/test/unit/algorithm/geometry_defect/output_files/test_case_{}.png", test_case);
        opencv::imgcodecs::imwrite(&output_path, &img, &Vector::new())?;
        Ok(())
    }
    ///
    /// Testing `eval`
    #[test]
    fn eval() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        let dbg = Dbg::own("geometry_defect");
        log::debug!("\n{}", dbg);
        let test_duration = TestDuration::new(&dbg, Duration::from_secs(1));
        test_duration.run().unwrap();
        let test_data = [
            (
                1,
                Threshold(1.1),
                InitialPoints {
                    sides: IndexMap::from(
                        [
                            (
                                Side::Upper,
                                vec![
                                    Dot { x: 10  , y: 100 },
                                    Dot { x: 20  , y: 105 },
                                    Dot { x: 30  , y: 110 },
                                    Dot { x: 40  , y: 120 },
                                    Dot { x: 50  , y: 130 },
                                    Dot { x: 60  , y: 135 },
                                    Dot { x: 70  , y: 130 },
                                    Dot { x: 80  , y: 120 },
                                    Dot { x: 90  , y: 110 },
                                    Dot { x: 100 , y: 105 },
                                    Dot { x: 110 , y: 100 },
                                ],
                            ),
                            (
                                Side::Lower,
                                vec![
                                    Dot { x: 10  , y: 50 },
                                    Dot { x: 20  , y: 45 },
                                    Dot { x: 30  , y: 40 },
                                    Dot { x: 40  , y: 30 },
                                    Dot { x: 50  , y: 20 },
                                    Dot { x: 60  , y: 15 },
                                    Dot { x: 70  , y: 20 },
                                    Dot { x: 80  , y: 30 },
                                    Dot { x: 90  , y: 40 },
                                    Dot { x: 100 , y: 45 },
                                    Dot { x: 110 , y: 50 },
                                ],
                            ),
                        ]
                    )
                },
                vec![
                    GeometryDefectType::Expansion(Bond { x: 50, y: 130 }), 
                    GeometryDefectType::Expansion(Bond { x: 50, y: 20 }), 
                    GeometryDefectType::Expansion(Bond { x: 60, y: 135 }), 
                    GeometryDefectType::Expansion(Bond { x: 60, y: 15 }),
                    GeometryDefectType::Expansion(Bond { x: 70, y: 130 }), 
                    GeometryDefectType::Expansion(Bond { x: 70, y: 20 })
                ]
            ),
            (
                2,
                Threshold(1.1),
                InitialPoints {
                    sides: IndexMap::from(
                        [
                            (
                                Side::Upper,
                                vec![
                                    Dot { x: 10  , y: 100 },
                                    Dot { x: 20  , y: 105 },
                                    Dot { x: 30  , y: 110 },
                                    Dot { x: 40  , y: 85 },
                                    Dot { x: 50  , y: 80 },
                                    Dot { x: 60  , y: 75 },
                                    Dot { x: 70  , y: 70 },
                                    Dot { x: 80  , y: 85 },
                                    Dot { x: 90  , y: 110 },
                                    Dot { x: 100 , y: 105 },
                                    Dot { x: 110 , y: 100 },
                                ],
                            ),
                            (
                                Side::Lower,
                                vec![
                                    Dot { x: 10  , y: 50 },
                                    Dot { x: 20  , y: 45 },
                                    Dot { x: 30  , y: 40 },
                                    Dot { x: 40  , y: 70 },
                                    Dot { x: 50  , y: 80 },
                                    Dot { x: 60  , y: 73 },
                                    Dot { x: 70  , y: 68 },
                                    Dot { x: 80  , y: 40 },
                                    Dot { x: 90  , y: 40 },
                                    Dot { x: 100 , y: 45 },
                                    Dot { x: 110 , y: 50 },
                                ],
                            )
                        ]
                    )
                },
                vec![
                    GeometryDefectType::Contraction(Bond { x: 40, y: 85 }), 
                    GeometryDefectType::Contraction(Bond { x: 40, y: 70 }), 
                    GeometryDefectType::Contraction(Bond { x: 50, y: 80 }), 
                    GeometryDefectType::Contraction(Bond { x: 50, y: 80 }), 
                    GeometryDefectType::Contraction(Bond { x: 60, y: 75 }), 
                    GeometryDefectType::Contraction(Bond { x: 60, y: 73 }), 
                    GeometryDefectType::Contraction(Bond { x: 70, y: 70 }), 
                    GeometryDefectType::Contraction(Bond { x: 70, y: 68 })
                ]
            ),
            (
                3,
                Threshold(1.1),
                InitialPoints {
                    sides: IndexMap::from(
                        [
                            (
                                Side::Upper,
                                vec![
                                    Dot { x: 10  , y: 100 },
                                    Dot { x: 20  , y: 105 },
                                    Dot { x: 30  , y: 110 },
                                    Dot { x: 40  , y: 85 },
                                    Dot { x: 50  , y: 80 },
                                    Dot { x: 60  , y: 75 },
                                    Dot { x: 70  , y: 70 },
                                    Dot { x: 80  , y: 85 },
                                    Dot { x: 90  , y: 110 },
                                    Dot { x: 100 , y: 105 },
                                    Dot { x: 110 , y: 100 },
                                ],
                            ),
                            (
                                Side::Lower,
                                vec![
                                    Dot { x: 10  , y: 50 },
                                    Dot { x: 20  , y: 45 },
                                    Dot { x: 30  , y: 46 },
                                    Dot { x: 40  , y: 50 },
                                    Dot { x: 50  , y: 50 },
                                    Dot { x: 60  , y: 53 },
                                    Dot { x: 70  , y: 58 },
                                    Dot { x: 80  , y: 50 },
                                    Dot { x: 90  , y: 46 },
                                    Dot { x: 100 , y: 45 },
                                    Dot { x: 110 , y: 50 },
                                ],
                            )
                        ]
                    )
                },
                vec![
                    GeometryDefectType::Mound(Bond { x: 50, y: 80 }), 
                    GeometryDefectType::Mound(Bond { x: 60, y: 75 }), 
                    GeometryDefectType::Contraction(Bond { x: 70, y: 70 }), 
                    GeometryDefectType::Contraction(Bond { x: 70, y: 58 }), 
                ]
            ),
        ];
        for (step, threshold, initial_points, target) in test_data {
            let mut ctx = MocEval {
                ctx: Context::new(
                    InitialCtx::new(
                        Image::default()
                    )
                ),
            };
            ctx.ctx = ctx.ctx
                .clone()
                .write(initial_points.clone())
                .unwrap();
            let result = GeometryDefect::new(
                threshold,
                *Box::new(Mad::new()),
                WidthEmissions::new(threshold, 
                    *Box::new(Mad::new()), 
                    ctx
                ),
            ).eval(());
            match result {
                Ok(result) => {
                    let result = ContextRead::<GeometryDefectCtx>::read(&result)
                        .result.clone();
                    assert!(
                        result == target, 
                        "step {} \nresult: {:?}\ntarget: {:?}", 
                        step, 
                        result, 
                        target
                    );
                    if let Err(e) = visualize_with_opencv(step, &initial_points, &result) {
                        log::error!("Failed to visualize: {}", e);
                    }   
                },
                Err(err) => panic!("step {} \nerror: {:#?}", step, err),
            }
        }
        test_duration.exit();
    }
    ///
    ///
    #[derive(Debug, Clone)]
    struct MocEval {
        pub ctx: Context,
    }
    //
    //
    impl Eval<(), EvalResult> for MocEval {
        fn eval(&self, _: ()) -> EvalResult {
            Result::Ok(self.ctx.clone())
        }
    }
}

