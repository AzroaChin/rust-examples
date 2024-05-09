fn main() {
    let v = Some(0u8);
    match v {
        Some(3) => println!("Three"),
        _ => println!("Others")
    }

    if let Some(3) = v {
        println!("three")
    } else {
        println!("others")
    }
}
