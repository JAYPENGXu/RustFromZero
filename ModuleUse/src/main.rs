mod math; //引入match目录下的mod.rs
mod utils;
fn main() {
    println!("Hello, world!");
    let res = math::basic::multiply(4, 6);
    println!("res is : {}",res);
    
    let len = utils::string::length("rust");
    let sum = utils::math::add(1, 2);
    println!("len is: {}, sum is: {}", len, sum);
}


/*
mod：定义一个模块，可以理解为一个命名空间或子文件
pub：表示对外公开，默认是私有
crate：当前整个包（项目）范围，也就是根作用域
use：用来引入模块或函数等路径，就像import
*/

// 模块中的所有东西默认都是私有的private
// 用pub公开给其他模块访问
// 也可以使用pub(crate)表示仅在当前crate可见

/*
crate是什么：
每个Cargo.toml项目就是一个crate
main.rs是crate的根 （对于binary crate）
lib.rs是crate的根 (对于library crate)

*/