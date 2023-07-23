
pub const MAIN_COLOR: [f32; 4] = [0.3, 0.0, 1.0, 1.0];

pub fn hsl_to_rgb(h: f32, s: f32, l: f32) -> [f32; 4] {
    let a = s * l.min(1.0 - l);
    let f = |n: f32| {
        let k = (n + h / 30.0) % 12.0;
        l - a * (k.min(9.0 - k.min(3.0)).max(-1.0))
    };
    [f(0.0), f(8.0), f(4.0), 1.0]
}

pub fn darken_color_bg(color: [f32; 4]) -> [f32; 4] {
    [
        color[0] * 0.1,
        color[1] * 0.1,
        color[2] * 0.1,
        color[3] * 0.5,
    ]
}