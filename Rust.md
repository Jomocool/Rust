## Rust

## 1.intro

```rust
!代表宏
{}代表后面参数的打印出来后的所在位置
println!("Hello {}!","World");
```

## 2.variables

```rust
赋值，且第一次定义必须赋初始值
:定义变量的类型，一般编译器可以自己推导
mut使变量x的值可以被修改，因为rust的变量默认不可被修改immutable
let mut x: i32 = 5;
x = "T-H-R-E-E";这是错误的，因为rust是强定义的，不能赋不同类型的值给变量

---------------------------------------------------------------------------------------------

const定义变量必须给出变量类型和初始值
const NUMBER: i32 = 3;
```

## 3.functions

```rust
定义空函数
fn call_me(){
    
}

---------------------------------------------------------------------------------------------

定义带参数的函数
调用带参函数必须传入相应类型的参数
fn call_me(num: i32){
    for i in 0..num{
        println!("Ring! Call number {}",i+1);
    }
}

---------------------------------------------------------------------------------------------

定义带参并且有返回值的函数
tips:
某行语句没有;说明该行是返回值，且必须是“最后一行”
函数参数列表后的 -> 用于指定返回值类型
fn sale_price(price: i32) -> i32{
    if(is_even(price)){
        price-10
    }else{
        price-3
    }
}
fn is_even(num: i32) -> bool{
    num%2==0
}
```

## 4.if

```rust
if conditon1{
    //do somethings
}else if condition2{
    //do somethings
}else{
    //do sonmethings
}
```

