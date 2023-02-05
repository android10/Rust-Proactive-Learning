pub fn scratch_sheet() {
    scratch();
    simple_references();
    box_references();
    vectors();
}

fn scratch() {
    println!("--------------------------------------------------------------");
    println!("SCRATCH SHEET:");
    println!("--------------------------------------------------------------");
    println!("Book: Programming Rust 2nd Edition");
    println!("--------------------------------------------------------------");
}

fn simple_references() {
    let _my_value = 6_u8;
    let _my_reference = &_my_value;
}

fn box_references() {
    let _eggs = (12, "eggs");
    let _eggs_reference_heap = Box::new(_eggs);
}

fn vectors() {
    let mut primes = vec![2, 3, 5, 7];
    assert_eq!(primes.len(), 4);
    assert_eq!(primes.iter().product::<i32>(), 210);
}