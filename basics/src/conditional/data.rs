fn main(){
    let arr: [i32; 6] = [1,2,3,4,5,6];

    println!("The numbers are {:?}",arr);
    let fruits:[&str;3] = ["apple","mango","banana"];
    println!("Fruits are {:?}",fruits);

    //Tuples!!

    let human:(&str,i32,bool) = ("akash",20,true);
    println!("Tuple: {:?}", human);

    //Slices

    let num_slices: &[i32] = &[1,2,3,4,5];
    println!("Number slice: {:?}", num_slices);

    let book_slices: &[&str] = &["harry","GOT","HOTD","rich_dad"];
    println!("Book slice: {:?}", book_slices);
    let animal_slices: &[String] = &["lion".to_string(),"elephant".to_string(),"ostrich".to_string(),"monkey".to_string()];
    println!("animal slice: {:?}", animal_slices);

    // String and slicing!!

    let  mut one:String = String::from("Hell  ");
    println!("one says {}",one);
    one.push_str("i was lying!!!");
    println!("{}",one);
}