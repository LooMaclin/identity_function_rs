// this is really cool identity function, hi from jerk.rs
pub fn idt<T>(t: T) -> T {
    t
}

#[cfg(test)]
mod tests {

    use super::idt;

    #[test]
    fn idt_test() {
        let a = Some(Some(0));
        let b = a.and_then(idt);
        assert_eq!(Some(0), b);
    }
}
