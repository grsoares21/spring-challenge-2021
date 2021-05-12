pub fn ease_in_out_sine(x: f32) -> f32 {
  -((std::f32::consts::PI * x).cos() - 1.0) / 2.0
}

pub fn ease_in_out_circ(x: f32) -> f32 {
  if x < 0.5 {
    (1.0 - (1.0 - (x * 2.0).powf(2.0)).sqrt()) / 2.0
  } else {
    ((1.0 - (-2.0 * x + 2.0).powf(2.0)).sqrt() + 1.0) / 2.0
  }
}

pub fn ease_out_circ(x: f32) -> f32 {
  (1.0 - (x - 1.0).powf(2.0)).sqrt()
}

//-(cos(PI * x) - 1) / 2;
