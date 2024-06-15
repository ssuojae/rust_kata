/// `print_wonyoung_thinking` 매크로는 주어진 표현식을 출력하고,
/// 그 뒤에 "완전 러키비키잖아🥰"를 덧붙입니다.
///
/// # 사용 예시
///
/// ```rust
/// print_wonyoung_thinking!("배고파");
/// // 출력: 배고파 완전 러키비키잖아🥰
#[allow(unused_macros)]
macro_rules! print_wonyoung_thinking {
    ($expression:expr) => {
        println!("{} 완전 러키비키잖아🥰", $expression);
    };
}

fn make_mutiplcation_table1() {
    for y in 1..10 {
        for x in 1..10 {
            print!("{:3},", x * y);
        }
        println!("");
    }
}

fn  make_mutiplcation_table2() {
    for y in 1..10 {
        let s = (1..10)
                .map(|x| format!("{:3}", x*y))
                .collect::<Vec<String>>().join(",");
            println!("{}", s);
    }
}

fn main() {
    println!("Make muliplication table 1. ============");
    make_mutiplcation_table1();

    println!("Make muliplication table 2. ============");
    make_mutiplcation_table2();

}
