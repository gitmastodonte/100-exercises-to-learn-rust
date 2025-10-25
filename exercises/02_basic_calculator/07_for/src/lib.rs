// Rewrite the factorial function using a `for` loop.
pub fn factorial(n: u32) -> u32 {
    // 1. conseguir el número anterior ✅
    // 2. Multiplicar el número actual por el anterior y guradaarlo en variable
    // 3. multiplicar el valor de esa variable por el número actual.

    // 5 x 4 = 20 -> 20 * 3 = 60 -> 60 * 2 -> 120 * 1 -> 120!
    let mut current_factorial = n;

    for current_n in (1..n).rev() {k
        current_factorial = current_factorial * current_n;
    }

    if current_factorial == 0 {
        1
    } else {
        current_factorial
    }
}

fn main() {
    let factorial = factorial(2);

    println!("Factorial final: {}", factorial)
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
