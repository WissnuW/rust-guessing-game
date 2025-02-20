# OVERVIEW


```
use std::cmp::Ordering;
fn main() {
    let a = 10;
    let b = 20;
    match a.cmp(&b) {
        Ordering::Less => println!("a lebih kecil dari b"),
        Ordering::Greater => println!("a lebih besar dari b"),
        Ordering::Equal => println!("a sama dengan b"),
    }
} 
