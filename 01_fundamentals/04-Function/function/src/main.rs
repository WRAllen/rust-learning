// 基本定义：两个 i32 参数，返回 i32
// 和 go 的 func add(a, b int) int 很像，rust 用 -> 声明返回类型
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// 最后一行是表达式，没有分号，自动作为返回值
fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

// 用 return 提前返回，和 go/python 一样
fn is_positive(n: i32) -> bool {
    if n > 0 {
        return true;
    }
    false
}

// 表达式 vs 语句：let x = 3 是语句（有分号），x + 1 是表达式（无分号，作为返回值）
fn demo_expression() -> i32 {
    // 编辑器会自动推导x的类型为i32
    let x = 3;
    x + 1
}

// 如果最后一行写成 x + 1; 会报错，因为加了分号后变成语句，函数就没有合法返回值了
// fn demo_wrong() -> i32 {
//     let x = 3;
//     x + 1;
// }

// 无返回值：省略 ->，等价于 -> ()
// () 叫 unit 类型，可以理解成"没有有意义的返回值"
fn say_hello() {
    println!("hello");
}

// 也可以显式写成 -> ()，效果和上面一样
fn say_hello_explicit() -> () {
    println!("hello, explicit");
}

// 接收 &str 参数，只读字符串时常用 &str（03 里学过）
fn greet(name: &str) {
    println!("hello, {}", name);
}

// 多个参数，混合数值和字符串
fn build_message(name: &str, age: i32) -> String {
    format!("name: {}, age: {}", name, age)
}

// 函数必须写在 main 外面，不能嵌套在 main 里面定义
fn main() {
    println!("-----------基本定义和调用-----------");
    // 调用方式和 python/go 一样：函数名 + 括号
    let result = add(1, 2);
    println!("add(1, 2) = {}", result);

    let product = multiply(3, 4);
    println!("multiply(3, 4) = {}", product);

    println!("-----------return 提前返回-----------");
    println!("is_positive(5) = {}", is_positive(5));
    println!("is_positive(-1) = {}", is_positive(-1));

    println!("-----------表达式 vs 语句-----------");
    println!("demo_expression() = {}", demo_expression());
    // 记住：函数最后一行想自动返回，就别加分号

    println!("-----------无返回值-----------");
    say_hello();
    say_hello_explicit();

    println!("-----------字符串参数-----------");
    greet("Allen");
    let msg = build_message("Allen", 25);
    println!("{}", msg);

    println!("-----------main 也是函数-----------");
    // main 是程序入口，rust 启动时自动调用，不需要我们手动调用
    // 上面所有 println! 都是在 main 这个函数里执行的
}
