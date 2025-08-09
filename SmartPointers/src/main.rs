use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    println!("Hello, world!");

    let b = Box::new(5);
    println!("b = {}", b);



    let data = Rc::new(String::from("rust"));  // Rc只能适用于单线程环境，只读共享，不可变共享
    let a = Rc::clone(&data); //引用计数+1
    let b = Rc::clone(&data); //引用计数+1
    println!("{}, {}", a, b);    
    println!("ref cnt : {}", Rc::strong_count(&data)); // 适合多个模块都需要读同一份数据的场景



    let cell = RefCell::new(5);  // 单线程运行时可变借用检查, 搭配Rc<RefCell<T>> 实现多共享+可变
    *cell.borrow_mut() += 1; //内部可变
    println!("cell = {}", cell.borrow()); // 编译时看起来是不可变的，运行时检查可变性，同时borrow和borrow_mut会panic


    
    let info = Rc::new(RefCell::new(100));
    let d1 = Rc::clone(&info);
    let d2 = Rc::clone(&info);
    *d1.borrow_mut() += 1;
    println!("d2 sees: {}", d2.borrow()); //101,通过简单的组件，实现复杂而安全的共享可变结构

}


/*
Rust中没有GC，通过 所有权+智能指针+生命周期 实现了安全且高性能的内存管理
需要多个地方共享数据，修改数据，动态分配内存，这时候就需要智能指针


| 智能指针类型       | 用途                                    |
| ------------ | ------------------------                     |
| `Box<T>`     | 把数据放到堆上，保持所有权（递归类型、大小未知） |
| `Rc<T>`      | 多个只读所有者共享同一份数据（引用计数）        |
| `RefCell<T>` | 单线程内部可变性（允许运行时借用检查）          |
| `Arc<T>`     | 多线程共享所有权（原子引用计数）               |
| `Mutex<T>`   | 多线程可变共享（加锁）                        |

*/