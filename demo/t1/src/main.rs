#[derive(Debug)]
struct Rectangular {
    width: u32,
    length: u32
}

fn main() {
    let rect = Rectangular {
        width: 30,
        length: 40
    };
    let r = count(&rect);
    println!("{:#?} -> {}", rect, r)
}

fn count(rect: &Rectangular) -> u32 {
    rect.width * rect.length
}
