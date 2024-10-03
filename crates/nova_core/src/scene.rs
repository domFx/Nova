use crate::game_object::GameObject;

#[allow(unused)]
#[derive(Default)]
pub struct Scene {
    game_objects: Vec<GameObject>,
}

#[allow(unused)]
trait SceneImpl {
    fn new() {}
}
