// Rewrite the factorial function using a `while` loop.
pub fn factorial(n: u32) -> u32 {
    // The `todo!()` macro is a placeholder that the compiler
    // interprets as "I'll get back to this later", thus
    // suppressing type errors.
    // It panics at runtime.
    // todo!();
    // let mut fact: u32 = 0;

    let mut fact: u32 = 1;
    let mut i: u32 = n;

    if n == 1 || n == 0 {
        return 1;
    }
    
    while i > 1 {
        fact *= i;
        i -= 1;
    }

    return fact;

}

#[cfg(test)]
mod tests {
    use crate::factorial;

    #[test]
    fn first() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn second() {
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn fifth() {
        assert_eq!(factorial(5), 120);
    }
}
