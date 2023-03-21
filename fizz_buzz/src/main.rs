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

fn main() {
//    println!("Exec FizzBuzz");
    //fizz_buzz_1(30);
    println!("Exec FizzBuzz Type while if");
    fizz_buzz_1(30);
    println!("Exec FizzBuzz Type for match");
    fizz_buzz_2(30);
}


fn fizz_buzz_2(max_number: i32){
 
    for mut number in 0..max_number {
    
     println!("number is {}", number);
     mutch;
     number += 1;
    }
}



fn fizz_buzz_1(max_number: i32){
   let mut number = 1;

   while number <= max_number {

    if number % 3 == 0 && number % 5 == 0{
        println!("The number is : FizzBuzz !")
    
    }
    else if number % 3 == 0 {
    println!("The number is : Fizz !")

    }
    else if number % 5 == 0{
        println!("The number is : Buzz !")
    
    }
    else {
    println!("The number is : {}",number)
    }

    number += 1;
   }


}


