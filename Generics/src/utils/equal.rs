use std::cmp::{PartialOrd, Ordering, PartialEq};
use std::fmt;

pub struct Person {
    pub name: String,
    pub age: u32,
}

impl PartialEq for Person {
    fn eq(&self, other: &Self) -> bool {
        self.age == other.age
    }
}

// u32自带的partial_cmp，返回Some(Ordering::Less)或Some(Ordering::Greater)
impl PartialOrd for Person {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {  // Self大写表示当前结构体类型
        self.age.partial_cmp(&other.age)
    }
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.age)
    }
}