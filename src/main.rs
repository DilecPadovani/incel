use std::env;
use std::collections::HashMap;
use std::fs;
use colored::Colorize;
fn extract_json(file: &str) -> serde_json::value::Value {
    let file = fs::File::open(file).unwrap();
    let json: serde_json::Value = serde_json::from_reader(file).unwrap();
    json
}


fn main() {
   let args: Vec<String> = env::args().skip(1).collect();
   println!("{:?}", args);
    
    let data = extract_json("data.json");
  //let data = &data.to_str().unwrap();
   println!("{:?}", data );
 // let m: HashMap<String ,String> = serde_json::from_str(data).unwrap();
//  println!("{:?}", m);
  for a in &args {
        
 //       for (key, value) in &m {
//    if value.to_uppercase() == a.to_uppercase() {
//        println!("{} --> {}", key.green(), value);
//    }
// }

        
        println!(" {} --> {:?}" , a.green() , data[a] );
       }
            

        

    


}
