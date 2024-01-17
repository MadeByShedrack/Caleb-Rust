fn main() {

    let mut index = 0;
    let mut sum = 0;

    while index < 13 {
        sum += index;
        index += 1;
    }

    println!("Sum -> {}", sum)
}