pub trait MoreOptions<T> {
    fn is_none_or(&self, f: impl FnOnce(&T) -> bool) -> bool;
}

impl<T> MoreOptions<T> for Option<T> {
    fn is_none_or(&self, f: impl FnOnce(&T) -> bool) -> bool {
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
}
