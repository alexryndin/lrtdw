use std::arch::x86_64::*;
use std::f64::consts::PI;
use std::mem;
use std::alloc;

#[allow(non_upper_case_globals, non_camel_case_types, non_snake_case)]

const SOLAR_MASS: f64 = 4. * PI * PI;
const DAYS_PER_YEAR: f64 = 365.24;
const BODIES_COUNT: usize = 5;

#[repr(C)]
struct body {
    position: [f64; 3],
    velocity: [f64; 3],
    mass: f64,
}

fn main() {
    let test = PI;






}
