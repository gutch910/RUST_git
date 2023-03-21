// * 演習 FizzBuzz問題
// ・１から順に数え上げ、以下の条件で出力を行う
// ・３の倍数　    →　「Fizz」を出力
// ・５の倍数　    →　「Buzz」を出力
// ・３・５の倍数  →　「FizzBuzz」を出力
// ・そのほか      →　その数値を出力
// 
// 仕様
// ・fizzbuzz関数を作成し、main関数から呼び出す形で実装する
// ・fizzbuzz関数の引数に数え上げの終端の値を渡せるようにする
// 
// 手法
// ・while と if を使用した実装
// ・for と match を使用した実装
// ・match のパターンをタプルに置き換え
// 
fn main() {
    //println!("Hello, world!");
    fizzbuzz(1);
    //let a = print(10);
    //println!("Hello test {}", a);
}

fn fizzbuzz(x: i32) -> i32 {

    let mut limit = 10;
    //println!("{}", x);
    

    while limit != 0 {

    println!("{}",limit);


    


     limit -= 1;   
    }
    limit

}

//fn print(x: i32) -> i32 {
//    println!("fn test");
    //let x = 10;
//   x
//}
