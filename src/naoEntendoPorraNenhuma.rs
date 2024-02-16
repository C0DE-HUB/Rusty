fn main() {
    let mut counter = Some(0);

    while let Some(i) = counter {
        if i == 10 {
            counter = None;
        } else {
            println!("{i}");
            counter = Some(i + 1);
        }
    }
}
