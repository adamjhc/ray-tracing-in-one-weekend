use crate::vec3::{Point3, Vec3};
use rand::{thread_rng, Rng};

pub struct Perlin {
    ranvec: Vec<Vec3>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

impl Perlin {
    const POINT_COUNT: usize = 256;

    pub fn new() -> Self {
        Self {
            ranvec: (0..Self::POINT_COUNT)
                .map(|_| Vec3::random_within(-1.0, 1.0).unit_vector())
                .collect(),
            perm_x: Self::perlin_generate_perm(),
            perm_y: Self::perlin_generate_perm(),
            perm_z: Self::perlin_generate_perm(),
        }
    }

    pub fn turbulence(&self, p: &Point3, depth: i32) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = *p;
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }

        accum.abs()
    }

    pub fn noise(&self, p: &Point3) -> f64 {
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();

        let i = p.x.floor() as isize;
        let j = p.y.floor() as isize;
        let k = p.z.floor() as isize;

        let mut c = vec![vec![vec![Vec3::default(); 3]; 3]; 3];
        (0..2).for_each(|di| {
            (0..2).for_each(|dj| {
                (0..2).for_each(|dk| {
                    c[di][dj][dk] = self.ranvec[self.perm_x[((i + di as isize) & 255) as usize]
                        ^ self.perm_y[((j + dj as isize) & 255) as usize]
                        ^ self.perm_z[((k + dk as isize) & 255) as usize]];
                })
            })
        });

        Self::perlin_interp(c, u, v, w)
    }

    fn perlin_generate_perm() -> Vec<usize> {
        let p = (0..Self::POINT_COUNT).collect();

        Self::permute(p, Self::POINT_COUNT)
    }

    fn permute(mut p: Vec<usize>, n: usize) -> Vec<usize> {
        let mut thread_rng = thread_rng();
        (1..n).rev().for_each(|i| {
            let target = thread_rng.gen_range(0..=i);
            p.swap(i, target);
        });

        p
    }

    fn perlin_interp(c: Vec<Vec<Vec<Vec3>>>, u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);

        let mut accum = 0.0;
        (0..2).for_each(|i| {
            let fi = i as f64;
            (0..2).for_each(|j| {
                let fj = j as f64;
                (0..2).for_each(|k| {
                    let fk = k as f64;
                    let weight_v = Vec3::new(u - fi, v - fj, w - fk);
                    accum += (fi * uu + (1.0 - fi) * (1.0 - uu))
                        * (fj * vv + (1.0 - fj) * (1.0 - vv))
                        * (fk * ww + (1.0 - fk) * (1.0 - ww))
                        * c[i][j][k].dot(&weight_v);
                })
            })
        });

        accum
    }
}
