# Enums and pattern matching

_Enums_ allows you to define a type by enumarating its posible variants.

### IP address as example

There are two variations of IP address which can be `v4` or `v6` and an IP address can be of only one type. Both versions are still fundamentally IP addresses and should be treated as one type code handling them.
I.e.

```Rust
enum IPAddrKind{
    v4,
    v6,
}
```

`IPAddrKind` is now a custom type we could use anywhere in our code.

#### Enum Value

we can create and instance of the two variants of `IPAddrKind` like this.

```Rust
let four = IPAddrKind::v4;
let six = IPAddrKind::v6;
```

Variants of the enum are namespaced under its identifier, in this case `IPAddrKind`
We can define a function that takes in any `IPAddrKind`. i.e.

```Rust
fn Routes(IP: IPAddrKind){}
```

When calling this function we use either variant. I.e.

```Rust
Routes(IPAddrKind::v4);
Routes(IPAddrKind::v4);
```

> For now we only know the IP kind but not the IP data(address). To store this we could use the _Structs_ route or we could use _Enums_ a more coincice way to solve it.
> I.e.

```Rust
enum IpAddr{
    v4(String),
    v6(String),
}

let four = IpAddr::v4(String::from("127.0.0.1"));
let six = IpAddr::v6(String::from("::1"));
```

This definition of `IpAddr` shows that `v4` and `v6` will have `String` associated values.
By doing this we automatically get a constructor function defined; `IpAddr::v4()` takes in a `String` argument and returns an instance of type `IpAddr`

> Enums provide an advantage over structs such that each variant can have different types and amounts of associated data.

> An Enum variant can have any kind of data in it: it can even have enums in it.

```Rust
enum Message{
    Quit,
    Move{x:i32, y:i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

This enum has 4 variants:

- `Quit` - has no data associated with it at all.
- `Move` - Has named fields just like in structs.
- `Write` - Has a String argument associated with it.
- `ChangeColor` - includes 3 `i32` values.

If we had used structs we'd have done this:

```Rust
struct QuitMessage; //Unit struct
struct MoveMessage{
    x: i32,
    y: i32,
};
struct WriteMessage(String); //Tuple struct
struct ChangeColor(i32, i32, i32); //Tuple struct
```

By using structs we canot define a function that takes `Message` as a type unilike in Enums.

One Feature that enumus has related to structs is that we can derive methods from it using `impl`. I.e.

```Rust
impl Message{
    fn call(&self){
        // method body
    }
}
let m = Message::Write(String::from("data"));
m.call();
```

## The Option Enum

_Option_ is an enum defined by the standard library.
It encodes very common scenarios where a value could be something or nothing.
Since Rust does not have _Null_ like other programming languages do, we use Option enum to check if a result is _null_ or _not_null_
The enum is `Option<T>` and is defined by the std lib as follows:

```Rust
enum Option<T>{
    None,
    Some(T)
}
```

The `Option` enum is included in the every Rust program prelude so we can freely use `Some` and `None` in the program without importing them to the file.

#### Example

```Rust
let some_number = Some(5);
let some_string = Some("data");

let absent_number: Option<i32> = None;
```

Rust can infer the types of `some_number` and `some_string` automaticaly because we have specified a value in the `Some` variant. For `None` variant we are required to annotate the type in this case we annotate it as `i32`.

###

When we have a `Some` value we know that the value is present and is held by the `Some` variant.
When we have the `None` value, in some sense it means the same thing as null; we have an invalid value.
