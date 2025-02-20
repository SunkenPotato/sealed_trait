# `sealed_trait`
A utility for generating sealed traits in Rust.
Inspired by Java's `sealed class` syntax.

## Usage
You can create a sealed trait by using the `sealed_trait` macro:
```rs
sealed_trait! {
    pub sealed trait TestTrait permits i32 => {
        fn print_me(self);
    }

    impl TestTrait for i32 => {
        fn print_me(self) {
            println!("{self}")
        }
    }
}
```
You can also add supertraits to your traits, but they have to be inside square brackets:
```rs
sealed_trait! {
    pub sealed trait TestTrait: [Sized, Into<u32>] permits i32 => {
        ...
    }
}
