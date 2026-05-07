use opencv::{core::Mat, imgproc};
use sal_core::error::Error;
use crate::domain::Eval;
///
/// ### Canny Edge Detector filter
/// - Finds edges in an image using the Canny86 algorithm.
/// 
/// Алгоритм Кенни работает с градиентами (перепадами яркости пикселей).
/// Для стандартных 8-битных изображений (где цвет пикселя от 0 до 255)
/// значения порогов обычно лежат в диапазоне **от 0 до 255**, иногда чуть выше,
/// если градиент считается жестко, но базовый ориентир — шкала до 255.
/// 
/// * **`canny_thresh2` (Верхний порог — Жесткая граница):** Все пиксели, где перепад яркости выше этого значения, алгоритм безапелляционно считает 100% границей контура.
/// * **`canny_thresh1` (Нижний порог — Зона отсечения):** Все пиксели с перепадом ниже этого значения считаются шумом и удаляются.
/// * **Гистерезис (Магия алгоритма):** Пиксели, которые попали в «серую зону» (между `thresh1` и `thresh2`), считаются слабыми границами. Кенни оставит их только в том случае, если они физически соприкасаются с пикселями жесткой границы. Это позволяет не разрывать контур каната, даже если на каком-то участке свет упал неудачно.
/// 
/// **Как настраивать:**
/// Классическое инженерное правило — соотношение порогов **1:2** или **1:3**.
/// 1. Начни с `canny_thresh1 = 50` и `canny_thresh2 = 150`.
/// 2. Если контур каната рвется на куски (слишком строгий отбор) — снижай верхний порог (например, до 100).
/// 3. Если вокруг каната остается много мелкой "пыли" (матричный шум) — поднимай нижний порог (например, до 80).
pub struct Canny {
    threshold1: f64,
    threshold2: f64,
    ctx: Box<dyn Eval<Mat, Result<Mat, Error>> + Send + Sync>,
}
//
//
impl Canny {
    ///
    /// Returns [Canny] new instance
    /// - `threshold1` - First threshold for the hysteresis procedure.
    /// - `threshold2` - Second threshold for the hysteresis procedure.
    /// - `ctx` - Previous step returning [Mat].
    pub fn new(
        threshold1: f64,
        threshold2: f64,
        ctx: impl Eval<Mat, Result<Mat, Error>> + Send + Sync + 'static,
    ) -> Self {
        Self {
            threshold1,
            threshold2,
            ctx: Box::new(ctx),
        }
    }
}
//
//
impl Eval<Mat, Result<Mat, Error>> for Canny {
    fn eval(&self, mat: Mat) -> Result<Mat, Error> {
        let error = Error::new("Canny", "eval");
        let src = self.ctx.eval(mat).map_err(|err| error.clone().pass(err.to_string()))?;
        let mut dst = Mat::default();
        imgproc::canny(&src, &mut dst, self.threshold1, self.threshold2, 3, false)
            .map_err(|err| error.pass(err.to_string()))?;
        Ok(dst)
    }
}
