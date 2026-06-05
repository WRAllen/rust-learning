# 前言

学完了变量、函数、流程控制，接下来要学习 Rust 里相当于 Python/Java `class`、Go `struct` 的东西——**struct（结构体）**。

Rust **没有 class**，也没有传统面向对象里的继承。它用 `struct` 定义数据结构，用 `impl` 给结构体加方法，这套组合就能完成大部分"把数据和行为绑在一起"的需求。

# struct 是什么

可以把 struct 理解成：把多个相关字段打包成一个类型。

```rust
struct User {
    name: String,
    age: i32,
    active: bool,
}
```

和常见语言对比：

| 语言   | 写法                     | 继承     |
| ---- | ---------------------- | ------ |
| Python | `class User: ...`      | 有      |
| Java   | `class User { ... }`   | 有      |
| Go     | `type User struct {...}` | 无      |
| Rust   | `struct User { ... }`  | 无，用 trait 扩展 |

初学者先记住：**Rust 用 struct 存数据，用 impl 写方法**。

## 定义和创建实例

```rust
struct User {
    name: String,
    age: i32,
}

let user = User {
    name: String::from("Allen"),
    age: 25,
};
```

字段名和变量名一样，用 snake_case。

## 访问字段

和 Go、Python 对象属性类似，用 `.` 访问：

```rust
println!("{}", user.name);
```

## 修改字段需要 `mut`

struct 实例默认不可变，要改字段得加 `mut`：

```rust
let mut user = User { ... };
user.age += 1;
```

这点和 `let` / `let mut` 的规则一致。

# impl：给 struct 加方法

方法写在 `impl` 块里，第一个参数通常是 `&self`、`&mut self` 或 `self`：

```rust
impl User {
    fn greet(&self) {
        println!("hello, {}", self.name);
    }
}
```

调用方式和大多数语言一样：

```rust
user.greet();
```

| 第一个参数   | 含义           | 类比           |
| -------- | ------------ | ------------ |
| `&self`  | 不可变借用，只读     | 只读方法         |
| `&mut self` | 可变借用，可修改字段  | 会改对象状态的方法    |
| `self`   | 获取所有权，消费掉实例  | 少见，入门先了解即可   |

## 关联函数（类似静态方法）

`impl` 里没有 `self` 的函数叫**关联函数**，用 `::` 调用，常用来当构造函数：

```rust
impl User {
    fn new(name: String, age: i32) -> User {
        User { name, age }
    }
}

let user = User::new(String::from("Allen"), 25);
```

这和 Python 的 `@staticmethod`、Java 的 `static` 方法有点像。

# 字段简写

如果变量名和字段名一样，可以简写：

```rust
let name = String::from("Allen");
let age = 25;
let user = User { name, age };
```

# 其他 struct 形式（了解即可）

## 元组结构体

字段没有名字，靠位置访问：

```rust
struct Point(i32, i32);
let p = Point(3, 4);
println!("{}", p.0);
```

## 单元结构体

没有字段，常用来标记类型：

```rust
struct Marker;
```

入门阶段知道有这两种就行，日常开发用得最多的是普通 struct。

# 小结

对于初学者，struct 这块先记住这些：

1. Rust 没有 class，用 `struct` + `impl`
2. `struct` 定义字段，`impl` 定义方法
3. 访问字段用 `.`，修改字段需要 `mut`
4. `&self` 是只读方法，`&mut self` 是可修改字段的方法
5. 没有 `self` 的函数是关联函数，用 `StructName::func()` 调用

下面我在 README.md 里面就不多介绍了，直接在[代码里面](./struct_demo/src/main.rs)用注释的方式来说明。
