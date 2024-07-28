use crate::{
    aabb::{surrounding_box, AABB},
    hittable::{HitRecord, Hittable},
    hittable_list::HittableList,
    rtweekend::{random_int, Ray},
};
use std::cmp::Ordering;
use std::sync::Arc;

pub struct BvhNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bbox: AABB,
}

impl BvhNode {
    pub fn new(
        objects: &mut Vec<Arc<dyn Hittable>>,
        start: usize,
        end: usize,
        time0: f64,
        time1: f64,
    ) -> Self {
        let axis = random_int(0, 2);
        let comparator = match axis {
            0 => box_x_compare,
            1 => box_y_compare,
            2 => box_z_compare,
            _ => unreachable!(),
        };

        let object_span = end - start;

        let (left, right): (Arc<dyn Hittable>, Arc<dyn Hittable>) = match object_span {
            1 => (objects[start].clone(), objects[start].clone()),
            2 => {
                if comparator(&objects[start], &objects[start + 1]) == Ordering::Less {
                    (objects[start].clone(), objects[start + 1].clone())
                } else {
                    (objects[start + 1].clone(), objects[start].clone())
                }
            }
            _ => {
                objects[start..end].sort_by(comparator);
                let mid = start + object_span / 2;
                (
                    Arc::new(BvhNode::new(objects, start, mid, time0, time1)),
                    Arc::new(BvhNode::new(objects, mid, end, time0, time1)),
                )
            }
        };

        let mut box_left = AABB::new_with_empty();
        let mut box_right = AABB::new_with_empty();

        if !left.bounding_box(time0, time1, &mut box_left)
            || !right.bounding_box(time0, time1, &mut box_right)
        {
            eprintln!("No bounding box in bvh_node constructor.");
        }

        let bbox = surrounding_box(box_left, box_right);

        Self { left, right, bbox }
    }

    pub fn new_with_list(list: &mut HittableList, time0: f64, time1: f64) -> Self {
        let length = list.objects.len();
        Self::new(&mut list.objects, 0, length, time0, time1)
    }
}

impl Hittable for BvhNode {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        if !self.bbox.hit(r, t_min, t_max) {
            return false;
        }

        let hit_left = self.left.hit(r, t_min, t_max, rec);
        let hit_right = self
            .right
            .hit(r, t_min, if hit_left { rec.t } else { t_max }, rec);

        hit_left || hit_right
    }

    fn bounding_box(&self, _: f64, _: f64, output_box: &mut AABB) -> bool {
        *output_box = self.bbox.clone();
        true
    }
}

fn box_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>, axis: usize) -> Ordering {
    let mut box_a = AABB::new_with_empty();
    let mut box_b = AABB::new_with_empty();

    if !a.bounding_box(0.0, 0.0, &mut box_a) || !b.bounding_box(0.0, 0.0, &mut box_b) {
        eprintln!("No bounding box in bvh_node constructor.");
    }

    box_a.min.e[axis]
        .partial_cmp(&box_b.min.e[axis])
        .unwrap_or(Ordering::Equal)
}

fn box_x_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
    box_compare(a, b, 0)
}

fn box_y_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
    box_compare(a, b, 1)
}

fn box_z_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
    box_compare(a, b, 2)
}
