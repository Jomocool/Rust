//use:引用其他的库
use rand::Rng; //trait 接口
use std::cmp::Ordering;
use std::io; //prelude //比较大小

fn main() {
    println!("猜数！");

    //生成一个[1,101)之间的随机数
    let secret_number = rand::thread_rng().gen_range(1, 101);

    //循环
    loop {
        println!("猜测一个数");
        /*
        let mut foo = 1;//mutable
        let bar = foo;//immutable
         */
        let mut guess = String::new();

        //引用了io库
        //read_line(&mut guess):将值输入到guess中。传入可变引用可以在两个地方访问同一块数据，并且能够修改值
        //expect():在无法读取时，打印错误信息
        //read_line()返回值是io::Result，是一个枚举类，有两个实体Ok和Err
        //如果读取成功，返回Ok
        //如果读取失败，返回Err并有附带信息
        io::stdin().read_line(&mut guess).expect("无法读取行");

        //如果成功，parse()返回Ok，成功赋值；如果不成功parse()返回Err，程序会崩溃，然后程序按expect里的信息报错
        //这样做程序不够健壮
        //let guess: u32 = guess.trim().parse().expect("Please type a number!");

        //返回Ok(num)就把num赋值给guess;返回任意Err(_)，就从循环开始的地方重新执行
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        //shadow隐藏机制，rust允许用同名，从当前行开始，遇到的guess都是指新的guess
        //{}是一个占位符，在输出时会变成后面变量的值，如果有多个{}，则按顺序对应变量值
        println!("你猜测的数是：{}", guess);

        //让guess和secret_number比较，然后和下面的三种情况去匹配，注意各arm之间使用,区分
        match guess.cmp(&secret_number) {
            //由于guess是u32类型，因为要比较，所以secret_number也会被编译器识别为u32
            Ordering::Less => println!("Too small!"), //arm
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {//猜对，打印信息并退出循环
                println!("You win!");
                break;
            }
        }
    }
}
