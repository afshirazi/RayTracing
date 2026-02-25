#[derive(Clone)]
pub struct TrowbridgeReitzDistribution {
    alpha_x: f32,
    alpha_y: f32,
}

impl TrowbridgeReitzDistribution {
    pub fn zero() -> Self {
        Self {
            alpha_x: 0.0,
            alpha_y: 0.0,
        }
    }
    pub fn effectively_smooth(&self) -> bool {
        f32::max(self.alpha_x, self.alpha_y) < 1e-3
    }
}
