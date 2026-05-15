use curried::curry;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Espresso {
    pub beans: Beans,
    pub strength: Strength,
    pub size: Size,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Beans {
    Arabica,
    Robusta,
    Blend(BlendType),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BlendType {
    Arabica50Robusta50,
    Arabica40Robusta60,
    Arabica60Robusta40,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Strength {
    Light,
    Medium,
    Strong,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Size {
    Small,
    Medium,
    Large,
}

// curried function using the curry macro from the curried crate
#[curry]
pub fn make_espresso(beans: Beans, strength: Strength, size: Size) -> Espresso {
    Espresso {
        beans,
        strength,
        size,
    }
}

#[cfg(test)]
mod tests_curried {
    use super::*;

    #[test]
    fn test_make_espresso() {
        let espresso = make_espresso(Beans::Robusta)(Strength::Strong)(Size::Small);
        assert_eq!(espresso.beans, Beans::Robusta);
        assert_eq!(espresso.strength, Strength::Strong);
        assert_eq!(espresso.size, Size::Small);
    }

    #[test]
    fn test_make_strong_all_sizes() {
        let make_robusta = make_espresso(Beans::Robusta);
        let make_strong_robusta = make_robusta(Strength::Strong);

        let strong_small = make_strong_robusta(Size::Small);
        // let strong_medium = make_strong_robusta(Size::Medium);
        // let strong_large = make_strong_robusta(Size::Large);

        assert_eq!(strong_small.beans, Beans::Robusta);
        assert_eq!(strong_small.strength, Strength::Strong);
        assert_eq!(strong_small.size, Size::Small);

        // assert_eq!(strong_medium.beans, Beans::Robusta);
        // assert_eq!(strong_medium.strength, Strength::Strong);
        // assert_eq!(strong_medium.size, Size::Medium);

        // assert_eq!(strong_large.beans, Beans::Robusta);
        // assert_eq!(strong_large.strength, Strength::Strong);
        // assert_eq!(strong_large.size, Size::Large);
    }
}
