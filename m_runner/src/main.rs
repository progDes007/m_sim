use m_engine::vec2;
use m_engine::generators;
fn main() {
    let _v = vec2::Vec2::new(0.0, 0.0);
    generators::generate_grid(
        vec2::Vec2::new(0.0, 0.0),
        vec2::Vec2::new(1.0, 0.0),
        1.0,
        1.0,
        1,
        1,
        generators::constant_velocity(vec2::Vec2::new(1.0, 2.0)),
        2,
    );
    
}
