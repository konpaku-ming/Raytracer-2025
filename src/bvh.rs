use crate::aabb::Aabb;
use crate::hit_checker::{HitRecord, Hittable, HittableList};
use crate::interval::Interval;
use crate::ray::Ray;
use std::cmp::Ordering;
use std::sync::Arc;

pub struct BvhNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bbox: Aabb,
}

impl BvhNode {
    pub fn from_list(list: &mut HittableList) -> Self {
        let len = list.objects.len();
        BvhNode::from_range(&mut list.objects, 0, len)
    }

    pub fn from_range(objects: &mut Vec<Arc<dyn Hittable>>, start: usize, end: usize) -> Self {
        let left: Arc<dyn Hittable>;
        let right: Arc<dyn Hittable>;

        let mut bbox = Aabb::EMPTY;

        for obj in objects.iter().take(end).skip(start) {
            bbox = Aabb::from_box(bbox, obj.bounding_box());
        }

        let axis = bbox.longest_axis();

        let comparator = BvhNode::box_compare(axis);

        let object_span = end - start;
        match object_span {
            1 => {
                left = objects[start].clone();
                right = objects[start].clone();
            }
            2 => {
                left = objects[start].clone();
                right = objects[start + 1].clone();
            }
            _ => {
                objects[start..end].sort_by(comparator);
                let mid = start + object_span / 2;
                left = Arc::new(BvhNode::from_range(objects, start, mid));
                right = Arc::new(BvhNode::from_range(objects, mid, end));
            }
        }

        Self { left, right, bbox }
    }

    fn box_compare(
        axis_index: usize,
    ) -> impl Fn(&Arc<dyn Hittable>, &Arc<dyn Hittable>) -> Ordering {
        move |a, b| {
            let box1 = a.bounding_box();
            let box2 = b.bounding_box();
            let a_interval = box1.axis_interval(axis_index);
            let b_interval = box2.axis_interval(axis_index);
            a_interval
                .min
                .partial_cmp(&b_interval.min)
                .unwrap_or(Ordering::Equal)
        }
    }
}

impl Hittable for BvhNode {
    fn hit(&self, r: &Ray, mut ray_t: Interval, rec: &mut HitRecord) -> bool {
        if !self.bbox.hit(r, &mut ray_t) {
            return false;
        }
        let hit_left = self.left.hit(r, ray_t, rec);
        let new_max = if hit_left { rec.t } else { ray_t.max };
        let right_t = Interval::new(ray_t.min, new_max);
        let hit_right = self.right.hit(r, right_t, rec);
        hit_left || hit_right
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
