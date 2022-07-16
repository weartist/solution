pub trait Area {
    fn area(&self) -> f32;
}

struct Rectangel {
    width: f32,
    height: f32,
}

impl Area for Rectangel {
    fn area(&self) -> f32 {
        return self.width * self.height
    }
}

struct Triangle {
    base: f32,
    height: f32,
}

impl Area for Triangle {
    fn area(&self) -> f32 {
        return (self.base * self.height) / 2.0
    }
}

struct Circle {
    radius: f32,
}

impl Area for Circle {
    fn area(&self) -> f32 {
        return 3.14 * self.radius.powf(2.0)
    }
}

pub fn print_area<T: Area>(item: T) {
    println!("area is {}",item.area());
}

fn main() {

    let a = Rectangel {
        width: 32.0,
        height: 33.0,
    };

    let b: Triangle = Triangle {
        base: (10.0),
        height: (20.0)
    };

    let c: Circle = Circle {
        radius: 10.0
    };

    print_area(a);
    print_area(b);
    print_area(c);
}