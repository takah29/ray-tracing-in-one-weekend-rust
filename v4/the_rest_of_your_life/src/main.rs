use std::sync::Arc;
use the_rest_of_your_life::{bvh::BvhNode, hittable::Hittable};

#[allow(unused_imports)]
use the_rest_of_your_life::build_scene::{cornell_box, minimul_scene};

fn main() {
    let (mut hittable_list, lights, cam, direct_light_sampling) = minimul_scene();

    // let world: Box<dyn Hittable> = Box::new(hittable_list);
    let world: Box<dyn Hittable> = Box::new(BvhNode::new_with_list(&mut hittable_list, 0.0, 1.0));
    let lights: Arc<dyn Hittable> = Arc::new(lights);

    cam.render(&world, &lights, direct_light_sampling);
}
