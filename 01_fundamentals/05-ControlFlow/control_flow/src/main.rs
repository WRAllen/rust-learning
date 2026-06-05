use std::collections::HashMap;

fn main() {
    println!("-----------if / else-----------");
    let n = 5;
    // 和 go 一样，条件必须是 bool，不能像 python 那样写 if 1
    if n > 0 {
        println!("{} 是正数", n);
    } else if n == 0 {
        println!("零");
    } else {
        println!("负数");
    }

    println!("-----------if 作为表达式-----------");
    // rust 没有三元运算符 a ? b : c，用 if 表达式代替
    let label = if n > 0 { "正数" } else { "非正数" };
    println!("n = {}, label = {}", n, label);
    // if 两边返回值类型必须一致，下面这样会报错：
    // let wrong = if n > 0 { "正数" } else { 0 };

    println!("-----------loop 无限循环-----------");
    // loop 类似 go 的 for {}，靠 break 退出
    let mut count = 0;
    let result = loop {
        count += 1;
        if count == 3 {
            // break 可以带返回值，赋值给 result
            break count;
        }
    };
    println!("loop 退出时 count = {}, result = {}", count, result);

    println!("-----------while-----------");
    // 和 python/go 的 while 一样
    let mut num = 3;
    while num > 0 {
        println!("num = {}", num);
        num -= 1;
    }

    println!("-----------for 遍历范围-----------");
    // 0..3 左闭右开，输出 0 1 2，类似 python 的 range(0, 3)
    print!("0..3: ");
    for i in 0..3 {
        print!("{} ", i);
    }
    println!();

    // 0..=3 包含 3，输出 0 1 2 3
    print!("0..=3: ");
    for i in 0..=3 {
        print!("{} ", i);
    }
    println!();

    println!("-----------for 遍历 Vec-----------");
    // 03 里已经用过，这里再系统演示一下
    let v = vec![10, 20, 30];
    for item in v.iter() {
        println!("item = {}", item);
    }

    println!("-----------for enumerate 带索引-----------");
    // 03 里也用过，同时拿到索引和值
    for (index, value) in v.iter().enumerate() {
        println!("index = {}, value = {}", index, value);
    }

    println!("-----------for 遍历map-----------");
    let mut m = HashMap::new();
    m.insert("one", 1);
    m.insert("two", 2);
    m.insert("three", 3);
    for (key, value) in m.iter() {
        println!("key = {}, value = {}", key, value);
    }

    println!("-----------match 基本用法-----------");
    // match 比 go 的 switch 更严格，必须覆盖所有情况
    let code = 404;
    match code {
        200 => println!("成功"),
        404 => println!("未找到"),
        // _ 是通配符，兜底其他所有情况，类似 go 的 default
        _ => println!("其他状态码: {}", code),
    }

    // match 也是表达式，可以返回值
    let msg: &str = match code {
        200 => "ok",
        404 => "not found",
        _ => "unknown",
    };
    println!("msg = {}", msg);

    println!("-----------match 和 Option-----------");
    // 03 里 hashmap 取值时用过，这里单独演示
    let value: Option<i32> = Some(42);
    match value {
        Some(v) => println!("有值: {}", v),
        None => println!("没有值"),
    }

    let empty: Option<i32> = None;
    match empty {
        Some(v) => println!("有值: {}", v),
        None => println!("没有值"),
    }

    println!("-----------match guard-----------");
    // match 左边是模式，不是表达式，不能写 x + 1 => ...
    // 如果想加额外条件，用 guard：在 => 前写 if 条件
    let x = 2;
    let n = 3;
    match n {
        v if v == x + 1 => println!("n = {}，等于 x + 1", v),
        v if v > 10 => println!("n = {}，大于 10", v),
        _ => println!("n = {}，其他情况", n),
    }

    // guard 也可以让 match 作为表达式返回值(同基础)
    let score = 85;
    let grade = match score {
        s if s >= 90 => "优秀",
        s if s >= 60 => "及格",
        _ => "不及格",
    };
    println!("score = {}, grade = {}", score, grade);

    println!("-----------break 和 continue-----------");
    // 和 python/go 一样
    for i in 0..10 {
        if i % 2 == 0 {
            continue; // 跳过偶数
        }
        if i > 5 {
            break; // 大于 5 就退出
        }
        println!("奇数: {}", i);
    }
}
