fn main() {
    println!("Hello, world!");
}
// 项目使用cargo new命令生成，会初始化一个新的git仓库
// src文件夹里放着项目源代码
// TOML(Tom's Obvious,Minimal Language)格式，是Cargo的配置格式
/*
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2024"

[dependencies] 
*/
//上面是toml文件的内容
// [package]是一个区域标题,表示下方内容是配置包(package)的
// name 项目名
// version 项目版本
// authors 项目作者
// edition 使用的rust版本
// [dependencies],另一个区域的开始,会列出项目依赖项

// rust里面,代码的包被称为 crate
// 顶层目录可以放置 README,许可文件等与程序源码无关的文件

// 如果创建项目时没有使用cargo，也可以把项目转化为使用cargo
// 只需要把源代码放入src文件夹，再创建并填写toml配置文件

// cargo build 可以创建可执行文件,路径为 target/debug/.
// 第一次 cargo build 会在顶层目录生成 lock 文件,负责追踪项目依赖的版本，无需也不要手动修改
// cargo run 命令可以构建并运行cargo项目,如果之前编译过且源代码未更改，则直接运行二进制文件
// cargo check 检查代码，确保通过编译，不生成可执行文件，且比build快得多
// cargo build --release 为发布构建，编译时会进行优化，代码执行会变快，编译时间也会更长
// 生成的文件会放在 target/release/. 路径