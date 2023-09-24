// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration {
    value: u64,
}

impl From<u64> for Duration {
    fn from(value: u64) -> Self {
        Self { value }
    }
}

pub trait Planet {
    const ORBITAL_PERIOD: f64;

    fn years_during(d: &Duration) -> f64 {
        (d.value as f64) / (Self::ORBITAL_PERIOD * 3_1557_600.0)
    }
}

macro_rules! planet {
    ($i:ident, $orbital_period:tt) => {
        pub struct $i;

        impl Planet for $i {
            const ORBITAL_PERIOD: f64 = $orbital_period;
        }
    };
}

planet!(Mercury, 0.2408467);
planet!(Venus, 0.61519726);
planet!(Earth, 1.0);
planet!(Mars, 1.8808158);
planet!(Jupiter, 11.862615);
planet!(Saturn, 29.447498);
planet!(Uranus, 84.016846);
planet!(Neptune, 164.79132);
