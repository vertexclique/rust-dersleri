
fn multiply(x:i64,y:i64) ->i64
{
    return x*y;
}

fn main()
{
    let x: i32 = 15;
    let y: i64 = 1000;

    println!("{x} * {y} = {}", multiply(x.into(), y as i64));


    let z: i64 = 12783891;
    println!("{z} * {y} = {}", multiply(z, y as i64));
}