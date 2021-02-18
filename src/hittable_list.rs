use crate::{hittable::{Hittable, HitRecord}, rays::Ray};


pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>
}


impl HittableList {

    pub fn new() -> HittableList{
        HittableList {
            objects: Vec::new()
        }
    }

    pub fn add(&self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}


impl Hittable for HittableList {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let hit_anything = false;
        let closest_so_far = t_max;

        for object in self.objects.iter() {
            let mut tmp_rec = HitRecord::new();

            if object.hit(r, t_min, closest_so_far, &mut tmp_rec) {
                hit_anything = true;
                closest_so_far = tmp_rec.t;
                *rec = tmp_rec;
            }
        }

        hit_anything
    }
}

