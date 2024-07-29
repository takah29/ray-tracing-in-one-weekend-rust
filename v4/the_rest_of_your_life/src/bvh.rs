use crate::{
    aabb::Aabb,
    hittable::{HitRecord, Hittable},
    hittable_list::HittableList,
    interval::Interval,
    rtweekend::Ray,
};
use std::cmp::Ordering;
use std::sync::Arc;

pub struct BvhNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bbox: Aabb,
}

impl BvhNode {
    pub fn new(
        objects: &mut Vec<Arc<dyn Hittable>>,
        start: usize,
        end: usize,
        time0: f64,
        time1: f64,
    ) -> Self {
        let mut bbox = Aabb::new_with_empty();
        for object_index in start..end {
            bbox = Aabb::from_boxes(bbox, objects[object_index].bounding_box());
        }

        let axis = bbox.longest_axis();

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

        Self { left, right, bbox }
    }

    pub fn new_with_list(list: &mut HittableList, time0: f64, time1: f64) -> Self {
        let length = list.objects.len();
        Self::new(&mut list.objects, 0, length, time0, time1)
    }
}

impl Hittable for BvhNode {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        if !self.bbox.hit(r, ray_t) {
            return false;
        }

        let hit_left = self.left.hit(r, ray_t, rec);
        let hit_right = self.right.hit(
            r,
            Interval::new(ray_t.min, if hit_left { rec.t } else { ray_t.max }),
            rec,
        );

        hit_left || hit_right
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}

fn box_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>, axis_index: usize) -> Ordering {
    let a_axis_interval = a.bounding_box().axis_interval(axis_index);
    let b_axis_interval = b.bounding_box().axis_interval(axis_index);

    a_axis_interval
        .min
        .partial_cmp(&b_axis_interval.min)
        .expect("Values are not comparable")
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
