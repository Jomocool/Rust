//use:引用其他的库
use std::io;//prelude

fn main() {
    println!("猜数！");
    println!("猜测一个数");

    /*
    let mut foo = 1;//mutable
    let bar = foo;//immutable
     */

    let mut guess=String::new();

    //引用了io库
    //read_line(&mut guess):将值输入到guess中。传入可变引用可以在两个地方访问同一块数据，并且能够修改值
    //expect():在无法读取时，打印错误信息
    //read_line()返回值是io::Result，是一个枚举类，有两个实体Ok和Err
    //如果读取成功，返回Ok
    //如果读取失败，返回Err并有附带信息
    io::stdin().read_line(&mut guess).expect("无法读取行");

    //{}是一个占位符，在输出时会变成后面变量的值，如果有多个{}，则按顺序对应变量值
    println!("你猜测的数是：{}",guess);
}
