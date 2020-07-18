//Factorial
fn fact(n: i64) -> i64{
    if n==0{
        1
    } else {
        n*fact(n - 1)
    }
}

//Fibonacci number
fn fibo(n: i64) -> i64{
    fn fiboiter(n: i64, a: i64, b: i64) -> i64{
        if n==0{
            return a;
        }else{
            fiboiter(n-1, b, a+b)
        }
    }
    fiboiter(n, 0, 1)
}

//ArraySum
fn sum(func: fn(i32) -> i32, seq: &[i32]) -> i32{
    let mut acc: i32 = 0;
    for i in 0 .. seq.len(){
        acc += func(seq[i]);
        }
    acc
}

fn main() {
    for n in 10 .. 20{
        println!("{}", fact(n));
    }
    for n in 40 .. 50{
        println!("{}", fibo(n));
    }

    //local function
    fn identity(x: i32) ->i32 {x}
    fn square(x: i32) ->i32 {x*x}
    fn cube(x: i32) ->i32 {x*x*x}

    let seq: [i32; 10] = [1,2,3,4,5,6,7,8,9,10];
    println!("{}", sum(identity, &seq));
    println!("{}", sum(square, &seq));
    println!("{}", sum(cube, &seq));
}
