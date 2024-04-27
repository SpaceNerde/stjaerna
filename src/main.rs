enum Class {
    O,
    B,
    A,
    F,
    G,
    K,
    M,
}

struct Star {
    passed_lifetime: f64,   // years
    mass: f64,              // sun masses
    radius: f64,            // kilometers
    luminsity: f64,         // watts
    energy_total: f64,      // joules
    class: Class,
    blackhole: bool,
    dead: bool,
} 

impl Star {
    fn new(mass: f64, radius: f64, class: Class, energy_total: f64, luminsity: f64) -> Star {
        Star {
            passed_lifetime: 0.,
            mass,
            radius,
            luminsity,
            energy_total,
            class,
            blackhole: false,
            dead: false,
        }
    }

    // lifetime of the star in years
    fn get_total_lifetime(&self) -> f64{
        self.energy_total / self.luminsity / 31500000000.
    }
}

fn main() {
    let mut test_star = Star::new(1., 696000., Class::G, 1.79 * 10_f64.powf(47.), 3.828 * 10_f64.powf(26.)); 
    println!("The total lifetime of our sun is: {} years", test_star.get_total_lifetime())
}
