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

## The match controll flow

**Match** allows to compare a value against a series of patterns and then execute code based on what pattern it matches.
Match can be likened to a coin-sorting machine such that the coin falls into the first hole it fits into. Using the coin analogy:

```Rust
enum Coin{
    Penny,
    Nickel,
    Dime,
    Quater,
}

fn value_in_cents(coin: Coin) -> u8{
    match coin{
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quater => 25,
    }
}
```

To break it down:

> In the `value_in_cents` function we declare `match` followed by and expression `coin`; just like in `if` statements with the difference being `if` statements need to return Boolean values but `match` can go with any type.
> Next are the `match` arms. Arm is comprised of two parts which are: the first part which contains the pattern which is the value; `Coin::Penny` and the `=>` operator which separates the pattern and the code to run; in this case return `1`.
> The code associated in each arm is an expresion and the resulting value from the expresion is returned as the value for the whole match expression.

### Patterns that match values

Match can bind to the parts of the values that match the pattern; for instance:

```Rust
enum UsState{
    Alabama,
    Alaska,
}

enum Coin{
    Penny,
    Nickel,
    Dime,
    Quater(UsState),
}
```

The `Quater` variant includes a UsState value stored in it
In a match expresion for this enum we add a `state` variable that matches the values of variant `Coin::Quater`. When a `Coin::Quater` matches, the state variable will bind to the value of that quater's state. We can then use `state` in the code of that arm.
I.e.

```Rust
fn value_in_cents(coin::Coin) ->u8{
    match coin{
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin:Quater(state) =>{
            Println!("State Quater from {:?}", state);
            1
        }
    }
}
```

we can call this by:

```Rust
value_in_cents(Coin::Quater(UsState::Alaska));
```

### Matching with Option\<T>

We can use `match` to handle `T` in the `Some` variant. I.e.

```Rust
fn plus_one(x: Option<i32>) -> Option<i32>{
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);

```

Here when we pass `plus_one` with `five` it checks if it has a match, in the first arm it finds no match so it moves to the second arm `Some(i)...` and finds a match. The `i` binds to the value contained in `Some` so `i` takes the value `5`. The code in the match arm is executed; 1 is added to the given value.
In tge secod instance `none` it checks the arm and matches the `None` arm so the code executed will be `None`.

### Matches are exhausive

The are exhausive in a way that we must handle all the cases in a given pattern. The Rust compiler would result in errors if we don't handle all cases.

### Catch-all Patterns and the \_placeholder

Using enums we can take actions for a few particular values and also for all values that tale the default action. I.e.

```Rust
let dice_roll = 9;
match dice_roll{
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    other => move_player(other),
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_palyer(num_spaces: u8) {}
```

The first two arms cover the `3` and `7` values. The last arm covers every other possible values, the pattern uses a varible we have choosen to name `other`. The code that runs in the `other` uses the variable by passing it to the `move_player` function.
The code compiles, even though we haven't listed any values a `u8` will have, because the last pattern will match all values no specified.
The catch-all pattern meets the requirements that a `match` must be exhausive.
A catch-all arm should be put as the last arm in a match since patterns are evaluated in order.

When we don't want to use a value in a catch-all pattern we use: `_`, which is a special pattern that matches any value and does not bind to that value.

```Rust
let dice_roll = 9;
match dice_roll{
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => reroll(),
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn reroll() {}
```

or `_ => (),` if we want nothing to occur if we fail to pass first and second arms.
