use crate::{util, vec3::Vec3};

#[cfg(all(
    target_feature = "avx2",
    target_feature = "sse2",
    target_feature = "fma",
    any(target_arch = "x86_64", target_arch = "x86")
))]
pub mod vel_verlet_avx2;

pub mod rk4;
pub mod vel_verlet;
pub mod yoshida4;

#[derive(Clone, Copy, Debug)]
pub struct State3 {
    pub p: [Vec3; 3],
    pub v: [Vec3; 3],
    pub m: [f64; 3],
    pub t: u64,
}

impl State3 {
    pub fn calc_kinetic_energy(self) -> f64 {
        return (self.v[0].len2() * self.m[0]
            + self.v[1].len2() * self.m[1]
            + self.v[2].len2() * self.m[2])
            / 2.0;
    }

    pub fn calc_potential_energy(self) -> f64 {
        return -(self.m[0] * self.m[1] / (self.p[0] - self.p[1]).len()
            + self.m[0] * self.m[2] / (self.p[0] - self.p[2]).len()
            + self.m[1] * self.m[2] / (self.p[1] - self.p[2]).len())
            * util::GRAVITY_CONSTANT;
    }

    pub fn calc_total_energy(self) -> f64 {
        return self.calc_kinetic_energy() + self.calc_potential_energy();
    }

    pub fn calc_momentum(self) -> Vec3 {
        return self.v[0] * self.m[0] + self.v[1] * self.m[1] + self.v[2] * self.m[2];
    }

    pub fn calc_center_of_mass(self) -> Vec3 {
        return (self.p[0] * self.m[0] + self.p[0] * self.m[0] + self.p[0] * self.m[0])
            / self.total_mass();
    }

    pub fn total_mass(self) -> f64 {
        return self.m[0] + self.m[1] + self.m[2];
    }

    pub fn print_summary(self) {
        let Ek = self.calc_kinetic_energy();
        let Ep = self.calc_potential_energy();
        let p = self.calc_momentum();
        let CoM = self.calc_center_of_mass();
        println!(
            "Ep= {:.10e}\nEk= {:.10e}\nE=  {:.10e}\np=  {:?}\nCoM= {:?}",
            Ep,
            Ek,
            Ep + Ek,
            p,
            CoM
        );
    }
}
