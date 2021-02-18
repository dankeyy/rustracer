use crate::{hittable::*, vectors::*, rays::Ray};

pub struct hittable_list {
    objects: Vec<Box<dyn hittable>>
}

impl hittable_list {

    fn new() -> hittable_list {
        hittable_list {
            objects: Vec::new()
        }
    }

    fn add(&self, object: box<dyn hittable>) -> hittable_list {
        self.objects.push(object);
    }
}

impl hit for hittable {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut hit_record) {
        let mut tmp_rec: &mut hit_record = 
        let hit_anything = false;
        let closest_so_far = t_max;

        for object in self.objects.iter() {
            if 
        }
    }
}

