// The sum of the squares of the first ten natural numbers is,
//  1.pow(2) + 2.pow(2) + ... + 10.pow(2) = 385.
// The square of the sum of the first ten natural numbers is,
// (1 + 2 + ... + 10).pow(2) = 55.pow(2) = 3025.
// Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is
// 3025 - 385 = 2640
// Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.

fn main() {
    // default value: 100
    let amount = 10000;

    let sum_of_squares: u128 = (1..=amount).map(|x: u128| x.pow(2)).sum();
    let squares_of_sum: u128 = (1..=amount).sum::<u128>().pow(2);
    let difference = squares_of_sum - sum_of_squares;

    // Creative name I know | idk if better but it works ¯\_(ツ)_/¯
    let difference2: u128 = (1..=amount).sum::<u128>().pow(2) - (1..=amount).map(|x: u128| x.pow(2)).sum::<u128>();


    println!("{}", difference);
    println!("--------------------------------");
    println!("{}", difference2)

}
