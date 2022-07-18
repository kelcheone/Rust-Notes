## Structs

_Structs_ are a custom datatype that lets you package and name multiple related values to a meaningful group.

### Defining and instantiating structs

We define structs using the `struct` keyword: i.e.

```Rust
struct User{
    active: bool,
    username: String,
    email: String,
}
```

So as to use a struct after defining it, we instantiate it by defining concrete values for each value in the struct fields. Fields can be filled in any order without following how they were defined in the struct.

```Rust
fn main(){
    let user = User{
        username: String::from("Bob"),
        email: String::from("bob@mail.com"),
        active: true,
    }
}
```

We use dot notation to get a specific value in a struct i.e. `user.email` to get the user email. For mutable struct instances we can change the value of a given field also using the dot natation.

```Rust
user.name = String::from("Alice");
```

We can also return Struct instances i.e.

```Rust
fn rt_user(username: &str, email: &str) -> User{
User{
    username,
    email,
    active: true,
}
}
```

##### Struct update syntax

When re-using values from a previous instanciated struct we could use a more cleaner pattern. i.e.

```Rust
let user2 = User{
    email = String::from("Alice@mail.com")
    ..user1
}
```

The above code uses the other fields from user1 and just updates the email.

### Method Syntax

Methods behave like methods but unlike methods they are defined within the construct of a struct/(enum / traits).
The first parameter of a method is always a `self` which represents the instance of the struct the method is being called on.

###### Defining methods

Example of a method on a struct

```Rust
#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}

impl Rectangle{
    fn area(&self) -> u32{
        self.width * self.height
    }
}

fn main(){
    let rect = Rectangle{
        width: 23,
        height: 12,
    }

    println!("The are of the rectangle is {} in square pixels.", rect.area() )
}
```

So as to define a function within the context of `Rectangle`, we use `impl` for the `Rectangle`. By doing this everyting within the `impl` block will be asociated with `Rectangle`.
We then move the `area` function within the `impl` block and change its first parameter to `self`. To access the `area` method in the `main` function we use dot notation.
The `&self` signature is short for `self: &Self`. Within an `impl` block `Self` is an alias for the type that
the `impl` is for; in this case `Rectangle`. Methods must have a parameter named `self` of type `Self` as its first parameter, and Rust allows us to abreviate it as `self`

> A method can have several parameters.

### Associated Functions

All functions defined withing an `impl` block are refered to as _associated functions_; this is because they are associated to the type named after the `impl`.
We can define associated functions that don't have the `self` as a parameter (thus are not methods) because they don't need an instance of type to work with.
Associated functions that aren't methods are often used for constructors that will return a new instance of the struct.
To call associated function we use `::`syntax with the struct name, i.e.

```Rust
impl Rectangle{
    fn Square(size: u32) -> Rectangle{
        Rectangle{
            width: size,
            height: size,
        }
    }
}

fn main(){
    let square = Rectangle::Square(3);
}
```

The `::` syntax is used for both associated functions and in namespaces created by modules.

### multiple `impl` blocks

You could have multiple `impl` blocks within an `impl` block.
