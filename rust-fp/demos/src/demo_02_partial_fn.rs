/// counts the number of beans in a portion of a given weight.
/// It is a partial function because it is not defined for b == 0.
/// There is no way to not return a value, the only option is to panic.
pub fn count_beans_partial(portion_weight_mg: i32, bean_weight_mg: i32) -> i32 {
    // Undefined behavior if bean_weight == 0
    portion_weight_mg / bean_weight_mg
}

/// counts the number of beans in a portion of a given weight.
/// It is a total function because it is defined for all inputs.
/// It returns an `Option<i32>` to indicate that the result may not be available.
pub fn count_beans_total(portion_weight_mg: i32, bean_weight_mg: i32) -> Option<i32> {
    portion_weight_mg.checked_div(bean_weight_mg)
}

#[cfg(test)]
mod tests_partial_fn {
    use super::*;

    #[test]
    fn test_count_beans_partial_ok() {
        assert_eq!(count_beans_partial(6500, 130), 50);
    }

    #[test]
    #[should_panic(expected = "attempt to divide by zero")]
    fn test_count_beans_partial_panic() {
        count_beans_partial(6500, 0);
    }
}

#[cfg(test)]
mod tests_total_fn {
    use super::*;

    #[test]
    fn test_count_beans_total_ok() {
        assert_eq!(count_beans_total(6500, 130), Some(50));
    }

    #[test]
    fn test_count_beans_total_none() {
        assert_eq!(count_beans_total(6500, 0), None);
    }
}
