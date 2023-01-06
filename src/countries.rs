use crate::countries::powerful_countries::NuclearPowers;

pub mod powerful_countries {
        use std::fmt::{Display, Formatter};

        #[derive(Debug)] #[allow(dead_code)]
        pub enum NuclearPowers {
                India, USA, England, Australia
        }

}

pub fn get_country_capital(name: NuclearPowers) -> String {
        match name {
                NuclearPowers::India => String::from("New Delhi"),
                NuclearPowers::USA => String::from("Washington DC"),
                _ => String::from("Other city")
        }
}