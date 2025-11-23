fn main(){
    let a = {
        let price: u16 = 10;
        let quant: u16 = 9;
        price * quant
    };
    println!("Total: {}",a);
    let x = add(10,20);
    println!("The sum is {}",x);
}
fn add(a: u16, b: u16) -> u16{
    a + b
}