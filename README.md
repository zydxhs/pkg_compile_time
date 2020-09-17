# pkg_compile_time
一个返回编译日期、时间的过程宏。

# 用法

use pkg_compile_time::*;

输出编译日期：
```
println!("Compile date is: {}", pkg_compile_date!());
```

输出编译时间：
```
println!("Compile time is: {}", pkg_compile_time!());
```