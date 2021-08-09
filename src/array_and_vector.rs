use std::io;
pub fn table (x: i32) ->i32{
    let mut sum=0;
    println!(" ***************multiplaction table of {} *********************", x);
    for i in 1..11 {
        println!("{}  x  {} =   {}", x,i, x*i);
        sum+=x*i;
    }
    //println!("sum of multiplaction table {} =   {}", x,sum);
    return sum;
}
pub fn sum_of_numbers (x: i32){
    let mut sum=0;
    for i in 1..x+1 {
        
        //let mut n = String::new();
        //io::stdin().read_line(&mut n).expect("failed to read input.");
        //let n: i32 = n.trim().parse().expect("invalid input");
        sum+=i;
    }
    println!("Sum of N Numbers {}", sum);
}