fn main() {
    // for number in (1..4).rev() {
    //     println!("{}!", number)
    // }

    // let x = 5;
    // let y = x;
    // println!("{},{}",x, y)

    let my_string = String::from("Hello world");
    let world_index = first_world(&my_string);

    let my_string_literal = "Hello world";
    let world_index_literal = first_world(my_string_literal);

    println!("{}, {}", world_index, world_index_literal)
}

fn first_world(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
