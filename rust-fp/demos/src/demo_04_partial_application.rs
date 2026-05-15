pub struct Espresso {
    pub strength: Strength,
    pub size: Size,
}

#[derive(Debug, Clone, Copy)]
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

// standard function, no cyrrying
pub fn make_espresso(strength: Strength, size: Size) -> Espresso {
    Espresso { strength, size }
}

#[cfg(test)]
mod test_no_currying {
    use super::*;

    #[test]
    fn test_make_espresso() {
        let espresso = make_espresso(Strength::Strong, Size::Small);
        assert_eq!(espresso.strength, Strength::Strong);
        assert_eq!(espresso.size, Size::Small);
    }

    #[test]
    fn test_make_strong_all_sizes() {
        let make_strong = |size| make_espresso(Strength::Strong, size);
        let strong_small = make_strong(Size::Small);
        let strong_medium = make_strong(Size::Medium);
        let strong_large = make_strong(Size::Large);

        assert_eq!(strong_small.strength, Strength::Strong);
        assert_eq!(strong_small.size, Size::Small);

        assert_eq!(strong_medium.strength, Strength::Strong);
        assert_eq!(strong_medium.size, Size::Medium);

        assert_eq!(strong_large.strength, Strength::Strong);
        assert_eq!(strong_large.size, Size::Large);
    }
}

// curried function
pub fn make_espresso_curried(strength: Strength) -> impl Fn(Size) -> Espresso {
    move |size| Espresso { strength, size }
}

#[cfg(test)]
mod tests_curried {
    use super::*;

    #[test]
    fn test_make_espresso_curried() {
        let espresso = make_espresso_curried(Strength::Strong)(Size::Small);
        assert_eq!(espresso.strength, Strength::Strong);
        assert_eq!(espresso.size, Size::Small);
    }

    #[test]
    fn test_make_strong_all_sizes() {
        let make_strong = make_espresso_curried(Strength::Strong);
        let espresso_small = make_strong(Size::Small);
        let espresso_medium = make_strong(Size::Medium);
        let espresso_large = make_strong(Size::Large);

        assert_eq!(espresso_small.strength, Strength::Strong);
        assert_eq!(espresso_small.size, Size::Small);

        assert_eq!(espresso_medium.strength, Strength::Strong);
        assert_eq!(espresso_medium.size, Size::Medium);

        assert_eq!(espresso_large.strength, Strength::Strong);
        assert_eq!(espresso_large.size, Size::Large);
    }
}
