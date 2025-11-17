// Rewrite the factorial function using a `while` loop.
pub fn factorial(n: u32) -> u32 {
    // The `todo!()` macro is a placeholder that the compiler
    // interprets as "I'll get back to this later", thus
    // suppressing type errors.
    // It panics at runtime.

    // 1. conseguir el número anterior ✅
    // 2. Multiplicar el número actual por el anterior y guradaarlo en variable
    // 3. multiplicar el valor de esa variable por el número actual.

    // 5 x 4 = 20 -> 20 * 3 = 60 -> 60 * 2 -> 120 * 1 -> 120!

    let mut current_num = n;
    let mut current_factorial = n;

    while current_num > 1 {
        current_num -= 1;
        current_factorial *= current_num;
    }

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
