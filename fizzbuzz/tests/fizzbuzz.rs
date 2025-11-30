use fizzbuzz::FizzBuzz;

static FB: FizzBuzz = FizzBuzz();

#[test]
fn returns_the_number() {
    assert_eq!(FB.of(1), "1");
    assert_eq!(FB.of(2), "2");
}

#[test]
fn multiples_of_3() {
    assert_eq!(FB.of(3), "Fizz");
    assert_eq!(FB.of(6), "Fizz");
    assert_eq!(FB.of(9), "Fizz");
}

#[test]
fn multiples_of_5() {
    assert_eq!(FB.of(5), "Buzz");
    assert_eq!(FB.of(10), "Buzz");
}

#[test]
fn multiples_of_3_and_5() {
    assert_eq!(FB.of(15), "FizzBuzz");
    assert_eq!(FB.of(30), "FizzBuzz");
}