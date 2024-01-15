use crate::world::World;

pub trait Builder {
    fn build(&self, world: &mut World);
}
