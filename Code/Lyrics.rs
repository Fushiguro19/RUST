//Print lyrics of the song "The twelve days of Christmas"
use std::io;

fn main(){
    // array of days :
    let days = ["first","second","third","fourth","fifth","sixth","seventh","eighth","ninth","tenth","eleventh","twelfth"];

    //array of gifts as per days
    let gifts = ["A partridge in a pear tree",
                "Two turtle doves and",
                "Three french hens",
                "Four calling birds",
                "Five golden rings",
                "Six geese a-laying",
                "Seven swans a-swimming",
                "Eight maids a-milking",
                "Nine ladies dancing",
                "Ten lords a-leaping",
                "Eleven pipers piping",
                "Twelve drummers drumming"];
    
    loop{
        println!("On the ____ day of Christmas ? : ENter a number! ");
        let mut n = String::new();
        io::stdin()
            .read_line(&mut n)
            .expect("Failed!! Re-enter");
        let n:usize = match n.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        
        for i in (0..n){
            println!("On the {} day of Christmas, my true love sent to me",days[i]);  
            for j in (0..i+1).rev(){
                println!("{}",gifts[j]);
            }
            println!("\n");
        }
        
        
    }
    
}
