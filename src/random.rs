use crate::vec3::Vec3;

pub fn sample_square() -> Vec3 {
    Vec3::new(fastrand::f64() - 0.5, fastrand::f64() - 0.5, 0.0)
}

pub fn random_unit_vector() -> Vec3 {
    loop {
        let p = Vec3::new(
            2.0 * fastrand::f64() - 1.0,
            2.0 * fastrand::f64() - 1.0,
            2.0 * fastrand::f64() - 1.0,
        );
        let lp = p.dot(&p);
        if lp > 1e-160 && lp <= 1.0 {
            return p / lp.sqrt();
        }
    }
}

pub fn random_unit_disk() -> Vec3 {
    loop {
        let p = 2.0 * sample_square();
        if p.dot(&p) < 1.0 {
            return p;
        }
    }
}
