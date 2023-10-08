# Differences between associated types and generics

## 区别

1. 关联类型仅作用在 trait 上，泛型则除了 trait 外还能用于函数、结构体中
2. 泛型 trait 允许针对泛型参数的不同类型实现多次，关联类型则不行

## 举例说明

`Add<Rhs>` 这个标准库中的 trait 定义如下：

````rs
pub trait Add<Rhs = Self> {
    /// The resulting type after applying the `+` operator.
    #[stable(feature = "rust1", since = "1.0.0")]
    type Output;

    /// Performs the `+` operation.
    ///
    /// # Example
    ///
    /// ```
    /// assert_eq!(12 + 1, 13);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    #[rustc_diagnostic_item = "add"]
    #[stable(feature = "rust1", since = "1.0.0")]
    fn add(self, rhs: Rhs) -> Self::Output;
}
````

对于 `+` 运算符，可以有多种情况，比如 `String + Foo`，`String + Bar` 等，那么此时就需要分别为 String 实现右操作数类型为 Foo 和 Bar 时的表现，也就是：

```rs
// String + Foo
impl Add<Foo> for String {
  type Output = String

  fn add(self, rhs: Foo) -> Self::Output
}

// String + Bar
impl Add<Bar> for String {
  type Output = String

  fn add(self, rhs: Bar) -> Self::Output
}
```

**这种一个 trait 需要针对不同类型具有不同实现的场景适合用泛型**

另外，需要注意到该 trait 定义了一个关联类型 `Output`，为什么不将其作为泛型而是关联类型呢？

如果作为泛型，那意味着允许 `String + Foo` 得到 `String`，也允许 `String + Foo` 得到 `Foo`，但这显然是不合理的，两个确定的类型相加，得到的类型也应当是确定的，
因此这种场景就更加适合用关联类型。

总结下来就是 **在设计 trait 时，遇到不确定的类型，且希望针对不同类型有不同表现时使用泛型；关联类型则用于当你希望在一个 trait 的实现中只会有一种可能的具体类型时。**
