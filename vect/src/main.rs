
// 整数のリストが与えられ、ベクタを使ってmean(平均値)、median(ソートされた時に真ん中に来る値)、 mode(最も頻繁に出現する値; ハッシュマップがここでは有効活用できるでしょう)を返してください。

fn main() {
    // println!("Hello, world!");

    // let v: Vec<i32> = Vec::new();
    let mut sum = 0;
    let mut count = 0;
    //let mut ave = 0;

    let mut v = vec![3, 2, 4, 5, 6, 0, 9];

    v.sort();
    for i in &v {
        println!("{}", i);
         
        sum += i; 
        count += 1;
    }
    let ave = sum / count ;
    println! ("sum= {}, count={}, ave={}", sum, count,ave);

    let median = count/2;
    println! ("median={}", median);
    let median_num = &v[median];
    println! ("median_num={}", median_num);
}
