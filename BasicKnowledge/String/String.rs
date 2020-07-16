fn main(){
    let mut a = "hello,".to_string(); //data type is "&str (&'static str)"
    println!("{}", a);
    a.push_str("world"); //add character
    println!("{}", a);
    let b = a+",opps!"; //linking
    println!("{}", b);
}
