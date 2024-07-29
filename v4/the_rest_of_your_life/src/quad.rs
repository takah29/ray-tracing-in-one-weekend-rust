use crate::{
    aabb::Aabb,
    hittable::{HitRecord, Hittable},
    hittable_list::HittableList,
    interval::Interval,
    material::Material,
    rtweekend::{random, Point3, Ray, Vec3, INFINITY},
    {point3, vec3},
};
use std::sync::Arc;

pub struct Quad {
    q: Point3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    mat_ptr: Arc<dyn Material>,
    bbox: Aabb,
    normal: Vec3,
    d: f64,
    area: f64,
}

impl Quad {
    pub fn new(q: Point3, u: Vec3, v: Vec3, mat_ptr: Arc<dyn Material>) -> Self {
        let n = u.cross(v);
        let normal = n.unit();
        let d = normal.dot(q);
        let w = n / n.length_squared();
        let area = n.length();

        // set bounding box
        let bbox_diagonal1 = Aabb::new_with_points(q, q + u + v);
        let bbox_diagonal2 = Aabb::new_with_points(q + u, q + v);
        let bbox = Aabb::from_boxes(bbox_diagonal1, bbox_diagonal2);

        Self {
            q,
            u,
            v,
            w,
            mat_ptr,
            bbox,
            normal,
            d,
            area,
        }
    }

    fn is_interior(a: f64, b: f64, rec: &mut HitRecord) -> bool {
        let unit_interval = Interval::new(0.0, 1.0);

        if !unit_interval.contains(a) || !unit_interval.contains(b) {
            return false;
        }

        rec.u = a;
        rec.v = b;

        true
    }
}

impl Hittable for Quad {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let denom = self.normal.dot(r.dir);

        if denom.abs() < 1e-8 {
            return false;
        }

        let t = (self.d - self.normal.dot(r.orig)) / denom;
        if !ray_t.contains(t) {
            return false;
        }

        let intersection = r.at(t);
        let planar_hitpt_vector = intersection - self.q;
        let alpha = self.w.dot(planar_hitpt_vector.cross(self.v));
        let beta = self.w.dot(self.u.cross(planar_hitpt_vector));

        if !Quad::is_interior(alpha, beta, rec) {
            return false;
        }

        rec.t = t;
        rec.p = intersection;
        rec.opt_mat_ptr = Some(self.mat_ptr.clone());
        rec.set_face_normal(r, &self.normal);

        return true;
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }

    fn pdf_value(&self, origin: &Point3, direction: &Vec3) -> f64 {
        let mut rec = HitRecord::default();

        if !self.hit(
            &Ray::new(*origin, *direction),
            Interval::new(0.001, INFINITY),
            &mut rec,
        ) {
            return 0.0;
        }

        let distance_squared = rec.t.powi(2) * direction.length_squared();
        let cosine = direction.dot(rec.normal).abs() / direction.length();

        distance_squared / (cosine * self.area)
    }

    fn random(&self, origin: &Vec3) -> Vec3 {
        let p = self.q + (random() * self.u) + (random() * self.v);

        p - *origin
    }
}

pub fn create_box(a: Point3, b: Point3, mat_ptr: Arc<dyn Material>) -> HittableList {
    let mut sides = HittableList::new();

    // Construct the two opposite vertices with the minimum and maximum coordinates.
    let min = point3!(a.e[0].min(b.e[0]), a.e[1].min(b.e[1]), a.e[2].min(b.e[2]));
    let max = point3!(a.e[0].max(b.e[0]), a.e[1].max(b.e[1]), a.e[2].max(b.e[2]));

    let dx = vec3!(max.e[0] - min.e[0], 0, 0);
    let dy = vec3!(0, max.e[1] - min.e[1], 0);
    let dz = vec3!(0, 0, max.e[2] - min.e[2]);

    sides.add(Arc::new(Quad::new(
        point3!(min.e[0], min.e[1], max.e[2]),
        dx,
        dy,
        mat_ptr.clone(),
    ))); // front
    sides.add(Arc::new(Quad::new(
        point3!(max.e[0], min.e[1], max.e[2]),
        -dz,
        dy,
        mat_ptr.clone(),
    ))); // right
    sides.add(Arc::new(Quad::new(
        point3!(max.e[0], min.e[1], min.e[2]),
        -dx,
        dy,
        mat_ptr.clone(),
    ))); // back
    sides.add(Arc::new(Quad::new(
        point3!(min.e[0], min.e[1], min.e[2]),
        dz,
        dy,
        mat_ptr.clone(),
    ))); // left
    sides.add(Arc::new(Quad::new(
        point3!(min.e[0], max.e[1], max.e[2]),
        dx,
        -dz,
        mat_ptr.clone(),
    ))); // top
    sides.add(Arc::new(Quad::new(
        point3!(min.e[0], min.e[1], min.e[2]),
        dx,
        dz,
        mat_ptr.clone(),
    ))); // bottom

    sides
}
