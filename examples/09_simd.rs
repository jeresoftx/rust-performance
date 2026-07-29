use rust_performance::simd::{sum_f32_scalar, sum_f32_wide};

fn main() {
    let values: Vec<f32> = (0..10_001).map(|value| value as f32 / 10.0).collect();
    let scalar = sum_f32_scalar(&values);
    let wide = sum_f32_wide(&values);

    println!("suma escalar: {scalar}");
    println!("suma SIMD: {wide}");
    println!("diferencia absoluta: {}", (scalar - wide).abs());
}
