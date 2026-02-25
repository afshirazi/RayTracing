pub struct TrowbridgeReitzDistribution {
    alpha_x: f32,
    alpha_y: f32,
}

impl TrowbridgeReitzDistribution {
    pub fn effectively_smooth(&self) -> bool {
        f32::max(self.alpha_x, self.alpha_y) < 1e-3
    }
}
