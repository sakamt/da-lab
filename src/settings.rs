use super::da;

pub trait Induce<Dest> {
    fn induce(&self) -> Dest;
}

#[derive(RustcDecodable, Debug, Clone)]
pub struct Truth {
    pub dt: f64,
    pub length: usize,
}

#[derive(RustcDecodable, Debug, Clone)]
pub struct Observation {
    pub dt: f64,
    pub tau: usize,
    pub count: usize,
    pub r: f64,
}

impl Induce<Truth> for da::Setting {
    fn induce(&self) -> Truth {
        Truth {
            dt: self.dt,
            length: self.tau * self.count,
        }
    }
}

impl Induce<Observation> for da::Setting {
    fn induce(&self) -> Observation {
        Observation {
            dt: self.dt,
            tau: self.tau,
            count: self.count,
            r: self.r,
        }
    }
}
