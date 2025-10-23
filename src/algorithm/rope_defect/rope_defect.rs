use std::{any::TypeId, marker::PhantomData};
use sal_core::dbg::Dbg;
use crate::{
    algorithm::{
        RopeDefectCtx, RopeDefectKind, Threshold,
        mad::MadCtx, RopeDistortions, RopeDistortionsCtx, ContextRead, ContextWrite,
        FastEdgesCtx, FineEdgesCtx, EvalResult, Side,
        FastScanCtx, FineScanCtx,
    }, 
    domain::{Dot, Error, Eval, Image},
};

///
/// Represents detecting [rope geometry defect's](design/theory/geometry_rope_defects.md)
/// 
/// Classification of the rope defects on the bands already found in the RopeDistortions 
pub struct RopeDefect<Branch> {
    dbg: Dbg,
    threshold: Threshold,
    mad: Box<dyn Eval<Vec<usize>, Result<MadCtx, Error>> + Send + Sync>,
    ctx: RopeDistortions<Branch>,
    branch: PhantomData<Branch>,
}
//
//
impl<Branch> RopeDefect<Branch> {
    ///
    /// New instance [RopeDefect]
    /// 
    /// - `Branch` - the calculation branch [FastScanCtx] or [FineScanCtx]
    pub fn new(
        threshold: Threshold,
        mad: impl Eval<Vec<usize>, Result<MadCtx, Error>> + Send + Sync + 'static,
        ctx: RopeDistortions<Branch>,
    ) -> Self {
        Self {
            dbg: Dbg::own("RopeDefect"),
            threshold,
            mad: Box::new(mad),
            ctx,
            branch: PhantomData,
        }
    }
    ///
    /// Detecting [both sides width growing](design/references/GOST_33718-2015.pdf)
    fn expansion(&self, upper_point: &Dot<usize>, lower_point: &Dot<usize>, upper_mad: &MadCtx, lower_mad: &MadCtx) -> Option<()> {
        let deviation_upper = upper_point.y as f64 - upper_mad.median;
        let deviation_lower = lower_point.y as f64 - lower_mad.median;
        if (deviation_upper < self.threshold.0 * upper_mad.mad) &&
            (deviation_lower > self.threshold.0 * lower_mad.mad) {
            return Some(());
        }
        None
    }
    ///
    /// Detecting [both sides width reduction](design/references/GOST_33718-2015.pdf)
    fn compressing(&self, upper_point: &Dot<usize>, lower_point: &Dot<usize>, upper_mad: &MadCtx, lower_mad: &MadCtx) -> Option<()> {
        let deviation_upper = upper_point.y as f64 - upper_mad.median;
        let deviation_lower = lower_point.y as f64 - lower_mad.median;
        if (deviation_upper > self.threshold.0 * upper_mad.mad) &&
            (deviation_lower < self.threshold.0 * lower_mad.mad) {
            return Some(());
        }
        None
    }
    ///
    /// Detecting [one side drooping](design/references/GOST_33718-2015.pdf)
    fn pit(&self, upper_point: &Dot<usize>, lower_point: &Dot<usize>, upper_mad: &MadCtx, lower_mad: &MadCtx) -> Option<()> {
        let deviation_upper = upper_point.y as f64 - upper_mad.median;
        let deviation_lower = lower_point.y as f64 - lower_mad.median;
        if (deviation_upper.abs() < self.threshold.0 * upper_mad.mad) &&
        (deviation_lower > self.threshold.0 * lower_mad.mad) {
            return Some(());
        } else if (deviation_upper > self.threshold.0 * upper_mad.mad) &&
            (deviation_lower.abs() < self.threshold.0 * lower_mad.mad) {
            return Some(());
        }
        None
    }
    ///
    /// Detecting [one side raising](design/references/GOST_33718-2015.pdf)
    fn hill(&self, upper_point: &Dot<usize>, lower_point: &Dot<usize>, upper_mad: &MadCtx, lower_mad: &MadCtx) -> Option<()> {
        let deviation_upper = upper_point.y as f64 - upper_mad.median;
        let deviation_lower = lower_point.y as f64 - lower_mad.median;
        if (deviation_upper.abs() < self.threshold.0 * upper_mad.mad) &&
            (deviation_lower < -self.threshold.0 * lower_mad.mad) {
            return Some(());
        } else if (deviation_upper < -self.threshold.0 * upper_mad.mad) &&
            (deviation_lower.abs() < self.threshold.0 * lower_mad.mad) {
            return Some(());
        }
        None
    }   
}
//
//
impl<Branch: 'static> Eval<Image, EvalResult> for RopeDefect<Branch> {
    fn eval(&self, frame: Image) -> EvalResult {
        let error = Error::new(&self.dbg, "eval");
        match self.ctx.eval(frame) {
            Ok(ctx) => {
                // let mut result = Vec::new();
                // let width_emissions_result = ContextRead::<WidthEmissionsCtx>::read(&ctx).result.clone();
                let rope_distortions = match TypeId::of::<Branch>() {
                    typ if typ == TypeId::of::<FastScanCtx>() => &ContextRead::<RopeDistortionsCtx<FastScanCtx>>::read(&ctx).result,
                    typ if typ == TypeId::of::<FineScanCtx>() => &ContextRead::<RopeDistortionsCtx<FineScanCtx>>::read(&ctx).result,
                    _ => Err(error.err(format!("Can't read result from: '{:?}' branch of 'Context'", TypeId::of::<Branch>())))?,
                };
                if rope_distortions.is_empty() {
                    log::debug!("Frame without defect's");
                    return match TypeId::of::<Branch>() {
                        // typ if typ == TypeId::of::<FastScanCtx>() => ctx.write(RopeDefectCtx::<FastScanCtx>::new(vec![])),
                        typ if typ == TypeId::of::<FineScanCtx>() => ctx.write(RopeDefectCtx::<FineScanCtx>::new(vec![])),
                        _ => Err(error.err(format!("Can't read result from: '{:?}' branch of 'Context'", TypeId::of::<Branch>()))),
                    }
                }
                let (upper, lower) = match TypeId::of::<Branch>() {
                    typ if typ == TypeId::of::<FastScanCtx>() => {
                        let edges_ctx: &FastEdgesCtx = ctx.read();
                        (edges_ctx.edges.get(Side::Upper), edges_ctx.edges.get(Side::Lower))
                    }
                    typ if typ == TypeId::of::<FineScanCtx>() => {
                        let edges_ctx: &FineEdgesCtx = ctx.read();
                        (edges_ctx.edges.get(Side::Upper), edges_ctx.edges.get(Side::Lower))
                    }
                    _ => Err(error.err(format!("Can't read result from: '{:?}' branch of 'Context'", TypeId::of::<Branch>())))?,
                };
                let upper_mad = self.mad.eval(
                    upper.iter().map(|dot| dot.y).collect(),
                ).map_err(|err| error.pass(err))?;
                let lower_mad = self.mad.eval(
                    lower.iter().map(|dot| dot.y).collect()
                ).map_err(|err| error.pass(err))?;
                let mut result = Defects::new();
                let mut x = 0;
                for bend in rope_distortions {
                    for (upper_point, lower_point) in bend.upper.iter().zip(&bend.lower) {
                        match self.expansion(&upper_point, &lower_point, &upper_mad, &lower_mad) {
                            Some(_) => result.push(RopeDefectKind::Expansion(upper_point.x, upper_point.x)),
                            None => match self.compressing(&upper_point, &lower_point, &upper_mad, &lower_mad) {
                                Some(_) => result.push(RopeDefectKind::Compressing(upper_point.x, upper_point.x)),
                                None => match self.hill(&upper_point, &lower_point, &upper_mad, &lower_mad) {                   // Холмик
                                    Some(_) => result.push(RopeDefectKind::Hill(upper_point.x, upper_point.x)),
                                    None => match self.pit(&upper_point, &lower_point, &upper_mad, &lower_mad) {                // Ямка
                                        Some(_) => result.push(RopeDefectKind::Pit(upper_point.x, upper_point.x)),
                                        None => result.no_defect(upper_point.x),
                                    }
                                }
                            }
                        }
                        x = upper_point.x;
                    }
                    result.no_defect(x);
                }
                match TypeId::of::<Branch>() {
                    // typ if typ == TypeId::of::<FastScanCtx>() => ctx.write(RopeDefectCtx::<FastScanCtx>::new(result.all())),
                    typ if typ == TypeId::of::<FineScanCtx>() => ctx.write(RopeDefectCtx::<FineScanCtx>::new(result.all())),
                    _ => Err(error.err(format!("Can't read result from: '{:?}' branch of 'Context'", TypeId::of::<Branch>()))),
                }
            },
            Err(err) => Err(error.pass(err)),
        }
    }
}
///
/// 
struct Defects {
    items: Vec<RopeDefectKind>,
    prev: Option<RopeDefectKind>,

}
impl Defects {
    pub fn new() -> Self {
        Self { items: vec![], prev: None }
    }
    pub fn no_defect(&mut self, x: usize) {
        if let Some(prev) = &self.prev {
            let detected = prev.end_with(x);
            if detected.end() - detected.start() > 16 {
                self.items.push(detected);
            }
            self.prev = None;
        }
    }
    pub fn push(&mut self, defect: RopeDefectKind) {
        match &self.prev {
            Some(prev) => {
                if !defect.is_same(prev) {
                    if prev.is_same(&RopeDefectKind::Hill(0, 0)) && defect.is_same(&RopeDefectKind::Expansion(0, 0)) {
                        self.prev = Some(RopeDefectKind::Expansion(prev.start(), defect.end()));
                        return;
                    }
                    if prev.is_same(&RopeDefectKind::Expansion(0, 0)) && (defect.is_same(&RopeDefectKind::Pit(0, 0)) || defect.is_same(&RopeDefectKind::Hill(0, 0))) {
                        return;
                    }
                    if prev.is_same(&RopeDefectKind::Pit(0, 0)) && defect.is_same(&RopeDefectKind::Compressing(0, 0)) {
                        self.prev = Some(RopeDefectKind::Compressing(prev.start(), defect.end()));
                        return;
                    }
                    if prev.is_same(&RopeDefectKind::Compressing(0, 0)) && (defect.is_same(&RopeDefectKind::Pit(0, 0)) || defect.is_same(&RopeDefectKind::Hill(0, 0))) {
                        return;
                    }
                    let detected = prev.end_with(defect.start());
                    if detected.end() - detected.start() > 16 {
                        self.items.push(detected);
                    }
                    self.prev = Some(defect);
                }
            }
            None => {
                self.prev = Some(defect);
            }
        }
    }
    pub fn all(self) -> Vec<RopeDefectKind> {
        self.items
    }
}