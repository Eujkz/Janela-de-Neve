//! Snowhush ModKit options. Platform-independent and independent of Windows APIs.
//! Values are explicit indices so the Windows tray can persist them as REG_DWORD.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ModProfile {
    pub season: u32,   // 0 snow, 1 rain, 2 autumn, 3 spring
    pub storm: bool,
    pub density: u32,  // 0..4
    pub speed: u32,    // 0..3
    pub wind: u32,     // 0..3
    pub size: u32,     // 0..3
    pub cap: u32,      // 0..3
    pub cursor: u32,   // 0..3
    pub color: u32,    // 0..4
}
impl Default for ModProfile {
    fn default() -> Self {
        Self { season: 0, storm: false, density: 2, speed: 1, wind: 1, size: 1, cap: 2, cursor: 1, color: 0 }
    }
}
impl ModProfile {
    pub fn density_multiplier(self) -> f32 { [0.25, 0.5, 1.0, 1.5, 2.0].get(self.density as usize).copied().unwrap_or(1.0) }
    pub fn speed_multiplier(self) -> f32 { [0.5, 1.0, 1.5, 2.0].get(self.speed as usize).copied().unwrap_or(1.0) }
    pub fn wind_multiplier(self) -> f32 { [0.0, 1.0, 2.5, 5.0].get(self.wind as usize).copied().unwrap_or(1.0) }
    pub fn size_multiplier(self) -> f32 { [0.5, 1.0, 1.5, 2.0].get(self.size as usize).copied().unwrap_or(1.0) }
    pub fn cap_height(self) -> f32 { [8.0, 16.0, 32.0, 64.0].get(self.cap as usize).copied().unwrap_or(32.0) }
    pub fn cursor_radius(self) -> f32 { [6.0, 10.0, 18.0, 30.0].get(self.cursor as usize).copied().unwrap_or(10.0) }
    pub fn color_rgb(self) -> [f32;3] {
        match self.color {
            1 => [0.42, 0.72, 1.0],
            2 => [1.0, 0.55, 0.78],
            3 => [1.0, 0.20, 0.26],
            4 => [1.0, 0.78, 0.35],
            _ => match self.season {
                1 => [0.50, 0.72, 1.0],
                2 => [0.95, 0.53, 0.16],
                3 => [1.0, 0.65, 0.80],
                _ => [1.0, 1.0, 1.0],
            },
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn defaults_are_normal() {
        let p = ModProfile::default();
        assert_eq!(p.density_multiplier(), 1.0);
        assert_eq!(p.speed_multiplier(), 1.0);
        assert_eq!(p.cursor_radius(), 10.0);
    }
    #[test] fn invalid_registry_indices_are_safe() {
        let mut p = ModProfile::default();
        p.density = u32::MAX;
        p.color = u32::MAX;
        p.cap = u32::MAX;
        assert_eq!(p.density_multiplier(), 1.0);
        assert_eq!(p.cap_height(), 32.0);
    }
}
