fn main() {
    println!("Hello, world!");

    let a: i32 = 10;
    let b: i32 = 10;

    assert_eq!(a,10);
    assert_eq!(b,10);
    println!("Yes!!");
    muttable();
}
fn muttable(){
    let mut a:i32 = 5;
    let mut b: i32 = 10;

    a = 10;
    b = 29;
    println!("{}", a);
    println!("{}", b)
}
