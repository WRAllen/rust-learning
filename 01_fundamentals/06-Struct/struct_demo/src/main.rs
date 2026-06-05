// struct 定义数据结构，相当于把多个字段打包成一个类型
// rust 没有 class，用 struct + impl 代替 py/java 的 class、go 的 struct
struct User {
    name: String,
    age: i32,
    active: bool,
}

// impl 给 struct 加方法，写在 struct 外面、和 main 平级
impl User {
    // 关联函数：没有 self，类似静态方法 / 构造函数，用 User::new() 调用
    fn new(name: String, age: i32) -> User {
        User {
            name,
            age,
            active: true,
        }
    }

    // &self 表示不可变借用，只读，不能修改字段
    fn greet(&self) {
        println!("hello, {}", self.name);
    }

    fn describe(&self) -> String {
        format!("name: {}, age: {}, active: {}", self.name, self.age, self.active)
    }

    // &mut self 表示可变借用，可以修改自己的字段
    fn birthday(&mut self) {
        self.age += 1;
    }

    fn deactivate(&mut self) {
        self.active = false;
    }
}

// 元组结构体：字段没有名字，靠位置访问，了解即可
struct Point(i32, i32);

// 单元结构体：没有字段，常用来标记类型，了解即可
struct Marker;

fn main() {
    println!("-----------定义和创建实例-----------");
    // 创建实例，字段要逐个赋值
    let user = User {
        name: String::from("Allen"),
        age: 25,
        active: true,
    };
    println!("{}", user.describe());

    println!("-----------字段简写-----------");
    // 变量名和字段名一样时，可以简写
    let name = String::from("Bob");
    let age = 30;
    let user2 = User {
        name,
        age,
        active: true,
    };
    println!("{}", user2.describe());

    println!("-----------访问字段-----------");
    // 和 go/python 对象属性一样，用 . 访问
    println!("user.name = {}, user.age = {}", user.name, user.age);

    println!("-----------修改字段需要 mut-----------");
    // 默认不可变，改字段要 let mut
    let mut user3 = User::new(String::from("Carol"), 28);
    println!("修改前: {}", user3.describe());
    user3.birthday();
    user3.deactivate();
    println!("修改后: {}", user3.describe());

    println!("-----------impl 方法-----------");
    user.greet();

    println!("-----------关联函数 new-----------");
    let user4 = User::new(String::from("Dave"), 22);
    user4.greet();
    println!("{}", user4.describe());

    println!("-----------元组结构体-----------");
    let p = Point(3, 4);
    // 元组结构体用 .0 .1 访问，没有字段名
    println!("Point: ({}, {})", p.0, p.1);

    println!("-----------单元结构体-----------");
    // 没有字段，创建一个标记类型
    let _marker = Marker;
    println!("Marker 创建成功，一般用作类型标记");
}
