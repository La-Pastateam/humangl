use crate::model::Skeleton;

pub trait Animate {
    // dt : temps écoulé  depuis la derniere frame
    fn update(&mut self, dt: f32);

    fn skeleton(&self) -> &Skeleton;
}
