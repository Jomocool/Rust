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

## 5.primitive_types

```rust
bool
let is_evening: bool = true;

---------------------------------------------------------------------------------------------

char
let your_character: char = 'C';

---------------------------------------------------------------------------------------------

数组
a:总共有100个元素，每个都是0
let a=[0;100];

---------------------------------------------------------------------------------------------

切片
nice_slice:[2,3,4]
tips:
注意被切的数组前需要加&
1..4是左闭右开，即获取的切片是数组a中下标为1~3的元素
let a=[1,2,3,4,5];
let nice_slice=&a[1..4];

---------------------------------------------------------------------------------------------

二元组
可以把二元组定义为一个变量，也可以用一个二元组初始化两个变量
let cat=("Furry McFurson",3.5);
let (name,age)=cat;

---------------------------------------------------------------------------------------------

三元组
获取三元组各元素很简单，用对应下标即可，从-0开始
let numbers=(1,2,3);
let first=numbers.0;//1
let second=numbers.1;//2
let third=numbers.2;//3
```

## 6.vecs

```rust
用vec宏创建矢量
let v=vec![10,20,30,40];

---------------------------------------------------------------------------------------------

两种修改vec里值的方式
第一个函数的for循环是获取v中可变变量的迭代器，然后解析获取值之后乘2
第二个函数是遍历v中的值，然后将值乘2后返回，最后的collect()是将这些乘2后的值收集起来返回作为返回值返回
fn vec_loop(mut v: Vec<i32>) -> Vec<i32> {
    for element in v.iter_mut() {
        // TODO: Fill this up so that each element in the Vec `v` is
        // multiplied by 2.
        *element=*element*2;
    }

    // At this point, `v` should be equal to [4, 8, 12, 16, 20].
    v
}

fn vec_map(v: &Vec<i32>) -> Vec<i32> {
    v.iter().map(|element| {
        // TODO: Do the same thing as above - but instead of mutating the
        // Vec, you can just return the new number!
        element*2
    }).collect()
}
```

## 7.move_semantics

```rust
可变权
如果不是mut vec，则不能push，因为改变了vec的内容

---------------------------------------------------------------------------------------------

所有权
由于调用fill_vec函数把vec0当作参数传了进去，所以vec0的所有权转移给了fill_vec，而main函数后续无法再用vec0，因此报错了，而函数调用完之后，vec0就消失了
解决方法，传入vec0.clone()即vec0的拷贝即可
fn main() {
    let vec0 = Vec::new();

    // Do not move the following line!
    let mut vec1 = fill_vec(vec0);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}

---------------------------------------------------------------------------------------------

将传入的参数转为mut，才可修改内容
fn main() {
    let vec0 = Vec::new();

    let mut vec1 = fill_vec(vec0);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> {
    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}

---------------------------------------------------------------------------------------------

初始化vec大小为3。避免造成空间浪费
let mut vec = Vec::with_capacity(3);

---------------------------------------------------------------------------------------------

不变引用&x可以同时被多个变量引用，但是可变引用&mut x只能被一个变量引用
let mut x=100;
let y=&mut x;
//处于此处时，x被y引用
let z=&mut x;
//处于此处时，x被z引用，同时x不被y引用，因为可变引用只能同时被一个变量引用，意味着在这里不能再使用y了

---------------------------------------------------------------------------------------------

调用函数时，传入参数类型是引用时可以避免被remove
想让调用函数获取参数所有权时，就传入原值；不想被调用函数获取参数所有权时，就传入引用
```

## 8.structs

```rust
普通struct定义和其声明
struct ColorClassicStruct {
    // TODO: Something goes here
    red: u8,
    green: u8,
    blue: u8
}
let green = ColorClassicStruct{
    red: 0,
    green: 255,
    blue: 0,
};

---------------------------------------------------------------------------------------------

tuple类型struct定义和声明
struct ColorTupleStruct(u8,u8,u8);
let green = ColorTupleStruct(0,255,0);

---------------------------------------------------------------------------------------------

unit-like struct定义和声明
struct UnitLikeStruct;
let unit_like_struct = UnitLikeStruct;

---------------------------------------------------------------------------------------------

除了name和count，其它照搬order_template
..order_template代表复用order_template的其他成员变量
let your_order = Order{
    name:"Hacker in Rust".to_string(),
    count: 1,
    ..order_template
};

---------------------------------------------------------------------------------------------

定义struct内的方法
struct Package {
    sender_country: String,
    recipient_country: String,
    weight_in_grams: i32,
}
impl Package {
    fn new(sender_country: String, recipient_country: String, weight_in_grams: i32) -> Package {
        if weight_in_grams <= 0 {
            panic!("Can not ship a weightless package.")
        } else {
            Package {
                sender_country,
                recipient_country,
                weight_in_grams,
            }
        }
    }

    fn is_international(&self) -> bool {
        self.sender_country!=self.recipient_country
    }

    fn get_fees(&self, cents_per_gram: i32) -> i32 {
        self.weight_in_grams*cents_per_gram
    }
}
```

