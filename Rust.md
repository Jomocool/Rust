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

