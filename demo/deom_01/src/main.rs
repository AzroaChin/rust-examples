use std::mem::size_of;

#[derive(Debug)]
struct Rectangular {
    width: u32,
    length: u32
}

impl Rectangular {
    fn count(&self) -> u32 {
        self.width * self.length
    }

    fn can_hold(&self, other: &Rectangular) -> bool {
        self.width > other.width && self.length > other.length
    }

    fn square(size: u32) -> Rectangular {
        Rectangular{
            width: size,
            length: size
        }
    }
}

fn main() {
    let rect1 = Rectangular {
        width: 30,
        length: 40
    };
    let rect2 = Rectangular {
        width: 10,
        length: 50
    };
    let rect3 = Rectangular {
        width: 20,
        length: 20
    };
    println!("{}", rect1.can_hold(&rect2));
    println!("{}", rect1.can_hold(&rect3));

    // 关联函数: impl块里不把 self 作为第一个参数的函数叫关联函数（不是方法）
    let s = Rectangular::square(20);
    println!("{:#?}", s)
}
