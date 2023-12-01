//Find nth Fibonacci no

use std::io;

fn main(){
    loop{
        println!("enter nth position : ");
        let mut no = String::new();
        io::stdin()
            .read_line(&mut no)
            .expect("Failed! Re-enter");
        
        let no: u32 = match no.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        let y = fib(no);
    
        println!("The number at {} position in the fibonacci series is : {}\n", no, y);
        continue;
    }
    
    
}

fn fib(n:u32) -> u32{
    if n<=1{
        n
    } else {
        fib(n-1) + fib(n-2)
    }
}
