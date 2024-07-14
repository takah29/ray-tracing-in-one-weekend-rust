use crate::{
    aabb::{surrounding_box, AABB},
    hittable::{HitRecord, Hittable},
    hittable_list::HittableList,
    rtweekend::{random_int, Ray},
};

pub struct BvhNode {
    left: Box<dyn Hittable>,
    right: Box<dyn Hittable>,
    bbox: AABB,
}

impl BvhNode {
    pub fn new(
        objects: &Vec<Box<dyn Hittable>>,
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

        let (left, right) = match object_span {
            1 => (objects[start], objects[start]),
            2 => {
                if comparator(objects[start], objects[start + 1]) {
                    (objects[start], objects[start + 1])
                } else {
                    (objects[start + 1], objects[start])
                }
            }
            _ => {
                objects[start..end].sort_by(comparator);
                let mid = start + object_span / 2;
                (
                    BvhNode::new(objects, start, mid, time0, time1),
                    BvhNode::new(objects, mid, end, time0, time1),
                )
            }
        };

        let box_left = AABB::new_with_inf();
        let box_right = AABB::new_with_inf();

        if !left.bounding_box(time0, time1, &mut box_left)
            || !right.bounding_box(time0, time1, &mut box_right)
        {
            eprintln!("No bounding box in bvh_node constructor.");
        }

        let bbox = surrounding_box(box_left, box_right);

        Self { left, right, bbox }
    }

    pub fn new_with_list(list: HittableList, time0: f64, time1: f64) -> Self {
        Self::new(&list.objects, 0, list.objects.len(), time0, time1)
    }
}

impl Hittable for BvhNode {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        if self.bbox.hit(r, t_min, t_max) {
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
