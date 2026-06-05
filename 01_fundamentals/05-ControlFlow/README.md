# 前言

现在我们知道了怎么定义和使用变量、函数，但还有个关键的基础要说一下，那就是**流程控制**。

在 03 里我们已经零散用过 `for`、`if`、`match`，这一章把它们系统讲清楚。流程控制和 Python、Go 大体类似，但 Rust 有几个自己的特点值得注意。

# if / else

和 Python、Go 一样，用 `if` 做条件判断：

```rust
let n = 5;
if n > 0 {
    println!("正数");
} else if n == 0 {
    println!("零");
} else {
    println!("负数");
}
```

注意：Rust 的 `if` 条件必须是 **bool**，不能像 Python 那样写 `if 1` 或 `if "hello"`。

| 语言   | 条件类型要求        |
| ---- | ------------- |
| Python | 任意值，靠真假性判断    |
| Go     | 必须是 bool       |
| Rust   | 必须是 bool，和 Go 一样 |

## if 也是表达式

Rust 没有三元运算符 `a ? b : c`，但 `if` 本身可以是表达式，用来赋值：

```rust
let label = if n > 0 { "正数" } else { "非正数" };
```

`if` 和 `else` 两边的返回值类型必须一致。

# 循环

Rust 有三种循环：`loop`、`while`、`for`。

## loop

无限循环，靠 `break` 退出。和 Go 的 `for {}` 有点像：

```rust
let mut count = 0;
let result = loop {
    count += 1;
    if count == 3 {
        break count;
    }
};
```

`break` 还可以带返回值，上面 `result` 就是 `3`。

## while

条件为真时继续循环，和 Python、Go 的 `while` 一样：

```rust
let mut n = 3;
while n > 0 {
    n -= 1;
}
```

## for

Rust 里最常用的循环方式，用来遍历范围或可迭代对象：

```rust
// 遍历 0..3，左闭右开，类似 Python 的 range(0, 3)
for i in 0..3 {
    println!("{}", i);
}

// 遍历 Vec，03 里已经用过
for item in vec.iter() {
    println!("{}", item);
}
```

`0..3` 和 `0..=3` 的区别：

| 写法      | 范围           | 类比 Python      |
| ------- | ------------ | -------------- |
| `0..3`  | 0, 1, 2      | `range(0, 3)`  |
| `0..=3` | 0, 1, 2, 3   | `range(0, 4)`  |

# match

`match` 是 Rust 里非常核心的模式匹配，比 Go 的 `switch`、Python 的 `match`（3.10+）更严格。

```rust
let code = 200;
match code {
    200 => println!("成功"),
    404 => println!("未找到"),
    _ => println!("其他"),
}
```

几个要点：

1. **必须覆盖所有情况**，否则编译不过
2. `_` 是通配符，类似 Python 的 `_`、Go 的 `default`
3. `match` 也是表达式，可以返回值

03 里在 HashMap 取值的 `Option` 判断就用了 `match`：

```rust
match value {
    Some(v) => println!("{}", v),
    None => println!("不存在"),
}
```

## match guard

`match` 左边写的是**模式**，不是普通表达式，所以不能这样写：

```rust
// 不行
match n {
    x + 1 => {},
    _ => {},
}
```

如果想加额外判断，用 **guard**，在 `=>` 前面加 `if`：

```rust
let x = 2;
let n = 3;
match n {
    v if v == x + 1 => println!("等于 x + 1"),
    v if v > 10 => println!("大于 10"),
    _ => println!("其他"),
}
```

guard 也可以让 `match` 作为表达式返回值：

```rust
let grade = match score {
    s if s >= 90 => "优秀",
    s if s >= 60 => "及格",
    _ => "不及格",
};
```

# break 和 continue

和 Python、Go 一样：

- `break`：跳出循环
- `continue`：跳过本次，进入下一次

```rust
for i in 0..10 {
    if i % 2 == 0 {
        continue;
    }
    if i > 5 {
        break;
    }
    println!("{}", i);
}
```

# 小结

对于初学者，流程控制先记住这些：

1. `if` 条件必须是 bool，`if` 也可以当表达式用
2. 三种循环：`loop`（无限）、`while`（条件）、`for`（遍历，最常用）
3. `0..n` 左闭右开，`0..=n` 包含 n
4. `match` 必须穷尽所有分支，`_` 是兜底
5. `match` 左边是模式，复杂条件用 `v if 条件 =>`（guard）
6. `break` / `continue` 和其他语言一样

下面我在 README.md 里面就不多介绍了，直接在[代码里面](./control_flow/src/main.rs)用注释的方式来说明。
