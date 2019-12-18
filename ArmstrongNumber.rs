//Name : Mihir Deshpande
//Google code-in id : MihirD
//Organisation : CCEXTRACTOR DEVELOPMENT
//Task : Rust: Armstrong Numbers
use std::io;

fn main() 
{
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("failed to read input.");
    let n: i32 = user_input.trim().parse().expect("invalid input");
    //let n:i32 = 153;
    let mut temp:i32 = n;
    let mut temp1:i32 = n;
    let mut cnt:u32 = 0;
    let mut sum:i32 = 0;
    //Number has to be > 0
    while temp > 0
    {
        temp = temp / 10;
        cnt = cnt + 1;
    }
    while temp1 > 0
    {
        //sum of all digits, each raised to the power of the total digits
        let r:i32 = temp1 % 10;
        sum=sum + r.pow(cnt);
        temp1 /= 10;
    }
    if sum == n
    {
        println!("{} is an Armstrong number.", n);
    }
    else
    {
        println!("{} is not an Armstrong number.", n);
    }
}
