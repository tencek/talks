use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone)]
pub enum CoffeeOrder {
    /// Simple method with no additional data - a unit variant
    Instant3In1,

    /// Espresso preparation with parameters - a struct variant
    Espresso { bean: Bean, strength: Strength },

    /// Pour-over method with name and temperature - a tuple variant
    PourOver(Temperature, Time),

    /// Variant for other brewing methods - represented by another enum
    Other(BrewingMethod),
}

#[derive(Debug, Clone, Copy)]
pub enum Bean {
    Arabica,
    Robusta,
    Blend(BlendType),
}

#[derive(Debug, Clone, Copy)]
pub enum BlendType {
    Arabica50Robusta50,
    Arabica40Robusta60,
    Arabica60Robusta40,
}

pub type Time = u32;
pub type Temperature = u8;

#[derive(Debug, Clone, Copy)]
pub enum Strength {
    Light,
    Medium,
    Strong,
}

#[derive(Debug, Clone, Copy)]
pub enum Size {
    Small,
    Medium,
    Large,
}

#[derive(Debug, Clone, Copy)]
pub enum BrewingMethod {
    FrenchPress,
    Aeropress,
    ColdBrew,
}

#[derive(Debug, Clone)]
pub struct Espresso {
    pub size: Size,
    pub strength: Strength,
    pub milk: Option<Milk>,
}

#[derive(Debug, Clone, Copy)]
pub enum Milk {
    Whole,
    Skim,
    Soy,
    Almond,
}

/// The new-type idiom is used to create a new type for a string
pub struct Coffee(pub String);

pub fn choose_cup_color(espresso: Espresso) -> String {
    let color_args = (espresso.size, espresso.strength);
    let cup_color = match color_args {
        (Size::Small, Strength::Strong) => "black",
        (Size::Medium, Strength::Strong) => "red",
        (Size::Large, Strength::Strong) => "brown",
        _ => "white",
    };
    cup_color.to_string()
}

pub fn make_espresso(espresso: Espresso) -> Coffee {
    let Espresso {
        size,
        strength,
        milk,
    } = espresso;
    milk.map_or_else(
        || {
            Coffee(format!(
                "Espresso of {:?} size and {:?} strength",
                size, strength
            ))
        },
        |milk| {
            Coffee(format!(
                "Espresso of {:?} size, {:?} strength, and {:?} milk",
                size, strength, milk
            ))
        },
    )
}

impl Display for CoffeeOrder {
    // pattern matching of enums with destructuring where needed
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            CoffeeOrder::Instant3In1 => write!(f, "Instant 3-in-1 coffee"),
            CoffeeOrder::Espresso { bean, strength } => write!(
                f,
                "Espresso with {:?} beans and {:?} strength",
                bean, strength
            ),
            CoffeeOrder::PourOver(temperature, time) => {
                write!(f, "Pour-over at {}°C for {} seconds", temperature, time)
            }
            CoffeeOrder::Other(method) => write!(f, "Other method: {:?}", method),
        }
    }
}

impl Display for Espresso {
    // pattern matching with destructuring
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Espresso {
                size,
                strength,
                milk: Some(milk),
            } => write!(
                f,
                "Espresso of {:?} size, {:?} strength, and {:?} milk",
                size, strength, milk
            ),
            Espresso {
                size,
                strength,
                milk: None,
            } => write!(
                f,
                "Espresso of {:?} size and {:?} strength, no milk",
                size, strength
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_coffee_order() {
        let order = CoffeeOrder::Espresso {
            bean: Bean::Blend(BlendType::Arabica50Robusta50),
            strength: Strength::Strong,
        };
        assert_eq!(
            format!("{}", order),
            "Espresso with Blend(Arabica50Robusta50) beans and Strong strength"
        );
    }

    #[test]
    fn test_make_espresso() {
        let espresso = Espresso {
            size: Size::Medium,
            strength: Strength::Medium,
            milk: Some(Milk::Whole),
        };
        let coffee = make_espresso(espresso);
        assert_eq!(
            coffee.0,
            "Espresso of Medium size, Medium strength, and Whole milk"
        );
    }

    #[test]
    fn test_choose_cup_color() {
        let espresso = Espresso {
            size: Size::Small,
            strength: Strength::Strong,
            milk: None,
        };
        assert_eq!(choose_cup_color(espresso), "black");
    }
}
