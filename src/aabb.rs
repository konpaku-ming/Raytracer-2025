use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

#[derive(Clone, Copy, Debug)]
pub struct Aabb {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl Default for Aabb {
    fn default() -> Self {
        Self {
            x: Interval::EMPTY,
            y: Interval::EMPTY,
            z: Interval::EMPTY,
        }
    }
}

impl Aabb {
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        let mut temp = Aabb { x, y, z };
        temp.pad_to_minimums();
        temp
    }

    //两个对角点构造
    pub fn from_points(a: Point3, b: Point3) -> Self {
        let (x_min, x_max) = if a[0] <= b[0] {
            (a[0], b[0])
        } else {
            (b[0], a[0])
        };
        let (y_min, y_max) = if a[1] <= b[1] {
            (a[1], b[1])
        } else {
            (b[1], a[1])
        };
        let (z_min, z_max) = if a[2] <= b[2] {
            (a[2], b[2])
        } else {
            (b[2], a[2])
        };
        let mut temp = Aabb {
            x: Interval::new(x_min, x_max),
            y: Interval::new(y_min, y_max),
            z: Interval::new(z_min, z_max),
        };
        temp.pad_to_minimums();
        temp
    }

    pub fn from_box(box1: Aabb, box2: Aabb) -> Self {
        Self {
            x: Interval::union(box1.x, box2.x),
            y: Interval::union(box1.y, box2.y),
            z: Interval::union(box1.z, box2.z),
        }
    }

    pub fn axis_interval(&self, axis: usize) -> &Interval {
        match axis {
            1 => &self.y,
            2 => &self.z,
            _ => &self.x,
        }
    }

    //检测光线与盒子里是否有交
    pub fn hit(&self, r: &Ray, ray_t: &mut Interval) -> bool {
        let orig = r.origin();
        let dir = r.direction();

        for axis in 0..3 {
            let ax = self.axis_interval(axis);
            let inv_d = 1.0 / dir[axis];
            let t0 = (ax.min - orig[axis]) * inv_d;
            let t1 = (ax.max - orig[axis]) * inv_d;

            if t0 < t1 {
                if t0 > ray_t.min {
                    ray_t.min = t0;
                }
                if t1 < ray_t.max {
                    ray_t.max = t1;
                }
            } else {
                if t1 > ray_t.min {
                    ray_t.min = t1;
                }
                if t0 < ray_t.max {
                    ray_t.max = t0;
                }
            }

            if ray_t.max <= ray_t.min {
                return false;
            }
        }
        true
    }

    pub fn longest_axis(&self) -> usize {
        if self.x.size() > self.y.size() {
            if self.x.size() > self.z.size() { 0 } else { 2 }
        } else if self.y.size() > self.z.size() {
            1
        } else {
            2
        }
    }

    fn pad_to_minimums(&mut self) {
        let delta = 0.0001;
        if self.x.size() < delta {
            self.x = self.x.expand(delta);
        }
        if self.y.size() < delta {
            self.y = self.y.expand(delta);
        }
        if self.z.size() < delta {
            self.z = self.z.expand(delta);
        }
    }

    pub const EMPTY: Self = Self {
        x: Interval::EMPTY,
        y: Interval::EMPTY,
        z: Interval::EMPTY,
    };

    pub const UNIVERSE: Self = Self {
        x: Interval::UNIVERSE,
        y: Interval::UNIVERSE,
        z: Interval::UNIVERSE,
    };
}

impl std::ops::Add<Vec3> for Aabb {
    type Output = Aabb;

    fn add(self, offset: Vec3) -> Aabb {
        Aabb::new(
            self.x + offset.x(),
            self.y + offset.y(),
            self.z + offset.z(),
        )
    }
}

impl std::ops::Add<Aabb> for Vec3 {
    type Output = Aabb;

    fn add(self, bbox: Aabb) -> Aabb {
        bbox + self
    }
}
