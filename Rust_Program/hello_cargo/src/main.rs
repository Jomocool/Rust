//创建cargo项目：cargo new 项目名

//在Rust中，代码的包(库)称作crate

//构建cargo项目：cargo build
//构建并运行cargo项目：cargo run，编译代码过程+执行结果
//如果之前已经编译过且源码没变，则直接运行二进制文件

//cargo check:检查代码，确保能够通过编译，但是不产生任何可执行文件，比cargo build快很多

//cargo build --release:为发布构建，编译时会进行优化，代码会运行的更快，但是编译时间更长

fn main() {
    println!("Hello, world!");
}
