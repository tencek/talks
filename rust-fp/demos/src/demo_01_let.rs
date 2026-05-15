#[derive(Debug, Clone)]
pub struct CoffeeMachineSettings {
    pub grind_setting: GrindSetting,
    pub coffee_strength: CoffeeStrength,
    pub coffee_size: CoffeeSize,
}

#[derive(Debug, Clone)]
pub enum GrindSetting {
    Fine,
    Medium,
    Coarse,
}

#[derive(Debug, Clone)]
pub enum CoffeeStrength {
    Light,
    Medium,
    Strong,
}

#[derive(Debug, Clone)]
pub enum CoffeeSize {
    Small,
    Medium,
    Large,
}

impl Default for CoffeeMachineSettings {
    fn default() -> Self {
        CoffeeMachineSettings {
            grind_setting: GrindSetting::Medium,
            coffee_strength: CoffeeStrength::Medium,
            coffee_size: CoffeeSize::Medium,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CoffeeMachine(CoffeeMachineSettings);

impl From<CoffeeMachineSettings> for CoffeeMachine {
    fn from(settings: CoffeeMachineSettings) -> Self {
        CoffeeMachine(settings)
    }
}

impl CoffeeMachine {
    pub fn with_max_strength(self) -> Self {
        CoffeeMachine(CoffeeMachineSettings {
            coffee_strength: CoffeeStrength::Strong,
            ..self.0
        })
    }

    pub fn with_small_size(self) -> Self {
        CoffeeMachine(CoffeeMachineSettings {
            coffee_size: CoffeeSize::Small,
            ..self.0
        })
    }

    pub fn brew(&self) -> Result<Coffee, CoffeeMachineError> {
        Ok(Coffee {
            water_ml: match self.0.coffee_size {
                CoffeeSize::Small => 30,
                CoffeeSize::Medium => 60,
                CoffeeSize::Large => 120,
            },
            caffeine_mg: match self.0.coffee_strength {
                CoffeeStrength::Light => 20,
                CoffeeStrength::Medium => 30,
                CoffeeStrength::Strong => 40,
            },
        })
    }
}

#[derive(Debug, Clone)]
pub enum CoffeeMachineError {
    OutOfWater,
    OutOfBeans,
}

pub struct Coffee {
    pub water_ml: u32,
    pub caffeine_mg: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coffee_machine() {
        let coffee_machine = CoffeeMachine::from(CoffeeMachineSettings::default());
        let coffee_machine = coffee_machine.with_small_size();
        let coffee_machine = coffee_machine.with_max_strength();

        let coffee = coffee_machine.brew();
        assert!(coffee.is_ok());

        let coffee = coffee.unwrap();
        assert_eq!(coffee.water_ml, 30);
        assert_eq!(coffee.caffeine_mg, 40);
    }
}
