# 前言
在前面的学习中，我们已经在本地安装好了rust开发相关的工具了，

接下来我们要开始写经典的Hello World

# 开始使用cargo构建项目
rust贴心的为我们准备了一个项目构建的神器**cargo**，前面我们已经知道他被安装在了`~/.cargo/bin`

如果你运行cargo提示命令不存在，请检查对应的路径，以及确保这个路径在PATH里面
```
wangyu2@60305278M [23:02:04] [~/github/rust-learning] [main *]
-> % which cargo
/Users/wangyu2/.cargo/bin/cargo
```

## 创建项目
我们可以直接使用cargo new hello_world (rust的项目官方推荐使用snake_case（下划线）的方式)

我们直接在当前项目里面创建，运行命令`cargo new hello_world`

输出如下
```
wangyu2@60305278M [23:09:05] [~/github/rust-learning/01_fundamentals/02-HelloWorld] [main *]
-> % cargo new hello_world
    Creating binary (application) `hello_world` package
note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
```

他会在当前路径下创建一个最mini的rust项目，这个项目就是打印出hello world（和我给他取的项目名称没有关系）

## 运行项目

进入到hello_world子目录，然后直接`cargo run`就能看到输出了
```
wangyu2@60305278M [14:48:20] [~/github/rust-learning/01_fundamentals/02-HelloWorld] [main *]
-> % cd hello_world 
wangyu2@60305278M [14:48:28] [~/github/rust-learning/01_fundamentals/02-HelloWorld/hello_world] [main *]
-> % cargo run
   Compiling hello_world v0.1.0 (/Users/wangyu2/github/rust-learning/01_fundamentals/02-HelloWorld/hello_world)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.01s
     Running `target/debug/hello_world`
Hello, world!
```

他其实先帮你运行了build命令，然后把可执行文件生成到`target/debug/hello_world`，这点和golang一致，

go运行项目也是`go run main.go`也是先去build然后在执行build后的binary

## 编译项目

那上面也提到了build相关的了，我们可以运行`cargo build`(运行前先把之前run生成的target文件夹删了)
```
wangyu2@60305278M [14:51:03] [~/github/rust-learning/01_fundamentals/02-HelloWorld/hello_world] [main *]
-> % rm -rf target 
wangyu2@60305278M [14:51:10] [~/github/rust-learning/01_fundamentals/02-HelloWorld/hello_world] [main *]
-> % cargo build  
   Compiling hello_world v0.1.0 (/Users/wangyu2/github/rust-learning/01_fundamentals/02-HelloWorld/hello_world)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.15s
```
可以看到，也是一样的生成了一个binary在
```
wangyu2@60305278M [14:51:13] [~/github/rust-learning/01_fundamentals/02-HelloWorld/hello_world] [main *]
-> % ls target/debug/hello_world 
target/debug/hello_world
```

这里你会发现，他的路径在target下的debug，cargo默认的build是编译最快的，但是运行时慢，可以加上--release
```
wangyu2@60305278M [14:57:01] [~/github/rust-learning/01_fundamentals/02-HelloWorld/hello_world] [main *]
-> % rm -rf target 
wangyu2@60305278M [14:57:04] [~/github/rust-learning/01_fundamentals/02-HelloWorld/hello_world] [main *]
-> % cargo build --release
   Compiling hello_world v0.1.0 (/Users/wangyu2/github/rust-learning/01_fundamentals/02-HelloWorld/hello_world)
    Finished `release` profile [optimized] target(s) in 0.53s
```
可以看到，他生成了一个release的子目录
```
wangyu2@60305278M [14:57:06] [~/github/rust-learning/01_fundamentals/02-HelloWorld/hello_world] [main *]
-> % ls target/release/hello_world 
target/release/hello_world
```

可以看到rust的编译使用起来和go一样，还是非常简单的

## 交叉编译

类似go，默认build是根据你当前运行的系统来决定的，但是我们也可以交叉编译（比如在mac上编译linux，或者在linux上编译mac）


### 添加目标平台
通过下面命令，可以支持在任意平台编译出linux的binary
```
rustup target add x86_64-unknown-linux-gnu
```
运行输出
```
wangyu2@60305278M [14:57:37] [~/github/rust-learning/01_fundamentals/02-HelloWorld/hello_world] [main *]
-> % rustup target add x86_64-unknown-linux-gnu
info: downloading component 'rust-std' for 'x86_64-unknown-linux-gnu'
info: installing component 'rust-std' for 'x86_64-unknown-linux-gnu'
 28.2 MiB /  28.2 MiB (100 %)  25.6 MiB/s in  1s  
```

### 开始编译
运行如下命令
```
cargo build --release --target x86_64-unknown-linux-gnu
```
运行输出
```
wangyu2@60305278M [15:01:03] [~/github/rust-learning/01_fundamentals/02-HelloWorld/hello_world] [main *]
-> % cargo build --release --target x86_64-unknown-linux-gnu
   Compiling hello_world v0.1.0 (/Users/wangyu2/github/rust-learning/01_fundamentals/02-HelloWorld/hello_world)
error: linking with `cc` failed: exit status: 1
  |
  = note:  "cc" "-m64" "/var/folders/pb/kj1ljltd3rnfghcq1y4n_b9msf_46f/T/rustcAbdFIX/symbols.o" "<2 object files omitted>" "-Wl,--as-needed" "-Wl,-Bstatic" "<sysroot>/lib/rustlib/x86_64-unknown-linux-gnu/lib/{libstd-*,libpanic_unwind-*,libobject-*,libmemchr-*,libaddr2line-*,libgimli-*,libcfg_if-*,librustc_demangle-*,libstd_detect-*,libhashbrown-*,librustc_std_workspace_alloc-*,libminiz_oxide-*,libadler2-*,libunwind-*,liblibc-*,librustc_std_workspace_core-*,liballoc-*,libcore-*,libcompiler_builtins-*}.rlib" "-Wl,-Bdynamic" "-lgcc_s" "-lutil" "-lrt" "-lpthread" "-lm" "-ldl" "-lc" "-L" "/var/folders/pb/kj1ljltd3rnfghcq1y4n_b9msf_46f/T/rustcAbdFIX/raw-dylibs" "-Wl,--eh-frame-hdr" "-Wl,-z,noexecstack" "-L" "<sysroot>/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-o" "/Users/wangyu2/github/rust-learning/01_fundamentals/02-HelloWorld/hello_world/target/x86_64-unknown-linux-gnu/release/deps/hello_world-8481652fc3f34512" "-Wl,--gc-sections" "-pie" "-Wl,-z,relro,-z,now" "-Wl,-O1" "-Wl,--strip-debug" "-nodefaultlibs"
  = note: some arguments are omitted. use `--verbose` to show all linker arguments
  = note: clang: warning: argument unused during compilation: '-pie' [-Wunused-command-line-argument]
          ld: unknown options: --as-needed -Bstatic -Bdynamic --eh-frame-hdr -z --gc-sections -z -z --strip-debug 
          clang: error: linker command failed with exit code 1 (use -v to see invocation)
          

error: could not compile `hello_world` (bin "hello_world") due to 1 previous error
```
诶呀，发现报错了，原因是Mac 的 linker不认识linux的参数，这点就没go方便，ai推荐使用cross，但是由于cross本质上使用docker

拉取对应的linux容器来build的，这里由于我是公司电脑，不能安装docker，这里就不拓展了

一般实际中，我们都是在linux上编译，然后在linux上跑的

*****
以上就是基础的rust编译相关的内容，后续如果有，也会继续添加在这里
