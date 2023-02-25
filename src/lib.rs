pub trait MoreOptions<T> {
    fn is_none_or(&self, f: impl FnOnce(&T) -> bool) -> bool;
    fn owned_is_none_or(self, f: impl FnOnce(T) -> bool) -> bool;
}

impl<T> MoreOptions<T> for Option<T> {
    fn is_none_or(&self, f: impl FnOnce(&T) -> bool) -> bool {
        match self {
            Some(v) => f(v),
            None => true,
        }
    }

    fn owned_is_none_or(self, f: impl FnOnce(T) -> bool) -> bool {
        match self {
            Some(v) => f(v),
            None => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::MoreOptions;

    #[test]
    fn is_none_or_should_equal_value() {
        let input = Some("test");
        assert!(input.is_none_or(|x| x == &"test"));
    }

    #[test]
    fn is_none_or_should_be_none() {
        let input: Option<&str> = None;
        assert!(input.is_none_or(|x| x == &"test"));
    }

    #[test]
    fn is_none_or_should_return_false_on_different_value() {
        let input = Some("test");
        assert!(!input.is_none_or(|x| x == &"test2"));
    }

    #[test]
    fn is_none_or_should_return_literal_bool() {
        let input = Some("test");
        assert!(input.is_none_or(|_| true));
    }

        #[test]
    fn owned_is_none_or_should_equal_value() {
        let input = Some("test");
        assert!(input.owned_is_none_or(|x| x == "test"));
    }

    #[test]
    fn owned_is_none_or_should_be_none() {
        let input: Option<&str> = None;
        assert!(input.owned_is_none_or(|x| x == "test"));
    }

    #[test]
    fn owned_is_none_or_should_return_false_on_different_value() {
        let input = Some("test");
        assert!(!input.owned_is_none_or(|x| x == "test2"));
    }

    #[test]
    fn owned_is_none_or_should_return_literal_bool() {
        let input = Some("test");
        assert!(input.owned_is_none_or(|_| true));
    }
}
