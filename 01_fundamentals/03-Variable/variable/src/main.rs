// 字典使用
use std::collections::HashMap;

// 定义的常量
// 常量必须写类型，并且要大写，通过_链接单词，全局可用
const MAX_RETRY_TIME: usize = 3;
// 和c#类似，在语句要加上; 但是他和c#又不一样，对于表达式他不需要加;
// 这里先不拓展，记住大多数情况下都要加;
// str 是字符串内容对应的动态大小类型
// &str 是对 str 的借用/切片引用，平时真正常用的是 &str，不是裸 str
// 下文具体说明
const DEFALUT_USERNAME: &str = "Allen";

fn main() {
    println!("-----------数值类型-----------");
    // println!为宏函数，可以先简单理解为普通print函数
    println!("MAX_RETRY_TIME: {} DEFALUT_USERNAME: {}", MAX_RETRY_TIME, DEFALUT_USERNAME);
    // 首先先是简单的变量，定义数值类型的例子
    let a1: i32 = 1;
    let a2: f32 = 2.0;
    println!("a1: {} a2: {}", a1, a2);
    // usize是usize类型，是无符号整数，常用于表示长度、索引和下标（这个在for循环里面会用到）
    // usize的大小和具体平台有关，在64位系统上，usize是64位，在32位系统上，usize是32位（现在应该没有32位系统了吧:D）
    let a3: usize = 3;
    println!("a3: {}", a3);
    // 这里和go一样，不同的整型也是不能直接相加的
    let a4: i64 = a1 as i64 + 2;
    let a5: i32 = a4 as i32 + 1;
    let a6: usize = a4 as usize + 2;
    println!("a4: {} a5: {} a6: {}", a4, a5, a6);

    println!("-----------字符串类型-----------");
    // 下面是字符串类型的例子
    // "hello" 的类型通常是 &'static str 是一个“字面量”
    // str是字符串切片对应的“底层动态大小类型”，几乎不会单独拿来用（str 是“字符串内容本体”的类型，表示一段 UTF-8 文本。但它是 动态大小类型，大小在编译期不固定）
    // &str是字符串切片的引用，不拥有数据（指向一段 UTF-8 字符串数据，大小在编译期固定）
    // 字符串字面量 "hello" 在编译时就确定了，Rust 会把它放到程序的只读静态区域里，所以它本身不是一个动态分配、可增长的字符串对象
    // 所以下面要用&str，而不是str（所以字面量 "hello" 虽然更精确是 &'static str，但平时你也可以把它当成 &str 来理解。）
    let s1: &str = "hello";
    // 而String 是有所有权的、可增长的 UTF-8 字符串，数据通常放在堆上。
    let mut s2: String = String::from("worl");
    s2.push_str("d");
    println!("s1: {} s2: {}", s1, s2);
    // 类似python的[0:3],也是左闭右开
    // 但是有个坑 英文 "world" 这种没问题，因为每个字符都是 1 个字节。但是中文 "世界" 这种就不行了，因为每个字符都是 3 个字节。
    let s2_1: &str = &s2[0..3];
    println!("s2_1: {}", s2_1);
    // 清空字符串
    s2.clear();
    s2.push_str("你好世界");
    // 下面如果是0..4的话会报错，因为中文"你好世界"是 12 个字节，每个中文3字节，而你只取了 4 个字节，切在了非法 UTF-8 字符边界上
    // 报错如下：
    // thread 'main' (22706966) panicked at 01_fundamentals/03-Variable/variable/src/main.rs:46:25:
    // byte index 4 is not a char boundary; it is inside '好' (bytes 3..6) of `你好世界`
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    let s2_2: &str = &s2[0..3];
    println!("s2_2: {}", s2_2);
    // 下面是正确的做法
    // 取[1..2]，这里 collect() 产生的是一个新的字符串，所以最自然接收的是 String
    let part: String = s2.chars().skip(1).take(2).collect();
    println!("part: {}", part);
    // 下面这样可以变成&str类型
    let part_str: &str = &part;
    println!("part_str: {}", part_str);
    
    // 这里不能对good这个字符串做例如String的push_str的操作
    // s3 的类型是 &str
    // &str 不拥有底层字符串，也不能原地扩容
    // 所以不能 push_str
    // 这里的mut 只是说明s3这个变量是可以重新赋值的
    let mut s3: &str = "good";
    s3 = "bad";
    println!("s3: {}", s3);
    // 上面小结一下
    // &str：当你只是借用并读取字符串内容时常用，可以简单理解为当你不需要操作这个字符串时
    // String：当你需要拥有这段字符串，或者需要追加、拼接、修改时常用
    // ps：如果你运行了当前代码rust的编译器还会提醒里如下：
    // warning: value assigned to `s3` is never read
    // --> 01_fundamentals/03-Variable/variable/src/main.rs:30:24
    //     |
    // 30 |     let mut s3: &str = "good";
    //     |                        ^^^^^^
    //     |
    //     = help: maybe it is overwritten before being read?
    //     = note: `#[warn(unused_assignments)]` (part of `#[warn(unused)]`) on by default

    // String和&str是可以互相转换的，但是要注意的是
    // &str 像“借来的文本”
    // String 像“自己拥有的一份文本”，
    // &str 变 String，就是复制出一份自己拥有的版本，所以会有性能损耗
    let s4: String = s3.to_string();
    let s5: &str = s4.as_str();
    println!("s4: {} s5: {}", s4, s5);
    let s6: &str = "hello你好world";
    // 下面由于中文是3+3，所以是5+3+3+5=16个字节
    println!("s6的字节是: {}", s6.len());
    // 下面是预期的输出12个字符
    println!("s6的字符长度是: {}", s6.chars().count());


    println!("-----------布尔类型-----------");
    // 下面是布尔类型的例子
    let b1: bool = true;
    let b2: bool = false;
    println!("b1: {} b2: {}", b1, b2);

    // 上面是基础的数据类型，数值型，字符串类型，还有布尔类型
    // 但是我们常用的还有很多，下面介绍一下列表，字典，元组
    // 列表

    println!("-----------列表-----------");
    // vec!是宏，用于创建动态数组，类似println!
    let v1: Vec<i32> = vec![1, 2, 3, 4];
    let v2: Vec<&str> = vec!["hello", "world"];
    let v3: Vec<String> = vec!["hello".to_string(), "world".to_string()];
    // Vec::new() 创建一个空的动态数组，类型参数是泛型，所以可以传入任何类型,通过左边的定义确定类型
    let mut v4: Vec<i32> = Vec::new();
    println!("v1: {:?}, v2: {:?}, v3: {:?}, v4: {:?}", v1, v2, v3, v4);
    // 这里如果v1不用clone的话，v1的数据就转移到v4了，
    // iter就是遍历v1的每一个元素，cloned就是克隆，这样v1就可以继续被使用
    // 在rust可以经常看到这样的操作，简单一行就干了一个函数的事情
    v4.extend(v1.iter().cloned());
    v4.push(5);
    println!("v4: {:?}", v4);
    // 先可以不管map部分，简单说iter就是遍历，map就是对每个元素进行操作，collect就是收集结果
    // 效果就是，生成一个可变的[5, 6, 7, 8, 9]的数组
    let mut v5: Vec<i32> = v1.iter().map(|x| x + 5).collect();
    println!("v5: {:?}", v5);
    // append就是追加，&mut v5就是借用v5的可变引用，这样v5就不能再被使用(空了)，因为v4和v5指向的是同一个内存地址
    v4.append(&mut v5);
    println!("v4: {:?}, v5: {:?}", v4, v5);
    // 看数组长度
    println!("v4的长度是: {}", v4.len());
    // pop就是弹出最后一个元素
    v4.pop();
    println!("v4: {:?}", v4);
    // 倒序
    v4.reverse();
    println!("v4: {:?}", v4);
    // 获取前3个，例如py的[0:3]
    // 下面这样写，iter产出的是&i32，类型不对
    // let v4: Vec<i32> = v4.iter().take(3).collect();
    // 如果你要得到真正的 Vec<i32>，要把引用里的值拷出来：
    let mut v4: Vec<i32> = v4.iter().take(3).copied().collect();
    // 排序
    v4.sort();
    println!("v4: {:?}", v4);
    // 循环列表
    for i in v4.iter() {
        println!("v4的元素是: {}", i);
    }
    // 循环列表带index
    for (index, value) in v4.iter().enumerate() {
        println!("v4的元素是: {} 索引是: {}", value, index);
    }


    println!("-----------字典-----------");
    // 字典，需要引入use std::collections::HashMap;
    // 这里之前用过py，go的就会很奇怪，map难道不是核心类型吗，为啥没放到默认的std::prelude里面（在这里面的结构可以直接用，例如Vec）
    // Rust 没把 HashMap 放进 prelude，主要是因为 prelude 只放最通用、最基础的东西；
    // map 属于常用集合，但不是每个模块都一定需要，而且还涉及 HashMap / BTreeMap 等具体选择。
    let mut d1: HashMap<&str, i32> = HashMap::new();
    d1.insert("one", 1);
    d1.insert("two", 2);
    d1.insert("three", 3);
    println!("d1: {:?}", d1);
    // 是否包含key
    if d1.contains_key("one") {
        // Option<&i32> 表示可能包含也可能不包含i32的值
        let value: Option<&i32> = d1.get("one");
        match value {
            Some(value) => println!("d1包含 {}", value),
            None => println!("d1不包含key one"),
        }
        // 上面这种其实比较麻烦，因为if里面已经确保了包含了，所以取值肯定是ok的，所以用下面这种更简约
        let value: &i32 = d1.get("one").unwrap();
        println!("d1包含 {}", value);
        // 也可以例如py，go直接取值，没有就panic
        println!("d1 key:one value:{}", d1["one"]);
    } else {
        println!("d1不包含key one");
    }
    d1.insert("one", 10);
    println!("d1: {:?}", d1);
    // 删除key
    d1.remove("one");
    println!("d1: {:?}", d1);
    // 一共多少个key
    println!("d1一共多少个key: {}", d1.len());
    // 循环map
    for (key, value) in d1.iter() {
        println!("d1的key是: {} 值是: {}", key, value);
    }
    // 清空
    d1.clear();
    println!("d1: {:?}", d1);


    println!("-----------元组-----------");
    // 元组
    let t1: (i32, String, bool, &str) = (1, "hello".to_string(), true, "world");
    println!("t1: {:?} t1的第一个元素是: {}", t1, t1.0);
    println!("t1的第二个元素是: {}", t1.1);
    // 下面的.4编辑器会报错，因为元组只有4个元素
    // no field `4` on type `(i32, String, bool, &str)`
    // available fields are: `0`, `1`, `2`, `3`
    // println!("t1的第三个元素是: {}", t1.4);
}
