//represents fruit
enum Fruit{
    Apple, Banana, Grape, Orange
}

use fruit::*; //Omit fruit

//price
fn get_price(fruit: &Fruit) -> i32{
    match *fruit{
        Apple => 200,
        Banana => 150,
        Grape => 300,
        Orange => 100,
    }
}
