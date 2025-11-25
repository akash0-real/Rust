fn main(){
    let mut count = 0;
    let mut count1 = 0;
    let mut count2 = 0;
    for i in 1..=100{
        match (i%3, i%5) {
            (0,0) => {
                println!("{i}, FrizzBuzz");
                count += 1;
            },
            (0,_) => {
                println!("{i}, Frizz");
                count1 += 1;
            },
            (_,0) => {
                println!("{i}, Buzz");
                count2 += 1;
            },
            (_,_) => continue
        }
    }
    println!("Firzzbuzz count: {count}");
    println!("firzz count: {count1}");
    print!("Buzz count: {count2}");

}
