use std::borrow::Cow;

use deno_core::{
    GarbageCollected, Resource, op2,
    serde::{Deserialize, Serialize},
};
#[derive(Serialize, Deserialize)]
pub struct Vec2D {
    pub x: f64,
    pub y: f64,
}

impl Resource for Vec2D {
    fn name(&self) -> Cow<str> {
        Cow::Borrowed("Vec2D")
    }
}

impl GarbageCollected for Vec2D {}

#[op2(fast)]
impl Vec2D {
    #[constructor]
    #[cppgc]
    pub fn new(x: f64, y: f64) -> Vec2D {
        Vec2D { x, y }
    }

    #[fast]
    #[static_method]
    pub fn get_x(#[cppgc] obj: &Vec2D) -> f64 {
        obj.x
    }
}

impl Vec2D {
    pub fn new_raw(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}
