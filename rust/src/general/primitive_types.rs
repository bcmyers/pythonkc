#[allow(unused_variables)]
pub fn primitive_types() {

    // Booleans
    let a: bool = true;
    let b: bool = false;

    // Chars
    let c: char = 'a';
    let d: char = 'b';

    // Signed numerical types
    let e: i8 = 42;
    let f: i16 = 42;
    let g: i32 = 42;
    let h: i64 = 42;
    let i: isize = 42;

    // Unsigned numerical types
    let j: u8 = 42;
    let k: u16 = 42;
    let l: u32 = 42;
    let m: u64 = 42;
    let n: usize = 42;

    // Floating point numerical types
    let o: f32 = 42.0;
    let p: f64 = 42.0;

    // Arrays
    let q = [0i32; 5];
    let r: [i32; 5] = [0, 1, 2, 3, 4];
    println!("The array r has {} elements", r.len());
    println!("The 0th element of the array r is {}", r[0]);
    println!("The 100th element of the array r is {}\n",
             r.iter().nth(100).unwrap_or(&42));

    // Slices
    let s = &r[..]; // A slice containing all of the elements in b
    let t = &r[1..3]; // A slice of b: only the elements 1 and 2

    // str slices
    let u: &str = "€42";
    println!("The number of bytes in the str slice u is {}", u.len());
    println!("The 2nd byte in the str slice u is {:X}",
             u.as_bytes().iter().nth(2).unwrap_or(&42));
    println!("The number of characters in the str slice u is {}",
             u.chars().count());
    println!("The 0th character in the str slice u is {}\n",
             u.chars().nth(0).unwrap_or('!'));

    // Tuples
    let v: (i32, f32) = (42, 42.0);
    println!("The 0th element of the tuple v is {}\n", v.0);

    // Functions
    fn foo(w: i32) -> i32 {
        w
    }
    let x: fn(i32) -> i32 = foo;
    println!("The return value of the function x(42) is {}", x(42));

}
