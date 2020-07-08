fn main(){
    let a = vec![1,2,3,4,5]; //Array
    println!("{:?}", a); //print[1,2,3,4,5]
    let b = a;
    println!("{:?}", b); //print[1,2,3,4,5]
    /* compile err
    Println!("{:?}",a);
    */
    let c: Vec<i32>;
    {
        let d = b;
        /* compile err
        Println!("{:?}", b);
        */
        println!("{:?}", d);
        c = d;
    }
    println!("{:?}", c);
}
