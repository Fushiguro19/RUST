use std::io;

fn main(){
    println!("please input your temperature in Celsius! : ");
    let mut temp = String::new();
    io::stdin()
        .read_line(&mut temp)
        .expect("Failed! Re-enter");
    
    let temp: u32 = match temp.trim().parse(){
        Ok(num) => num,
        Err(_) => 404,
    };
    
    println!("You entered {temp} degree Celsius .....");
    
    let fah = ((temp*9)/5)+32 ;
    println!("{} degree Celsius = {} degree Fahrenheit .... ", temp, fah);
}
