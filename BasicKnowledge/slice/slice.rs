fn main(){
    let a = [1,2,3,4,5,6,7,8];
    println!("{}", a[0]);
    println!("{}", a.len());
    let a1 = a;                 //基本的な配列のコピー
    println!("{:?}", a);
    println!("{:?}", a1);

    let mut b = [0; 5];
    b[0] = 10;
    println!("{}", b[0]);
    let mut b1 = b;             //mutableも同様
    b1[0] = 20;
    println!("{:?}", b);
    println!("{:?}", b1);

    let c = &a[2..5];           //imutableのスライスはいくつでも作れる
    println!("{:?}", c);        //配列自体はコピーされない
    println!("{}", c.len());

    let d = &a[3..];
    println!("{:?}", d);
    println!("{}", d.len());

    {
        let e = &mut b[..];     //mutableな参照は１つのみ
        e[1] = 20;
        println!("{}", e[1]);
        /*compiler err
        b[1] = 30;
        */
    }
    b[1] = 30;                  //bにアクセス
    println!("{}", b[1]);
}
