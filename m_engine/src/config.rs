
#[cfg(feature = "single-precision")]
pub type Real = f64;

#[cfg(not(feature = "single-precision"))]
pub type Real = f32;
