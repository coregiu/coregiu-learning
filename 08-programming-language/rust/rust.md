# rust-learning
Rust学习总结
[参照：https://rustwiki.org/zh-CN/rust-by-example/index.html](https://rustwiki.org/zh-CN/rust-by-example/index.html)
#### 基础语法及数据类型
主函数
```rust
fn main(){
}
```
 

变量定义：
```rust
 let 变量名: 变量类型 = 值;  // 默认不可变变量

 let mut 变量名: 变量类型 = 值;  // 可变变量
```
 

常量定义
```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
``` 

数据类型：

一 标量

1 整型

| 长度    | 有符号 | 无符号 | 说明                                                         |
| ------- | ------ | ------ | ------------------------------------------------------------ |
| 8-bit   | i8     | u8     | 1字节                                                        |
| 16-bit  | i16    | u16    | 2字节                                                        |
| 32-bit  | i32    | u32    | 4字节                                                        |
| 64-bit  | i64    | u64    | 8字节                                                        |
| 128-bit | i128   | u128   | 16字节                                                       |
| arch    | isize  | usize  | isize 和 usize 类型依赖运行程序的计算机架构：64 位架构上它们是 64 位的，32 位架构上它们是 32   位的。 |

 

字面值

| 数字字面值                  | 例子        |
| --------------------------- | ----------- |
| Decimal   (十进制)          | 98_222/3u8  |
| Hex   (十六进制)            | 0xff        |
| Octal   (八进制)            | 0o77        |
| Binary   (二进制)           | 0b1111_0000 |
| Byte (单字节字符)(仅限于u8) | b'A'        |

 

2 浮点型

f32, f64两种类型，分别4，8字节。 默认f64类型
```rust
fn main() { 
let x = 2.0; // f64 
let y: f32 = 3.0; // f32 
}
```
 

3 布尔型  
```rust
fn main() { 
let t = true; 
let f: bool = false; // with explicit type annotation
}
```
占1个字节

 

4 字符型
```rust
fn main() { 
let c = 'z';
let z: char = 'ℤ'; // with explicit type annotation
let heart_eyed_cat = '😻';
}
```

占四个字节，

代表了一个 Unicode 标量值（Unicode Scalar Value），这意味着它可以比 ASCII 表示更多内容。在 Rust 中，带变音符号的字母（Accented letters），中文、日文、韩文等字符，emoji（绘文字）以及零长度的空白字符都是有效的 char 值。Unicode 标量值包含从 U+0000 到 U+D7FF 和 U+E000 到 U+10FFFF 在内的值。
 

二 复合类型

1 元组类型 tuple

元组是一个将多个其他类型的值组合进一个复合类型的主要方式。元组长度固定：一旦声明，其长度不会增大或缩小。

 
```rust
fn main() { 
let tup: (i32, f64, u8) = (500, 6.4, 1); 
}

//  解构访问
fn main() { 
let tup = (500, 6.4, 1); 
let (x, y, z) = tup;
println!("The value of y is: {y}"); 
}
```
 

// .序号访问
```rust
fn main() { 
let x: (i32, f64, u8) = (500, 6.4, 1); 
let five_hundred = x.0; 
let six_point_four = x.1; 
let one = x.2; 
}
```

不带任何值的元组有个特殊的名称，叫做 单元（unit） 元组。这种值以及对应的类型都写作 ()，表示空值或空的返回类型。如果表达式不返回任何其他值，则会隐式返回单元值。

 
2 数组类型 array

与元组不同，数组中的每个元素的类型必须相同。Rust 中的数组与一些其他语言中的数组不同，Rust 中的数组长度是固定的。

 
```rust
fn main() { 
let a = [1, 2, 3, 4, 5]; 
}
```
 

这样编写数组的类型：在方括号中包含每个元素的类型，后跟分号，再后跟数组元素的数量。
```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
```
 

还可以通过在方括号中指定初始值加分号再加元素个数的方式来创建一个每个元素都为相同值的数组：
```rust
let a = [3; 5];
```
 

[访问数组元素](https://kaisery.github.io/trpl-zh-cn/ch03-02-data-types.html#%E8%AE%BF%E9%97%AE%E6%95%B0%E7%BB%84%E5%85%83%E7%B4%A0)
```rust
fn main() { 
let a = [1, 2, 3, 4, 5]; 
let first = a[0]; 
let second = a[1]; 
}
```


#### 常用集合容器
[向量：Vec](demo/Vec.rs)

[双端队列: VecDeque](demo/VecDeque.rs)

[链表: LinkedList](demo/LinkedList.rs)

[映射：HashMap, BTreeMap](demo/Map.rs)

[集合：HashSet, BTreeSet](demo/Set.rs)

[优先队列：BinaryHeap](demo/BinaryHeap.rs)

[字符串：String](demo/String.rs)

#### 运算符

表 B-1 包含了 Rust 中的运算符、运算符如何出现在上下文中的示例、简短解释以及该运算符是否可重载。如果一个运算符是可重载的，则该运算符上用于重载的相关 trait 也会列出。

表 B-1: 运算符

| 运算符 | 示例                                     | 解释                                               | 是否可重载   |
| ------ | ---------------------------------------- | -------------------------------------------------- | ------------ |
| !      | ident!(...), ident!{...}, ident![...]    | 宏展开                                             |              |
| !      | !expr                                    | 按位非或逻辑非                                     | Not          |
| !=     | expr !=   expr                           | 不等比较                                           | PartialEq    |
| %      | expr %   expr                            | 算术取余                                           | Rem          |
| %=     | var %=   expr                            | 算术取余与赋值                                     | RemAssign    |
| &      | &expr, &mut expr                         | 借用                                               |              |
| &      | &type, &mut type, &'a type, &'a mut type | 借用指针类型                                       |              |
| &      | expr   & expr                            | 按位与                                             | BitAnd       |
| &=     | var   &= expr                            | 按位与及赋值                                       | BitAndAssign |
| &&     | expr   && expr                           | 短路（Short-circuiting）逻辑与                     |              |
| *      | expr *   expr                            | 算术乘法                                           | Mul          |
| *=     | var *=   expr                            | 算术乘法与赋值                                     | MulAssign    |
| *      | *expr                                    | 解引用                                             | Deref        |
| *      | *const type, *mut type                   | 裸指针                                             |              |
| +      | trait + trait, 'a + trait                | 复合类型限制                                       |              |
| +      | expr +   expr                            | 算术加法                                           | Add          |
| +=     | var +=   expr                            | 算术加法与赋值                                     | AddAssign    |
| ,      | expr,   expr                             | 参数以及元素分隔符                                 |              |
| -      | - expr                                   | 算术取负                                           | Neg          |
| -      | expr -   expr                            | 算术减法                                           | Sub          |
| -=     | var -=   expr                            | 算术减法与赋值                                     | SubAssign    |
| ->     | fn(...) -> type, \|...\| -> type         | 函数与闭包，返回类型                               |              |
| .      | expr.ident                               | 成员访问                                           |              |
| ..     | .., expr.., ..expr, expr..expr           | 右开区间范围                                       | PartialOrd   |
| ..=    | ..=expr, expr..=expr                     | 右闭区间范围模式                                   | PartialOrd   |
| ..     | ..expr                                   | 结构体更新语法                                     |              |
| ..     | variant(x, ..), struct_type { x, .. }    | “与剩余部分” 的模式绑定                            |              |
| ...    | expr...expr                              | （Deprecated，请使用 ..=）在模式中：闭区间范围模式 |              |
| /      | expr /   expr                            | 算术除法                                           | Div          |
| /=     | var /=   expr                            | 算术除法与赋值                                     | DivAssign    |
| :      | pat: type, ident: type                   | 约束                                               |              |
| :      | ident:   expr                            | 结构体字段初始化                                   |              |
| :      | 'a: loop   {...}                         | 循环标志                                           |              |
| ;      | expr;                                    | 语句和语句结束符                                   |              |
| ;      | [...;   len]                             | 固定大小数组语法的部分                             |              |
| <<     | expr   << expr                           | 左移                                               | Shl          |
| <<=    | var   <<= expr                           | 左移与赋值                                         | ShlAssign    |
| <      | expr   < expr                            | 小于比较                                           | PartialOrd   |
| <=     | expr   <= expr                           | 小于等于比较                                       | PartialOrd   |
| =      | var = expr, ident = type                 | 赋值/等值                                          |              |
| ==     | expr ==   expr                           | 等于比较                                           | PartialEq    |
| =>     | pat   => expr                            | 匹配准备语法的部分                                 |              |
| >      | expr   > expr                            | 大于比较                                           | PartialOrd   |
| >=     | expr   >= expr                           | 大于等于比较                                       | PartialOrd   |
| >>     | expr   >> expr                           | 右移                                               | Shr          |
| >>=    | var   >>= expr                           | 右移与赋值                                         | ShrAssign    |
| @      | ident @   pat                            | 模式绑定                                           |              |
| ^      | expr ^   expr                            | 按位异或                                           | BitXor       |
| ^=     | var ^=   expr                            | 按位异或与赋值                                     | BitXorAssign |
| \|     | pat \|   pat                             | 模式选择                                           |              |
| \|     | expr \|   expr                           | 按位或                                             | BitOr        |
| \|=    | var \|=   expr                           | 按位或与赋值                                       | BitOrAssign  |
| \|\|   | expr \|\|   expr                         | 短路（Short-circuiting）逻辑或                     |              |
| ?      | expr?                                    | 错误传播                                           |              |

[非运算符符号](https://kaisery.github.io/trpl-zh-cn/appendix-02-operators.html#%E9%9D%9E%E8%BF%90%E7%AE%97%E7%AC%A6%E7%AC%A6%E5%8F%B7)

下面的列表中包含了所有和运算符不一样功能的符号；也就是说，它们并不像函数调用或方法调用一样表现。

表 B-2 展示了以其自身出现以及出现在合法其他各个地方的符号。

表 B-2：独立语法

| 符号                               | 解释                                               |
| ---------------------------------- | -------------------------------------------------- |
| 'ident                             | 命名生命周期或循环标签                             |
| ...u8, ...i32, ...f64, ...usize 等 | 指定类型的数值常量                                 |
| "..."                              | 字符串常量                                         |
| r"...", r#"..."#, r##"..."##, etc. | 原始字符串字面值，未处理的转义字符                 |
| b"..."                             | 字节字符串字面值; 构造一个字节数组类型而非字符串   |
| br"...", br#"..."#, br##"..."## 等 | 原始字节字符串字面值，原始和字节字符串字面值的结合 |
| '...'                              | 字符字面值                                         |
| b'...'                             | ASCII   码字节字面值                               |
| \|...\|   expr                     | 闭包                                               |
| !                                  | 离散函数的总是为空的类型                           |
| _                                  | “忽略”   模式绑定；也用于增强整型字面值的可读性    |

表 B-3 展示了出现在从模块结构到项的路径上下文中的符号

表 B-3：路径相关语法

| 符号                                | 解释                                                         |
| ----------------------------------- | ------------------------------------------------------------ |
| ident::ident                        | 命名空间路径                                                 |
| ::path                              | 与 crate 根相对的路径（如一个显式绝对路径）                  |
| self::path                          | 与当前模块相对的路径（如一个显式相对路径）                   |
| super::path                         | 与父模块相对的路径                                           |
| type::ident, <type as trait>::ident | 关联常量、函数以及类型                                       |
| <type>::...                         | 不可以被直接命名的关联项类型（如 <&T>::...，<[T]>::...，等） |
| trait::method(...)                  | 通过命名定义的 trait 来消除方法调用的二义性                  |
| type::method(...)                   | 通过命名定义的类型来消除方法调用的二义性                     |
| <type   as trait>::method(...)      | 通过命名 trait 和类型来消除方法调用的二义性                  |

表 B-4 展示了出现在泛型类型参数上下文中的符号。

表 B-4：泛型

| 符号                       | 解释                                                         |
| -------------------------- | ------------------------------------------------------------ |
| path<...>                  | 为一个类型中的泛型指定具体参数（如 Vec<u8>）                 |
| path::<...>, method::<...> | 为一个泛型、函数或表达式中的方法指定具体参数，通常指 turbofish（如 "42".parse::<i32>()） |
| fn   ident<...> ...        | 泛型函数定义                                                 |
| struct   ident<...> ...    | 泛型结构体定义                                               |
| enum   ident<...> ...      | 泛型枚举定义                                                 |
| impl<...>   ...            | 定义泛型实现                                                 |
| for<...>   type            | 高级生命周期限制                                             |
| type<ident=type>           | 泛型，其一个或多个相关类型必须被指定为特定类型（如 Iterator<Item=T>） |

表 B-5 展示了出现在使用 trait bounds 约束泛型参数上下文中的符号。

表 B-5: Trait Bound 约束

| 符号                      | 解释                                                         |
| ------------------------- | ------------------------------------------------------------ |
| T: U                      | 泛型参数 T 约束于实现了 U 的类型                             |
| T: 'a                     | 泛型 T 的生命周期必须长于 'a（意味着该类型不能传递包含生命周期短于 'a 的任何引用） |
| T:   'static              | 泛型 T 不包含除 'static 之外的借用引用                       |
| 'b: 'a                    | 泛型 'b 生命周期必须长于泛型 'a                              |
| T:   ?Sized               | 使用一个不定大小的泛型类型                                   |
| 'a + trait, trait + trait | 复合类型限制                                                 |

表 B-6 展示了在调用或定义宏以及在其上指定属性时的上下文中出现的符号。

表 B-6: 宏与属性

| 符号                                  | 解释     |
| ------------------------------------- | -------- |
| #[meta]                               | 外部属性 |
| #![meta]                              | 内部属性 |
| $ident                                | 宏替换   |
| $ident:kind                           | 宏捕获   |
| $(…)…                                 | 宏重复   |
| ident!(...), ident!{...}, ident![...] | 宏调用   |

表 B-7 展示了写注释的符号。

表 B-7: 注释

| 符号     | 注释           |
| -------- | -------------- |
| //       | 行注释         |
| //!      | 内部行文档注释 |
| ///      | 外部行文档注释 |
| /*...*/  | 块注释         |
| /*!...*/ | 内部块文档注释 |
| /**...*/ | 外部块文档注释 |

表 B-8 展示了出现在使用元组时上下文中的符号。

表 B-8: 元组

| 符号                 | 解释                                                         |
| -------------------- | ------------------------------------------------------------ |
| ()                   | 空元组（亦称单元），即是字面值也是类型                       |
| (expr)               | 括号表达式                                                   |
| (expr,)              | 单一元素元组表达式                                           |
| (type,)              | 单一元素元组类型                                             |
| (expr,   ...)        | 元组表达式                                                   |
| (type,   ...)        | 元组类型                                                     |
| expr(expr,   ...)    | 函数调用表达式；也用于初始化元组结构体 struct 以及元组枚举 enum 变体 |
| expr.0, expr.1, etc. | 元组索引                                                     |

表 B-9 展示了使用大括号的上下文。

表 B-9: 大括号

| 符号         | 解释          |
| ------------ | ------------- |
| {...}        | 块表达式      |
| Type   {...} | struct 字面值 |

表 B-10 展示了使用方括号的上下文。

表 B-10: 方括号

| 符号                                       | 解释                                                         |
| ------------------------------------------ | ------------------------------------------------------------ |
| [...]                                      | 数组                                                         |
| [expr;   len]                              | 复制了 len个 expr的数组                                      |
| [type;   len]                              | 包含 len个 type 类型的数组                                   |
| expr[expr]                                 | 集合索引。重载（Index, IndexMut）                            |
| expr[..], expr[a..], expr[..b], expr[a..b] | 集合索引，使用 Range，RangeFrom，RangeTo 或 RangeFull 作为索引来代替集合 slice |

