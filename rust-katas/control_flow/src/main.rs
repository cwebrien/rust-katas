fn main() 
{
    let result: i64 = {
        let mut counter = 4;
        let mut a = 2;
        while counter > 0
        {
            a = a * a;
            counter = counter - 1;
        }
        a
    };
    println!("Result: {}", result);
}
