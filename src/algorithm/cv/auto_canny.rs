use opencv::{core, imgproc};
use sal_core::error::Error;
use crate::domain::Eval;
///
/// Adaptive Canny Edge Detector
/// Dynamically calculates thresholds using Otsu's method to handle varying lighting
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

pub struct AutoCanny {
    tune: f64,
    ctx: Box<dyn Eval<core::Mat, Result<core::Mat, Error>> + Send + Sync>,
}
//
//
impl AutoCanny {
    ///
    /// Returns [AutoCanny] new instance
    /// - `tune` - Tuning factor for Otsu threshold (e.g., 0.4 - 1.0)
    /// - `ctx` - Previous step returning [Mat]
    pub fn new(
        tune: f64,
        ctx: impl Eval<core::Mat, Result<core::Mat, Error>> + Send + Sync + 'static,
    ) -> Self {
        Self {
            tune,
            ctx: Box::new(ctx),
        }
    }
}
//
//
impl Eval<core::Mat, Result<core::Mat, Error>> for AutoCanny {
    fn eval(&self, mat: core::Mat) -> Result<core::Mat, Error> {
        let error = Error::new("AutoCanny", "eval");
        let src = self.ctx.eval(mat).map_err(|err| error.clone().pass(err.to_string()))?;
        // Холостой проход для получения идеального порога текущего кадра
        let otsu_thresh = imgproc::threshold(
            &src,
            &mut core::Mat::default(),
            0.0,
            255.0,
            imgproc::THRESH_OTSU, // imgproc::THRESH_BINARY | imgproc::THRESH_OTSU
        ).map_err(|err| error.pass(err.to_string()))?;
        // Рассчитываем динамические пороги: верхний (с тюнингом) и нижний (50% от верхнего)
        let high_thresh = otsu_thresh * self.tune;
        let low_thresh = high_thresh / 3.0; // * 0.5;
        let mut dst = core::Mat::default();
        imgproc::canny(&src, &mut dst, low_thresh, high_thresh, 3, false)
            .map_err(|err| error.pass(err.to_string()))?;
        Ok(dst)
    }
}
