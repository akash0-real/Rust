fn main(){
    let weight_kg:f64 = 62.5;
    let height_m:f64 = 1.73;

    let a = calculate_bmi(weight_kg,height_m);
    println!("The Bmi is {:.2}",a);

}

fn calculate_bmi(weight_kg:f64,height_m:f64) -> f64{
    weight_kg / (height_m *height_m)
}