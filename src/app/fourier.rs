use eframe::egui;
use integrate::prelude::*;
use std::collections::HashMap;

pub struct C {
    real: f32,
    imag: f32
}

impl C {
    pub fn new(r: f32, i: f32) -> Self {
        Self {
            real: r,
            imag: i
        }
    }

    pub fn to_string(&self) -> String {
        match self.imag {
            i if i >= 0.0 => return format!("{} + {} i", self.real, i),
            i if i < 0.0 => return format!("{} - {} i", self.real, -i),
            _ => return "ERROR".to_string()
        }
    }

    pub fn to_eguipos(&self) -> egui::Pos2 {
        egui::pos2(self.real, self.imag)
    }

    pub fn multiply(&self, other: &C) -> C {
        let real = self.real * other.real - self.imag * other.imag;
        let imag = self.real * other.imag + self.imag * other.real;
        C::new(real, imag)
    }

    pub fn add(&self, other: &C) -> C {
        C::new(self.real + other.real, self.imag + other.imag)
    }

    pub fn negate(&self) -> C {
        C::new(-self.real, -self.imag)
    }

    pub fn scale(&self, t: f32) -> C {
        self.multiply(&C::new(t, 0.0))
    }
}

pub fn linear(t: f32, start: &C, end: &C) -> C{
    start.add(&(end.add(&start.negate())).scale(t))
}

pub fn f(t: f32) -> C {
    let z1 = C::new(0.5, 0.5);
    let z2 = C::new(-0.5, 0.5);
    let z3 = C::new(-0.5, -0.5);
    let z4 = C::new(0.5, -0.5);
    match t {
        t if (t >= 0.0 && t < 0.25) => linear(4.0 * t, &z1, &z2),
        t if (t >= 0.25 && t < 0.5) => linear(4.0 * (t - 0.25), &z2, &z3),
        t if (t >= 0.5 && t < 0.75) => linear(4.0 * (t - 0.5), &z3, &z4),
        t if (t >= 0.75 && t < 1.0) => linear(4.0 * (t - 0.75), &z4, &z1),
        _ => C::new(0.0, 0.0)
    }
}

pub fn g(t: f32) -> C {
    let z4 = C::new(0.75, 0.75);
    let z5 = C::new(0.75, -0.75);
    let z1 = C::new(0.0, 0.25);
    let z2 = C::new(-0.75, -0.75);
    let z3 = C::new(-0.75, 0.75);
    match t {
        t if (t >= 0.0 && t < 0.2) => linear(5.0 * t, &z1, &z2),
        t if (t >= 0.2 && t < 0.4) => linear(5.0 * (t - 0.2), &z2, &z3),
        t if (t >= 0.4 && t < 0.6) => linear(5.0 * (t - 0.4), &z3, &z4),
        t if (t >= 0.6 && t < 0.8) => linear(5.0 * (t - 0.6), &z4, &z5),
        t if (t >= 0.8 && t < 1.0) => linear(5.0 * (t - 0.8), &z5, &z1),
        _ => C::new(0.0, 0.0)
    }
}

pub fn h(t: f32) -> C {
    exp2pii(t).scale(0.5)
}

pub fn exp2pii(t: f32) -> C {
    let real = f32::cos(std::f32::consts::TAU * t);
    let imag = f32::sin(std::f32::consts::TAU * t);
    C::new(real, imag)
}

pub fn fourier_coefficient(f: fn(f32) -> C, n: i32) -> C {
    let real = trapezoidal_rule(
        |t: f32| (f(t).multiply(&exp2pii(- t * (n as f32)))).real,
        0.0,
        1.0,
        1000_usize
    );
    let imag = trapezoidal_rule(
        |t: f32| (f(t).multiply(&exp2pii(- t * (n as f32)))).imag,
        0.0,
        1.0,
        1000_usize
    );
    C::new(real as f32, imag as f32)
}

pub fn inverse_fourier(coefs: &HashMap<i32, C>, t: f32) -> C {
    let mut result = C::new(0.0, 0.0);
    for (n, coef) in coefs {
        let term = coef.multiply(&exp2pii(t * (*n as f32)));
        
        result = result.add(&term);
    }
    result
}

pub fn fourier_approximation(f: fn(f32) -> C, num_coefs: i32) -> impl Fn(f32) -> C {
    let mut coefs = HashMap::new();
    for n in -num_coefs..=num_coefs {
        let coef = fourier_coefficient(f, n);
        coefs.insert(n, coef);
    }
    move |t: f32| inverse_fourier(&coefs, t)
}
