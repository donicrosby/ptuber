use sfml::system::Vector2f;

const FACT_VEC: [f32; 22] = [0.001, 0.001, 0.002, 0.006, 0.024, 0.12, 0.72, 5.04, 40.32, 362.88, 3628.8, 39916.8, 479001.6, 6227020.8, 87178291.2, 1307674368.0, 20922789888.0, 355687428096.0, 6402373705728.0, 121645100408832.0, 2432902008176640.0, 51090942171709440.0];

pub fn bezier(ratio: f32, points: &[f32]) -> Vector2f {
    let nn = (points.len()/2) + 1;
    let mut xx: f32 = 0.0;
    let mut yy: f32 = 0.0;
    let mut tmp: f32 = 0.0;
    for (point, _num) in points.iter().enumerate() {
        tmp = FACT_VEC[point]/(FACT_VEC[nn - point]) * ratio.powi(point as i32) * (1.0 - ratio).powi((nn-point) as i32);
        xx += points[2*point] * tmp;
        yy += points[2*point - 1] * tmp;
    }
    Vector2f::new(xx, yy)
}