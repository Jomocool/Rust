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

char(4字节)
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

## 9.enums

```rust
枚举的定义，注意和struct的定义类似，最后一个变量最后面也需要,
enum Message {
    Quit,
    Echo,
    Move,
    ChangeColor,
}

---------------------------------------------------------------------------------------------

变量有状态的枚举的定义和使用
tips:看下面怎么使用的就怎么定义，比如Move使用{}，那么定义时也用{}，如果是用()，定义时也要用()
enum Message {
    Move{x:i32,y:i32},
    Echo(String),
    ChangeColor(u8,u8,u8),
    Quit,
}
let messages = [
    Message::Move { x: 10, y: 30 },
    Message::Echo(String::from("hello world")),
    Message::ChangeColor(200, 255, 255),
    Message::Quit,
];

---------------------------------------------------------------------------------------------

匹配枚举内的变量，match enum_variables{}，然后用=>表示对应的处理方式
enum Message {
    // TODO: implement the message variant types based on their usage below
    ChangeColor(u8,u8,u8),
    Echo(String),
    Move(Point),
    Quit,
}

state.process(Message::ChangeColor(255, 0, 255));
state.process(Message::Echo(String::from("hello world")));
state.process(Message::Move(Point { x: 10, y: 15 }));
state.process(Message::Quit);

struct Point {
    x: u8,
    y: u8,
}

struct State {
    color: (u8, u8, u8),
    position: Point,
    quit: bool,
}

impl State {
    fn change_color(&mut self, color: (u8, u8, u8)) {
        self.color = color;
    }

    fn quit(&mut self) {
        self.quit = true;
    }

    fn echo(&self, s: String) {
        println!("{}", s);
    }

    fn move_position(&mut self, p: Point) {
        self.position = p;
    }

    fn process(&mut self, message: Message) {
        // TODO: create a match expression to process the different message variants
        // Remember: When passing a tuple as a function argument, you'll need extra parentheses: fn function((t, u, p, l, e))
        match message{
            Message::ChangeColor(red,green,blue) => self.change_color((red,green,blue)),
            Message::Echo(s) => self.echo(s),
            Message::Move(point) => self.move_position(point),
            Message::Quit => self.quit(),
        }
    }
}
```

## 10.strings

```rust
String:具有所有权
&str:字符串切片或静态字符串的引用，占两个字节，第一个字节用于记录起始位置，第二个字节用于记录切片长度。相当于一个视图，只可读不可更改

通过调用&str.to_string()或者String::from(&str)可以让&str转成String

let s:&str="blue";

---------------------------------------------------------------------------------------------

&String可以自动解引用为&str

---------------------------------------------------------------------------------------------

修改字符串
fn trim_me(input: &str) -> String {
    // TODO: Remove whitespace from both ends of a string!
    input.trim().to_string()
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There's multiple ways to do this!
    input.to_string()+" world!"
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons"!
    input.replace("cars","balloons")
}

---------------------------------------------------------------------------------------------

需要开辟新的空间就需要String，而如果不用开辟新的空间，则用&str即可
fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string_slice("blue");
    string("red".to_string());
    string(String::from("hi"));
    string("rust is fun!".to_owned());
    string("nice weather".into());//into()自动转换
    string(format!("Interpolation {}", "Station"));//拼接字符串
    string_slice(&String::from("abc")[0..1]);//"ab"
    string_slice("  hello there ".trim());
    string("Happy Monday!".to_string().replace("Mon", "Tues"));
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
```

## 11.modules

```rust
mod相当于一个命名空间

命名空间的成员默认是private的，要想外面能够看到并使用，就需要在对应的成员声明前面加上pub
mod sausage_factory {
    // Don't let anybody outside of this module see this!
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    pub fn make_sausage() {
        get_secret_recipe();
        println!("sausage!");
    }
}

---------------------------------------------------------------------------------------------

嵌套mod，但是将PEAR、CUCUMBER分别重命名为fruit、veggie，且前面加上pub，这样就可以在外面这样使用：
delicious_snacks::fruit;
delicious_snacks::veggie;
mod delicious_snacks {
    // TODO: Fix these use statements
    pub use self::fruits::PEAR as fruit;
    pub use self::veggies::CUCUMBER as veggie;

    mod fruits {
        pub const PEAR: &'static str = "Pear";
        pub const APPLE: &'static str = "Apple";
    }

    mod veggies {
        pub const CUCUMBER: &'static str = "Cucumber";
        pub const CARROT: &'static str = "Carrot";
    }
}

---------------------------------------------------------------------------------------------

使用系统mod，引入了std::time::SystemTime和std::time::UNIX_EPOCH
use std::time::{SystemTime,UNIX_EPOCH};

fn main() {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => println!("1970-01-01 00:00:00 UTC was {} seconds ago!", n.as_secs()),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}
```

## 12.hashmaps

```rust
声明一个哈希表并插入数据，会根据后面插入的数据来定义类型
let	mut hash_map = HashMap::new();
let mut basket = HashMap::with_capacity(3);//至少能够插入3个元素
basket.insert(String::from("banana"), 2);
basket.insert(String::from("apple"),3);
basket.insert(String::from("mango"),4);

获取哈希表的大小
basket.len()

获取哈希表value值的总和
basket.values().sum();

---------------------------------------------------------------------------------------------

插入数据但不破坏原先的数据
basket.entry(fruit).or_insert(1)//在basket没有fruit的情况下，才插入这个新元素(fruit,1)
basket.entry(fruit).or_insert_with(function())//如果插入值需要通过函数计算，就用or_insert_with

---------------------------------------------------------------------------------------------

由于传入函数之后会失去所有权，导致后面拿不到team_1_name，所以传入clone
如果有对应队伍，修改value即可
如果没有对应队伍，创建新的元素插入哈希表中
scores.entry(team_1_name.clone())
.and_modify(|team|{
    team.goals_scored+=team_1_score;
    team.goals_conceded+=team_2_score;
})
.or_insert_with(||{
    Team{name:team_1_name,goals_scored:team_1_score,goals_conceded:team_2_score,}
});

scores.entry(team_2_name.clone())
.and_modify(|team|{
    team.goals_scored+=team_2_score;
    team.goals_conceded+=team_1_score;
})
.or_insert_with(||{
    Team{name:team_2_name,goals_scored:team_2_score,goals_conceded:team_1_score,}
});
```

## 13.quiz2

```rust
mod my_module{
    pub fn transformer(){}
}
mod tests{
    //如果要想调用my_module的transformer()，要这样做
    use super::my_module::transformer//因为当前模块没有my_module，所以需要在前面加super
}
```

## 14.options

```rust
Option<type>:要么有值Some(value)，要么没有值None
fn maybe_icecream(time_of_day: u16) -> Option<u16> {
    if time_of_day<22{
        Some(5)
    }else if(time_of_day<24){
        Some(0)
    }else{
        None
    }
}
//确定有值才用unwrap()，否则不轻易使用
let icecreams = maybe_icecream(12).unwrap();

---------------------------------------------------------------------------------------------

fn simple_option() {
    let target = "rustlings";
    let optional_target = Some(target);

    // TODO: Make this an if let statement whose value is "Some" type
    if let Some(word) = optional_target {
        assert_eq!(word, target);
    }
    /*
    等价于：
    match optional_target{
    	Some(word) => {
    		assert_eq!(word,target);
    	}
    	None => {}
    }
    但是上面的写法更优雅且更易懂
    */
}

let mut optional_integers: Vec<Option<i8>> = vec![None];
optional_integets.pop()返回的是Option<T>，即Option<Option<i8>>
所以需要嵌套Some去接收，let Some(Some(integer))=optional_integets.pop();

---------------------------------------------------------------------------------------------

Some(p)会把p(也就是y对应的Point)的所有权拿走，所以后面再用到y就有问题了，解决方法是传入引用，即ref p
这里不用&的原因是，&是创建引用的运算符，而ref是用于模式匹配时绑定引用的语法，这里是想通过match y来匹配的，所以用ref
fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match y {
        Some(p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => panic!("no match!"),
    }
    y; // Fix without deleting this line.
}
```

## 15.errors

```rust
Result<T,E>:作为返回值时用于处理错误
有效时，返回Ok(T)
无效时，返回Err(E)

下面这个函数如果传进来的name是空的，说明无效，返回Err(String)；如果不为空，返回Ok(T)
pub fn generate_nametag_text(name: String) -> Result<String,String> {
    if name.is_empty() {
        // Empty names aren't allowed.
        Err("`name` was empty; it must be nonempty.".into())
    } else {
        Ok(format!("Hi! My name is {}", name))
    }
}

---------------------------------------------------------------------------------------------

E.parse::<T>()返回值类型是Result<T,E>
有效就取出让其对应值放入quantity中，然后将quantity赋值给qty；无效就把错误信息给e
pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    
    let qty = match item_quantity.parse::<i32>(){
        Ok(quantity) => quantity,
        Err(e) => return Err(e),
    };//等价于：let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}

---------------------------------------------------------------------------------------------

以下函数错误的地方在于使用了?符号，但是返回值类型却是()。返回值必须是Result或者Option来接收?符号。由上面可知，?等价于有效就取值，无效就返回一个Result，所以必须预先设置好返回值

更正方法是将返回值类型设为Result，由于不清楚具体的返回值类型，所以可以用泛型Result<(),ParseIntError>，然后在最后返回Ok(())
fn main() -> Result<(),ParseIntError>{
    let mut tokens = 100;
    let pretend_user_input = "8";

    let cost = total_cost(pretend_user_input)?;

    if cost > tokens {
        println!("You can't afford that many!");
    } else {
        tokens -= cost;
        println!("You now have {} tokens.", tokens);
    }

    Ok(())//上面都处理好后，返回Ok(())
}

---------------------------------------------------------------------------------------------

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

用x去接收value，然后根据x的值来作相应的处理
value as u64是将i64的值转为u64
_表示剩下的所有情况
impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value{
            x if x>0 => Ok(PositiveNonzeroInteger(value as u64)),
            x if x==0 => Err(CreationError::Zero),
            _ => Err(CreationError::Negative),
        }
    }
}

---------------------------------------------------------------------------------------------

以下函数有两种error类型，
一种是ParseIntError，另一种是CreationError，为了统一，就使用了Box<dyn error::Error>
fn main() -> Result<(), Box<dyn error::Error>> {
    let pretend_user_input = "42";
    let x: i64 = pretend_user_input.parse()?;
    println!("output={:?}", PositiveNonzeroInteger::new(x)?);
    Ok(())
}

// This is required so that `CreationError` can implement `error::Error`.
impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match *self {
            CreationError::Negative => "number is negative",
            CreationError::Zero => "number is zero",
        };
        f.write_str(description)
    }
}

impl error::Error for CreationError {}
```

## 16.generics

```rust
不定义类型，让编译器自己推断
let mut shopping_list = Vec::new();
shopping_list.push("milk");

---------------------------------------------------------------------------------------------

泛型struct的定义和impl
struct的类名需要加<T>
impl后也要加<T>，表示泛型；其后的类名也要加<T>对应
这样做的好处有Wrapper<&str>无法使用Wrapper<i32>的方法

struct Wrapper<T> {
    value: T,
}

impl<T> Wrapper<T> {
    pub fn new(value: T) -> Self {
        Wrapper { value }
    }
}
```

## 17.traits

```rust
接口，可以定义一些行为

定义接口，没有指定特定类型，用的是Self
trait AppendBar {
    fn append_bar(self) -> Self;
}

为某个特定类型实现接口，在这里self可看作String类型的参数，Self可看作String
impl AppendBar for String {
    fn append_bar(self) -> Self{
        format!("{}Bar",self)
    }
}

---------------------------------------------------------------------------------------------

trait AppendBar {
    fn append_bar(self) -> Self;
}

// TODO: Implement trait `AppendBar` for a vector of strings.
impl AppendBar for Vec<String>{
    fn append_bar(mut self) -> Self{
        self.push("Bar".to_string());
        self
    }
}

---------------------------------------------------------------------------------------------

trait默认实现
pub trait Licensed {
    fn licensing_info(&self) -> String{
        String::from("Some information")
    }
}

---------------------------------------------------------------------------------------------

pub trait Licensed {
    fn licensing_info(&self) -> String {
        "some information".to_string()
    }
}
struct SomeSoftware {}
struct OtherSoftware {}
impl Licensed for SomeSoftware {}
impl Licensed for OtherSoftware {}

自动匹配类型，impl Licensed可以匹配成SomeSoftware或OtherSoftware
fn compare_license_types(software: impl Licensed, software_two: impl Licensed) -> bool {
    software.licensing_info() == software_two.licensing_info()
}
等价于
fn compare_license_types<T:Licensed,U:Licensed>(software: T, software_two: U) -> bool {
    software.licensing_info() == software_two.licensing_info()
}
等价于
fn compare_license_types<T,U>(software: T, software_two: U) -> bool 
where
	T:Licensed,
	U:Licensed,
{
    software.licensing_info() == software_two.licensing_info()
}

---------------------------------------------------------------------------------------------

fn some_func<T>(item: T) -> bool 
where
    T:SomeTrait+OtherTrait//+:表示T既有SomeTrait的行为，也有OtherTrait的行为
{
    item.some_function() && item.other_function()
}
```

## 18.quiz3

```rust
use std::fmt::Display;

有了泛型之后，grade可以是多种类型的，比如数字或字符串
pub struct ReportCard<T> {
    pub grade: T,
    pub student_name: String,
    pub student_age: u8,
}

需要时Display类型才能打印
impl<T:Display> ReportCard<T> {
    pub fn print(&self) -> String {
        format!("{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, &self.grade)
    }
}
```

## 19.lifetimes

```rust
定义生命周期，主要是为了避免内存安全的问题

表示x、y和返回值的生命周期都是'a，
注意是在&运算符后添加生命周期'
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

string1的生命周期是main()函数里
string2是静态变量，生命周期是整个程序
二者的生命周期交集是在整个main()函数里，所以对应上面的'a，取交集，因此result的生命周期也持续到main结束，保证在执行println!语句是仍有效
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is '{}'", result);
}

也可以这样写，'b是'a的子集，但是'b>'a，'b的生命周期更长，二者以短的那个生命周期为准
fn longest<'a,'b:'a>(x: &'a str, y: &'b str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

---------------------------------------------------------------------------------------------

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

string1的生命周期是整个main()
string2的生命周期是声明string2的所在scope，即中间的花括号范围内
调用的函数传入的x、y要具有相同的生命周期，所以会取二者生命周期的交集，即string2的生命周期，到打印语句时result就已经失效了，因此一定会发生内存安全问题。
string2内存释放后，仍有指针指向string2，内存不安全
fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is '{}'", result);
}
更正方式：把string2拿出来即可，这样它的生命截止期就和string1一样了
fn main() {
    let string1 = String::from("long string is long");
    let string2 = String::from("xyz");
    let result;
    {
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is '{}'", result);
}

---------------------------------------------------------------------------------------------

定义struct的生命周期
struct Book<'a> {
    author: &'a str,
    title: &'a str,
}
```

## 20.tests

```rust
#[cfg(test)]//在此模块下的代码在运行cargo build时不会被编译器看到，只有在运行cargo test时才会被看到
mod tests {
    #[test]
    fn you_can_assert() {
        assert!(true);//只有当括号里是true时，才通过
    }
}

---------------------------------------------------------------------------------------------

验证结果是否符合预期要求
#[cfg(test)]
mod tests {
    #[test]
    fn you_can_assert_eq() {
        let bar = 1;
        assert_eq!(bar,1);
    }
}

---------------------------------------------------------------------------------------------

struct Rectangle {
    width: i32,
    height: i32
}

impl Rectangle {
    // Only change the test functions themselves
    pub fn new(width: i32, height: i32) -> Self {
        if width <= 0 || height <= 0 {
            panic!("Rectangle width and height cannot be negative!")
        }
        Rectangle {width, height}
    }
}

#[test]
#[should_panic]//加上这个，表示该测试应该抛出panic
fn negative_width() {
    // This test should check if program panics when we try to create rectangle with negative width
    let _rect = Rectangle::new(-10, 10);
}
```

## 21.iterators

```rust
初始化迭代器
let my_fav_fruits = vec!["banana", "custard apple", "avocado", "peach", "raspberry"];
let mut my_iterable_fav_fruits = my_fav_fruits.iter();//拿到引用而已
let mut my_iterable_fav_fruits = my_fav_fruits.into_iter(); //拿到所有权
let mut my_iterable_fav_fruits = my_fav_fruits.iter_mut();//拿到可变引用

使用迭代遍历
let mut my_iterable_fav_fruits = my_fav_fruits.iter();
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"banana"));
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"custard apple"));
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"avocado"));
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"peach"));
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"raspberry"));
    assert_eq!(my_iterable_fav_fruits.next(), None);
}

---------------------------------------------------------------------------------------------

将单词首字母改成大写
假设input="hello"：
assert_eq!(c.as_str(),"hello");
c.next();
assert_eq!(c.as_str(),"ello");
c.next();
assert_eq!(c.as_str(),"llo");
所以匹配第一个c.next()即首字母后，将其转为大写字母后直接拼接c.as_str()，此时的c.as_str()是已经少了首字母了
pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().to_string()+c.as_str(),
    }
}

pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    words.iter()
        .map(|&word|{//用&word去匹配，这样word就是&str类型了，因为&也参与匹配了
            capitalize_first(word)
        })
        .collect()
}

pub fn capitalize_words_string(words: &[&str]) -> String {
    words.iter()
        .map(|word|{
            capitalize_first(*word)//*解引用，使&&str类型变成&str类型
        })
        .collect()
}
编译器会根据不同的类型找到其collect()的实现

---------------------------------------------------------------------------------------------

#[derive(Debug, PartialEq, Eq)]
pub enum DivisionError {
    NotDivisible(NotDivisibleError),
    DivideByZero,
}

#[derive(Debug, PartialEq, Eq)]
pub struct NotDivisibleError {
    dividend: i32,
    divisor: i32,
}

// Calculate `a` divided by `b` if `a` is evenly divisible by `b`.
// Otherwise, return a suitable error.
pub fn divide(a: i32, b: i32) -> Result<i32, DivisionError> {
    return match (a,b){
        (_,0) => Err(DivisionError::DivideByZero),
        (a,b) if a%b!=0 => Err(DivisionError::NotDivisible(NotDivisibleError{dividend:a,divisor:b,})),
        _ => Ok(a/b),
    }
}

// Complete the function and return a value of the correct type so the test passes.
// Desired output: Ok([1, 11, 1426, 3])
fn result_with_list() -> Result<Vec<i32>,DivisionError> {
    let numbers = vec![27, 297, 38502, 81];
    numbers.into_iter()
        .map(|n| divide(n, 27))
        .collect()//会根据返回值类型返回一个相应类型的合集
}

// Complete the function and return a value of the correct type so the test passes.
// Desired output: [Ok(1), Ok(11), Ok(1426), Ok(3)]
fn list_of_results() -> Vec<Result<i32,DivisionError>> {
    let numbers = vec![27, 297, 38502, 81];
    numbers.into_iter()
    .map(|n| divide(n, 27))
    .collect()
}

---------------------------------------------------------------------------------------------

递归
pub fn factorial(num: u64) -> u64 {
    fn do_factorial(acc:u64,num:u64) -> u64{
        if num==0 || num==1 {
            acc
        }else{
            do_factorial(acc*num,num-1)
        }
    }

    do_factorial(1,num)
}

pub fn factorial(num: u64) -> u64 {
    fn do_factorial(acc:u64,num:u64) -> u64{
        match num {
            0 | 1 => acc,
            _ => do_factorial(acc*num,num-1),
        }
    }

    do_factorial(1,num)
}

非递归解法，本质还是递归
pub fn factorial(num: u64) -> u64 {
    (0..=num).fold(1,|acc,num|{
        match num {
            0 | 1 => acc,
            _ => acc*num,
        }
    })
}

---------------------------------------------------------------------------------------------

fn count_iterator(map: &HashMap<String, Progress>, value: Progress) -> usize {
    map.iter()
        .filter(|kv|kv.1==&value)//过滤到不符合括号里的条件的记录，然后计数
        .count()
}

从多个哈希表找到对应值的记录数
fn count_collection_iterator(collection: &[HashMap<String, Progress>], value: Progress) -> usize {
    collection.iter()
        .map(|hashmap|count_iterator(hashmap,value))
        .sum()
}
```

## 22.threads

```rust
创建了10个线程，每个线程睡眠250毫秒，然后打印自己的编号和耗时
使用JoinHandle的join方法来等待每个线程结束，并获取它们的返回值
for handle in handles {
	let result = handle.join().unwrap();
    results.push(result);
}

---------------------------------------------------------------------------------------------

unwrap函数：计算结果，如果有错误，painc并停止程序
use std::sync::{Arc,Mutex}  ;
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));//声明互斥锁
    let mut handles = vec![];
    for _ in 0..10 {
        let status_shared = status.clone();//拷贝对象
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // TODO: You must take an action before you update a shared value
            status_shared.lock().unwrap().jobs_completed += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
        // TODO: Print the value of the JobStatus.jobs_completed. Did you notice anything
        // interesting in the output? Do you have to 'join' on all the handles?
        println!("jobs completed {}", status.lock().unwrap().jobs_completed);
    }
}

---------------------------------------------------------------------------------------------

fn send_tx(q: Queue, tx: mpsc::Sender<u32>) -> () {
    let qc = Arc::new(q);
    let qc1 = Arc::clone(&qc);
    let qc2 = Arc::clone(&qc);

    let tx_cloned=tx.clone();

    thread::spawn(move || {
        for val in &qc1.first_half {
            println!("sending {:?}", val);
            tx.send(*val).unwrap();//这里move之后，下面就无法使用tx了，因此需要一个tx的clone
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        for val in &qc2.second_half {
            println!("sending {:?}", val);
            tx_cloned.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
}
```

## 23.smart_pointers

```rust
Box 类型是一个智能指针，它允许 Box 值被当作引用对待
pub enum List {
    Cons(i32, Box<List>),// Box将值放在堆上，而不是栈上
    Nil,
}


pub fn create_empty_list() -> List {
    List::Nil
}

pub fn create_non_empty_list() -> List {
    List::Cons(0, Box::new(List::Nil))
}

---------------------------------------------------------------------------------------------

// TODO
let saturn = Planet::Saturn(Rc::clone(&sun));
println!("reference count = {}", Rc::strong_count(&sun)); // 7 references
saturn.details();

// TODO
let uranus = Planet::Uranus(Rc::clone(&sun));
println!("reference count = {}", Rc::strong_count(&sun)); // 8 references
uranus.details();

// TODO
let neptune = Planet::Neptune(Rc::clone(&sun));
println!("reference count = {}", Rc::strong_count(&sun)); // 9 references
neptune.details();

// TODO
drop(earth);
println!("reference count = {}", Rc::strong_count(&sun)); // 3 references

// TODO
drop(venus);
println!("reference count = {}", Rc::strong_count(&sun)); // 2 references

// TODO
drop(mercury);
println!("reference count = {}", Rc::strong_count(&sun)); // 1 reference

---------------------------------------------------------------------------------------------

原子引用计数 Arc，但是原子性的使用会带来性能的降低
let shared_numbers = Arc::new(numbers);// TODO
let child_numbers = Arc::clone(&shared_numbers);// TODO
```

## 24.macros

```rust
宏调用需要加!
my_macro!();

---------------------------------------------------------------------------------------------

宏先声明后使用

---------------------------------------------------------------------------------------------

宏模块需要声明
#[macro_use]
mod macros {
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

---------------------------------------------------------------------------------------------

用;隔开宏的各模块
#[rustfmt::skip]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    };
}
```

## 25.clippy

```rust
clippy 工具是一系列 lint 的集合，用于捕捉常见错误和改进 Rust 代码
//let pi = 3.14f32;
let radius = 5.00f32;
let area = f32::consts::PI * f32::powi(radius, 2);

---------------------------------------------------------------------------------------------

if let Some(x) = option {
    res += x;
}

---------------------------------------------------------------------------------------------

#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    if my_option.is_none() {
        //my_option.unwrap();
    }

    let my_arr = &[-1, -2, -3 - 4, -5, -6];
    println!("My array! Here it is: {:?}", my_arr);

    let mut my_empty_vec = vec![1, 2, 3, 4, 5];
    my_empty_vec.clear();
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    // Let's swap these two!
    std::mem::swap(&mut value_a, &mut value_b);

    println!("value a: {}; value b: {}", value_a, value_b);
}
```

## 26.conversions

```rust
using_as
类型转换
total / values.len() as f64

---------------------------------------------------------------------------------------------

impl From<&str> for Person {
    fn from(s: &str) -> Person {
        let (name, age) = match s.split_once(',') {
            Some((name, age)) => (name.trim(), age.trim()),
            _ => return Person::default(),
        };

        if let Ok(age) = age.parse::<usize>() {
            if name.len() > 0 {
                return Person {
                    name: String::from(name),
                    age,
                };
            }
        }

        Person::default()
    }
}

---------------------------------------------------------------------------------------------

impl FromStr for Person {
    type Err = ParsePersonError;
    fn from_str(s: &str) -> Result<Person, Self::Err> {
        if s.is_empty() {
            return Err(ParsePersonError::Empty);
        }

        let splitted_item = s.split(',').collect::<Vec<&str>>();
        let (name, age) = match &splitted_item[..] {
            [name, age] => (
                name.to_string(),
                age.parse().map_err(ParsePersonError::ParseInt)?,
            ),
            _ => return Err(ParsePersonError::BadLen),
        };

        if name.is_empty() {
            return Err(ParsePersonError::NoName);
        }

        Ok(Person {
            name: name.into(),
            age,
        })
    }
}

---------------------------------------------------------------------------------------------

// Tuple implementation
impl TryFrom<(i16, i16, i16)> for Color {
    type Error = IntoColorError;
    fn try_from(tuple: (i16, i16, i16)) -> Result<Self, Self::Error> {
        let (red, green, blue) = tuple;

        for color in [red, green, blue] {
            if !(0..=255).contains(&color) {
                return Err(IntoColorError::IntConversion);
            }
        }
        Ok(Self {
            red: tuple.0 as u8,
            green: tuple.1 as u8,
            blue: tuple.2 as u8,
        })
    }
}

// Array implementation
impl TryFrom<[i16; 3]> for Color {
    type Error = IntoColorError;
    fn try_from(arr: [i16; 3]) -> Result<Self, Self::Error> {
        for color in arr {
            if !(0..=255).contains(&color) {
                return Err(IntoColorError::IntConversion);
            }
        }
        Ok(Self {
            red: arr[0] as u8,
            green: arr[1] as u8,
            blue: arr[2] as u8,
        })
    }
}

// Slice implementation
impl TryFrom<&[i16]> for Color {
    type Error = IntoColorError;
    fn try_from(slice: &[i16]) -> Result<Self, Self::Error> {
        if slice.len() != 3 {
            return Err(IntoColorError::BadLen);
        }
        for color in slice {
            if !(0..=255).contains(color) {
                return Err(IntoColorError::IntConversion);
            }
        }
        Ok(Self {
            red: slice[0] as u8,
            green: slice[1] as u8,
            blue: slice[2] as u8,
        })
    }
}

---------------------------------------------------------------------------------------------

// Add the AsRef trait appropriately as a trait bound
fn byte_counter<T: AsRef<str>>(arg: T) -> usize {
    arg.as_ref().as_bytes().len()
}

// Obtain the number of characters (not bytes) in the given argument
// Add the AsRef trait appropriately as a trait bound
fn char_counter<T: AsRef<str>>(arg: T) -> usize {
    arg.as_ref().chars().count()
}

// Squares a number using AsMut. Add the trait bound as is appropriate and
// implement the function body.
fn num_sq<T: AsMut<u32>>(arg: &mut T) {
    *arg.as_mut() *= *arg.as_mut()
}
```

