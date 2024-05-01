use fizz_buzz_golf::fizz_buzz;

fn main() {
    println!("INDEX | RESULT");
    println!("--------------");

    for (index, result) in (1..=20).map(|index| (index, fizz_buzz(index))) {
        println!("{index:>5} | {result}");
    }
}
