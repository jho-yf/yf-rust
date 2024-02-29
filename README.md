# yf-rust

## 基本使用

### 编译器rustc

```shell
# 查看版本
rustc --version

# 编译生成二进制文件
rustc -o target_filename file.rs

# 编译生成库文件
rustc --crate-type lib file.rs
```

### 包管理工具Cargo

隐式使用`rustc`进行编译

```shell
# 创建项目
cargo new project_name
# 创建rust库项目
cargo new --lib lib_name

# 构建项目（生成二进制可执行文件或库文件）
cargo build
# 生成优化的可执行文件，常用于生成环境
cargo build --release

# 检测
cargo check

# 运行
cargo run

# 测试
cargo test
```

#### cargo项目目录结构

库

- project_name/
  - Cargo.toml
  - src/
    - lib.rs

可执行文件

- project_name/
  - Cargo.toml
  - src/
    - main.rs

Cargo.toml文件

```toml
# 设置项目名、版本等
[package]
name = "project_name"
version = "0.1.0"
authors = ["author_name"]

# 设置依赖
[dependencies]

# 列出在构建项目时需要的依赖项
[build-dependencies]

# 列出只在开发时需要的依赖项
[dev-dependencies]
```

### 第三方库crate

> https://crates.io/

添加依赖

```toml
[dependencies]
crate_name = "0.1.0"
```

#### 使用cargo-edit安装依赖库

```shell
# 安装cargo-edit
cargo install cargo-edit

# 添加依赖库
cargo add dependency_name
# 添加指定版本依赖库
cargo add dependency_name@0.1.0
# 添加开发时用的依赖库
cargo add --dev dependency_name
# 添加构建时用的依赖库
cargo add --build dependency_name

# 删除库
cargo rm dependency_name
```

#### 设置国内源

> https://rsproxy.cn/

设置`~/.cargo/config`

