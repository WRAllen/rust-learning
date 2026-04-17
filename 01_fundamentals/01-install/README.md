# 安装rust相关
直接看官网的安装方式：https://rust-lang.org/tools/install/

# 使用rustup

就是下面这条命令，一键安装
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

运行`rustup -V`查看版本
```
-> % rustup -V
rustup 1.28.2 (e4f3ad6f8 2025-04-28)
info: This is the version for the rustup toolchain manager, not the rustc compiler.
info: The currently active `rustc` version is `rustc 1.93.0 (254b59607 2026-01-19)`
```

他会安装rust相关的工具到`~/.cargo/bin`这个路径下，包括 rustc、cargo 和 rustup

*下面我使用一段时间后的目录*
```
-> % ls ~/.cargo/bin
cargo          cargo-miri     just           rust-analyzer  rust-lldb      rustfmt
cargo-clippy   cargo-zigbuild openapi-nexus  rust-gdb       rustc          rustup
cargo-fmt      clippy-driver  rls            rust-gdbgui    rustdoc
```

所以如果你要能直接运行，可以把这个路径加入PATH（说是会自动配置，但是我忘记了）

最重要的就是rustup，cargo，rustc这几个，具体这几个干嘛的呢？

rustup = 管理工具链（版本管理器）
cargo  = 项目构建工具（类似 go mod / npm）
rustc  = 编译器（真正把代码变成二进制，把rs文件变成binary）


看一下我本地的版本,可以发现rustc和cargo的版本是一致的
```
wangyu2@60305278M [23:00:52] [~/github/rust-learning] [main *]
-> % rustc --version
rustc 1.93.0 (254b59607 2026-01-19)
wangyu2@60305278M [23:02:00] [~/github/rust-learning] [main *]
-> % cargo --version
cargo 1.93.0 (083ac5135 2025-12-15)
```

但是实际使用过，我们只会用cargo，在02-helloworld会介绍


# 卸载rustup
```
rustup self uninstall
``` 

