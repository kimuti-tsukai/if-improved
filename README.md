# Usage
```rs
if_improved::ifi! {
    if let Some(v) = Some(101) if v % 2 == 0 {
        println!("{} is an Odd", v);
    } else if let Some(v) = Some(101) if v % 2 == 1 {
        println!("{} is not an Odd", v);
    } else {
        unreachable!();
    }
```
