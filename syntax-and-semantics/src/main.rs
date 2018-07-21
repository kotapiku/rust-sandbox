#![allow(unused)]
use std::cell::Cell;

fn main() {
    {
        // 1 variable-bindigns
        let x: i32 = 5;
        let (x, y) = (1, 2);
        let mut z = 3;
        z = 10;
        println!("{}, {}, {}", x, y, z);
    }

    {
        // 2 functions
        let f: fn(i32) -> i32 = add_one;
        let six = f(5);
    }

    {
        // 3 primitive-types
        let x: bool = true;
        let x: char = 'x';
        let x = 1;   // x: i32
        let y = 1.0; // y: f64
        let a = [0, 1, 2, 3, 4];  // a: [i32; 5]
        let complete = &a[..];
        let middle = &a[1..4];  // middle: &[T]
        let a = [0; 20];
        println!("size of a is {}", a.len());

        let x: (i32, &str) = (1, "hello");
        println!("first value of tuple x is : {}", x.0);
    }

    {
        // 5 if
        let x = 5;
        if x == 5 {
            println!("x is 5");
        } else if x == 6 {
            println!("x is 6");
        } else {
            println!("x is neither 5 nor 6");
        }
    }

    {
        // 6 loop
        // loop { println!("Loop forever") };
        let mut x = 5;
        let mut done = false;
        while !done {
            x += x - 3;
            if x % 2 == 0 { continue; }
            if x % 5 == 0 { break; }
        }

        for x in 0..5{
            println!("{}", x);
        }

        for (i, j) in (5..10).enumerate() {
            println!("i = {} and j = {}", i, j);
        }

        'outer: for x in 0..10 {
            'inner: for y in 0..10 {
                if x % 2 == 0 { continue 'outer; }
                if y % 2 == 0 { continue 'inner; }
            }
        }
    }

    {
        // 8 references-and-borrowing
        fn foo(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
            42
        }

        let v1 = vec![1, 2, 3];
        let v2 = vec![1, 2, 3];
        let answer = foo(&v1, &v2);
    }

    {
        // 9 lifetimes
        struct Foo<'a> {
            x: &'a i32,
        }

        impl<'a> Foo<'a> {
            fn x(&self) -> &'a i32 { self.x }
        }

        let y = &5;
        let f = Foo { x: y };
        println!("{}", f.x);

        let x: &'static str = "Hello, World";
    }

    {
        // 10 mutability
        struct Point {
            x: i32,
            y: Cell<i32>,   // field levelのmutability
        }

        let point = Point { x: 5, y: Cell::new(6) };
        point.y.set(7);
        println!("y: {:?}", point.y);
    }

    {
        // 11 struct
        struct Point3d {
            x: i32,
            y: i32,
            z: i32,
        }

        let mut point = Point3d { x: 0, y: 0, z: 0 };
        point = Point3d { y: 1, .. point };

        struct Color(i32, i32, i32);
        let brack = Color(0, 0, 0);

        struct Electron;
        let x = Electron;
    }

    {
        // 12 elems
    }
}

// 2 functions
fn print_number(x: i32) {
    println!("x is : {}", x);
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn foo(x: i32) -> i32 {
    return x;
    x + 1
}

fn diverges() -> ! {
    panic!("this function never returns!");
}
