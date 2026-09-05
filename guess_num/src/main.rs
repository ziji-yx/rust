//生成一个1到100的随机数
//提示玩家输入一个猜测
//程序会提示太小了还是太大了
//如果刚好猜对，程序打印一个庆祝信息然后退出
use std::io;
use rand::Rng;//trait(特征)
use std::cmp::Ordering;//引入枚举类型Ordering
//引入外部包需要去toml里面修改dependencies,相关注释在toml文件中
fn main() {
    println!("猜数游戏");

    let secret_number=rand::thread_rng().gen_range(1,101);//i32,u32,i64三种类型可匹配，会自动匹配成i32

    loop{
        println!("猜测一个数：");

        let mut guess = String::new();// let 用于声明变量，mut表示可修改，若不特殊声明，一般rust的变量是immutable的

        io::stdin().read_line(&mut guess).expect("无法读取行");

        let guess:u32 = match guess.trim().parse(){
            Ok(num)=>num,
            Err(_)=>continue,
        };
        // rust 允许生成同名新变量来隐藏旧变量.此时新变量guess变immutable了
        // guess:u32是显式声明新变量的类型为u32
        // trim 会去掉两边的内容，比如输入数字时产生的\n
        // parse 会把guess转换成字符串
        println!("你猜测的数是：{}",guess);

        match guess.cmp(&secret_number){
            Ordering::Less=>println!("Too small!"), //match 有多个arm（分支）
            Ordering::Greater=>println!("Too big!"),//如果match后面的返回值与某一个arm匹配
            Ordering::Equal=>{                      //就会执行箭头后面的代码
                println!("You win!");
                break;
            }
            //按照顺序依次匹配
        };
        //由于cmp中guess与secret_number比较，secret_number的类型被猜测成了u32（与guess一致）
        // rust 支持极强的类型推导
    }
    //loop无限循环，相当于while
}
