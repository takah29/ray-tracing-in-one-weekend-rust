use crate::{
    aabb::AABB,
    aarect::{XyRect, XzRect, YzRect},
    hittable::{HitRecord, Hittable},
    hittable_list::HittableList,
    interval::Interval,
    material::Material,
    rtweekend::{Point3, Ray},
};
use std::sync::Arc;

pub struct Cuboid {
    box_min: Point3,
    box_max: Point3,
    sides: HittableList,
}

impl Cuboid {
    pub fn new(box_min: Point3, box_max: Point3, mat_ptr: Arc<dyn Material>) -> Self {
        let mut sides = HittableList::new();

        let Point3 { e: [p0x, p0y, p0z] } = box_min;
        let Point3 { e: [p1x, p1y, p1z] } = box_max;

        sides.add(Arc::new(XyRect::new(
            p0x,
            p1x,
            p0y,
            p1y,
            p1z,
            mat_ptr.clone(),
        )));
        sides.add(Arc::new(XyRect::new(
            p0x,
            p1x,
            p0y,
            p1y,
            p0z,
            mat_ptr.clone(),
        )));

        sides.add(Arc::new(XzRect::new(
            p0x,
            p1x,
            p0z,
            p1z,
            p1y,
            mat_ptr.clone(),
        )));
        sides.add(Arc::new(XzRect::new(
            p0x,
            p1x,
            p0z,
            p1z,
            p0y,
            mat_ptr.clone(),
        )));

        sides.add(Arc::new(YzRect::new(
            p0y,
            p1y,
            p0z,
            p1z,
            p1x,
            mat_ptr.clone(),
        )));
        sides.add(Arc::new(YzRect::new(
            p0y,
            p1y,
            p0z,
            p1z,
            p0x,
            mat_ptr.clone(),
        )));

        Self {
            box_min,
            box_max,
            sides,
        }
    }
}

impl Hittable for Cuboid {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        self.sides.hit(r, ray_t, rec)
    }

    fn bounding_box(&self, _: f64, _: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB::new(self.box_min, self.box_max);
        true
    }
}
