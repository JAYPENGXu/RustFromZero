fn main() {
    println!("Hello, world!");

    let add = |a: i32, b: i32| -> i32 {
        a + b
    };
    println!("{}", add(1, 2));

    let name = String::from("Jacky");
    let say_hi = || {
        println!("hi, {}", name);
    };

    say_hi();

    let nums = vec![1, 2, 3, 4, 5];
    for n in nums.iter() {
        println!("{}", n);
    }

    let even_squares: Vec<_> = nums
    .iter().filter(|x| *x %2 == 0).map(|x| x * x).collect();
    println!("{:?}", even_squares);

    // 判断是否为偶数
    let is_even = |x: &i32| x % 2 == 0;
    let evens: Vec<i32> = nums.into_iter().filter(is_even).collect();
    println!("{:?}", evens);

    // 用.map把每个字符串加上！
    let words = vec!["hello", "rust", "world"];
    let excited: Vec<String> = words.iter().map(|word| format!("{}!", word)).collect();
    println!("{:?}", excited);

    // 用fold累加数字、找最大值、拼接字符串
    let sum = vec![1, 2, 3, 4, 5].iter().fold(0, |acc, x| acc + x);
    println!("sum = {}", sum);

    let max = vec![1, 3, 7, 9, 2].iter().fold(i32::MIN, |acc, x| acc.max(*x));
    println!("max = {}", max);


    let parts = vec!["hello", "rust", "world"];
    let sentence = parts.iter().fold(String::new(), |mut acc, word|{
        acc.push_str(word);
        acc.push(' ');
        acc
    });
    println!("{}", sentence.trim());

    // ---------------
    let message = "hello000";
    let say_hello = || println!("{}", message); // 只读借用
    call_with_fn(say_hello);  // ✅ OK


    // ----------------
    let mut count = 0;
    let mut add = || {
        count += 1;  // 可变借用
        println!("count = {}", count);
    };
    call_with_fnmut(add);  // ✅ OK


    // ----------------
    let s = String::from("owned");

    let take = || {
        println!("using {}", s); // 👈 闭包会 move s
        drop(s); // 所有权被消费掉
    };

    call_with_fnonce(take);  // ✅ OK


}


/*
闭包就是可以捕获环境变量的匿名函数，可以赋值给变量、作为参数传递
*/

/* 
常用迭代器方法：
.iter() 获取不可变迭代器
.into_iter() 获取拿走值的迭代器
.map() 映射每个值
.filter() 过滤符合条件的项
.collect() 收集结果成Vec\HashMap
.fold() 归约，累加求和

三种.iter()差异：

| 用法              | 元素类型|闭包参数类型 | 说明               |
| ----------------- | ---- | -------  | ----------------------       |
| `iter()`          | `&T` |  `&&T`   | 常用于只读不转 ownership      |
| `iter().cloned()` | `&T` |  `&&T`   | 后续用 `.cloned()` 拿到 `T`   |
| `into_iter()`     | `T`  |  `T`     | 拿走所有权                    |

*/

// Fn:只读引用
fn call_with_fn<F: Fn()>(f: F) {
    f();
}

// FnMut 可变借用
fn call_with_fnmut<F: FnMut()> (mut f: F) {
    f();
    f();
}

// 闭包会拿走外部值的所有权
fn call_with_fnonce<F: FnOnce()>(f: F) {
    f(); //只能调用一次
}
