use std::collections::HashMap;

//fn for median
fn median(v : Vec<i32>) -> f32 {
    let l = v.len();
    if l%2 != 0 {
        let median = v[(l-1)/2];
        println!("The median for {:?} " , v);
        median as f32
    } else {
        let x = l-1;
        let sum = (v[(x-1)/2] + v[(x+1)/2]);
        let median  = sum as f32 / 2.0;
        println!("the median is " );
        median as f32
    }
}

//to calculate mode
fn mode(v : Vec<i32>){
    let mut max_value = 0;
    let mut max_key = 0;
    let mut map = HashMap::new();
    for i in v{
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
    
    for (i,j) in map {
        println!("{} : {}", i,j);
        if j > max_value{
            max_value = j;
            max_key = i;
        }
    }
    println!("Mode is : {} with frequency : {}", max_key, max_value);
}

//main fn
fn main() {
    let mut v = vec![4,4,3,1,3,5,5,2];
    println!("vector is : {:?} ", v);
    v.sort();
    println!("sorted form is {:?}", v);
    
    //to find the median : 
    println!("{:?}", median(v.clone()));
    
    //for mode
    println!("{:?}", mode(v.clone()));
}
