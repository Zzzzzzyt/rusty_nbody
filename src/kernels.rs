use crate::Vec3;

pub mod three_body;

#[derive(Clone, Debug)]
pub struct PhysicsState {
    pub p: Vec<Vec3>,
    pub v: Vec<Vec3>,
    pub m: Vec<f64>,
    pub t: u64,
}

impl PhysicsState {
    pub fn calc_kinetic_energy(&self) -> f64 {
        let mut e_k = 0.0;
        for i in 0..self.p.len() {
            e_k += self.m[i] * self.v[i].norm_squared();
        }
        return e_k / 2.0;
    }

    pub fn calc_potential_energy(&self) -> f64 {
        let mut e_p = 0.0;
        for i in 0..self.p.len() {
            for j in (i + 1)..self.p.len() {
                let r = self.p[i] - self.p[j];
                e_p -= self.m[i] * self.m[j] / r.norm();
            }
        }
        return e_p;
    }

    pub fn calc_total_energy(&self) -> f64 {
        return self.calc_kinetic_energy() + self.calc_potential_energy();
    }

    pub fn calc_momentum(&self) -> Vec3 {
        let mut p = Vec3::ZERO;
        for i in 0..self.p.len() {
            p += self.m[i] * self.v[i];
        }
        return p;
    }

    pub fn calc_center_of_mass(&self) -> Vec3 {
        let mut com = Vec3::ZERO;
        for i in 0..self.p.len() {
            com += self.m[i] * self.p[i];
        }
        return com / self.total_mass();
    }

    pub fn total_mass(&self) -> f64 {
        let mut m = 0.0;
        for i in 0..self.m.len() {
            m += self.m[i];
        }
        return m;
    }

    pub fn normalize(&mut self) {
        let com = self.calc_center_of_mass();
        for i in 0..self.p.len() {
            self.p[i] -= com;
        }

        let p = self.calc_momentum() / self.total_mass();
        for i in 0..self.p.len() {
            self.v[i] -= p;
        }
    }

    pub fn print_summary(&self) {
        let e_k = self.calc_kinetic_energy();
        let e_p = self.calc_potential_energy();
        let p = self.calc_momentum();
        let com = self.calc_center_of_mass();
        println!("Ep=  {:.10e}", e_p);
        println!("Ek=  {:.10e}", e_k);
        println!("E=   {:.10e}", e_p + e_k);
        println!("p=   {:?}", p);
        println!("CoM= {:?}", com);
    }

    pub fn print_errors(&self, state0: &PhysicsState) {
        let e_diff = self.calc_total_energy() - state0.calc_total_energy();
        println!("Energy error: {:.10e}", e_diff);
        println!(
            "Energy relative error: {:.5}",
            e_diff / state0.calc_total_energy().abs()
        );

        let p_diff = self.calc_momentum() - state0.calc_momentum();
        println!("Momentum error: {:.10e}", p_diff.norm());

        let com_diff = self.calc_center_of_mass() - state0.calc_center_of_mass();
        println!("CoM error: {:.10e}", com_diff.norm());
    }

    pub fn print_deviation(&self, ground_truth: &PhysicsState) -> (f64, f64, f64, f64) {
        let mut p_std: f64 = 0.0;
        let mut v_std: f64 = 0.0;
        let mut p_diff_max: f64 = 0.0;
        let mut v_diff_max: f64 = 0.0;
        for i in 0..self.p.len() {
            let p_diff = (self.p[i] - ground_truth.p[i]).norm_squared();
            let v_diff = (self.v[i] - ground_truth.v[i]).norm_squared();
            p_diff_max = p_diff_max.max(p_diff.sqrt());
            v_diff_max = v_diff_max.max(v_diff.sqrt());
            p_std += p_diff;
            v_std += v_diff;
        }
        p_std /= self.p.len() as f64;
        v_std /= self.p.len() as f64;
        p_std = p_std.sqrt();
        v_std = v_std.sqrt();
        println!("Position standard deviation: {:.10e}", p_std);
        println!("Velocity standard deviation: {:.10e}", v_std);
        println!("Position max deviation: {:.10e}", p_diff_max);
        println!("Velocity max deviation: {:.10e}", v_diff_max);
        return (p_std, v_std, p_diff_max, v_diff_max);
    }
}
