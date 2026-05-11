// use std::io;

// // ********************* Learning IO *******************
// // use std::io;
// fn main(){
//   let mut name = String::new();
//   println!("Enter your name: ");
//   io::stdin().read_line(&mut name).expect("Failed to read line");
//   println!("Hello, {}!", name.trim());
// }
// ********************* Learning HashMap *******************
use std::collections::HashMap;
#[derive(Debug, Clone)]
struct Bill {
    name: String,
    amount: f64,
}
fn main(){
  println!("Hello, world!");
  
  // let mut bills: Vec<Bill> = Vec::new();
  let mut bills: HashMap<String, Bill> = HashMap::new();
  bills.insert("Electricity".to_string(), Bill { name: "Electricity".to_string(), amount: 100.0 });
  bills.insert("Water".to_string(), Bill { name: "Water".to_string(), amount: 50.0 });
  println!("bills result: {:?}", bills.get("Electricity"));
  println!("water result ---------------");

  println!("bills result: {:?}", bills.get("Water"));
   
  println!("All data result ---------------");
  //  Converting HashMap values to a Vec for easier display
   let mut bill_values: Vec<&Bill> = bills.values().collect();
   println!("bills result: {:?}", bill_values  );
  //  Soerting by amount
   bill_values.sort_by(|a, b| b.name.partial_cmp(&a.name).unwrap());
   println!("Sorted bills by name: {:?}", bill_values);
    // for (key, bill) in &bills {
    //     println!("Key: {}, Bill: {:?}", key, bill);
    // }
}

// lo

// ********************* Expressions *******************
// fn main(){
//   enum Access {
//       Admin,
//       User
//   }

//     let results = match Access::Admin{
//       Access::Admin => "Admin",
//       Access::User => "User"
//     };
//   println!("who has access : {}",results);
// }

// lo

// *********************Struct and Enumeration *******************
// fn main(){
//   let (name,age) = ("name",23);
//   println!("name value: {}, the age value: {} ",name, age);
// }

// lo

// *********************Struct and Enumeration *******************
// enum Flavor {
//     Sweet,
//     Banana,
//     Orange
// }
// struct Drink {
//   flavor: Flavor,
//   cold_level: f32
// }
// fn render_drink(drink: Drink){
//   match drink.flavor {
//       Flavor::Banana => println!("flavor ->Banana"),
//       Flavor::Sweet => println!("flavor -> Sweet"),
//       Flavor::Orange => println!("flavor -> Orange"),
//   }
//   println!("Cold level: {:?}",drink.cold_level);
// }
// fn main(){
//    let drink_obj = Drink {
//      flavor: Flavor::Banana,
//      cold_level: 2.5
//    };
//    render_drink(drink_obj);

//    let drink_obj_sweet = Drink {
//      flavor: Flavor::Sweet,
//      cold_level: 4.1
//    };
//    render_drink(drink_obj_sweet);
// }

// // lo

// ********************* Enumeration *******************
// enum Color {
//     Red,
//     Green,
//     White
// }

// fn render_color(color:Color) {
//    match color {
//        Color::Green => println!("Green"),
//        Color::Red => println!("Red"),
//        Color::White => println!("White"),
//    }
// }

// fn main(){
//     render_color(Color::White);
// }

// lo
// ********************** Looping **************************

// fn main(){
//   println!("it's still running ..."); 
//   let mut number:i32 = 0;
  // -----looping in Rust-----
  // loop {
  //     if number == 5 {
  //       break;
  //     }
  //    number = number + 1;
  //    println!("num: {}",number)
  // }

  // ------- while -------
  // while number != 5 {
  //    number = number + 1;
  //    println!("num: {:?}",number)
  // }
// }

// lo

// ***************** RESULTS   Types **********************
// fn devide(a: i32, b: i32) -> Result<i32, String>{
//    if b == 0 {
//     //  let err_msg:String = "Not allowed it's zero".to_owned();
//     let message:String=String::from("division by zero");
//     //  Err(err_msg)
//     Err(message)
//     }else {
//       Ok(a / b)
//     }
// }

// fn main(){
//   let results = devide(8,0);
//   match results {
//     Ok(value)=>println!("value is: {}",value),
//     Err(error)=>println!("Error is: {}",error)     
//   }
//   // println!("New result: {}",results);
// }

// lo


// Enumeration enum by short
//  enum Direction {
//      Up,
//      Down,
//      Right,
//      Left
//  }

//  fn which_way(go: Direction) {
//       match go {
//     Direction::Down => println!("Direction down"),
//     Direction::Up => println!("Direction up"),
//     Direction::Right => println!("Direction right"),
//     Direction::Left => println!("Direction left"),
//      }
//  }

//  struct Restaurent<'a>{
//    name: &'a str,
//    price_rice: i32,
//    price_beans:i32
//  }

// enum Flavor{
//   Sparking,
//   Sweet,
//   Fruity
// }

// struct Drink{
//   flavor: Flavor,
//   fluid_oz: f64,
// }

// fn print_drink(drink: Drink){
//    match drink.flavor {
//        Flavor::Fruity=> println!("Fruit"),
//        Flavor::Sweet=> println!("Sweet"),
//        Flavor::Sparking=> println!("Sparking")
//    }
//    println!("Float {:?}",drink.fluid_oz);
// }

// fn main() {
   
   // let go: Direction = Direction::Up;
   //  which_way(Direction::Down);
   //struct part
  // let hakora_munda:Restaurent = Restaurent{
  //  name: "Mama Chegue",
  //  price_beans: 500,
  //  price_rice: 7800
  // };

  //  let juice:Drink = Drink{
  //   flavor: Flavor::Sparking,
  //   fluid_oz: 65.5
  //  };

  // print_drink(juice);

  // println!("{:?} ", hakora_munda.name);

// }
