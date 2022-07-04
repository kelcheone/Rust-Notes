## Macros

This is a way of writing code that writes other code also known as _metaprogramming_.
I.e. `println!`
They are usefull for reducing the amount of code you have to write or maintain.
They are powerfull more than functions because they can take a variable amounts of parameters; like in the case of `println!` it can take `println!("hello")` or `println!("{}", par)`

#### User Input

To get user input we use the rust standard library. To use it in a project :

```Rust
use std::io
```

By doing this you can now use the I/O library. i.e;

```Rust
// To get inputs from the cmd.
io::stdin().read_line()
```

## Variable Mutability.

By default variables are immutable in rust.
to make a variable mutable we use `mut` keyword i.e,

```Rust
let mut x = 23;
```

## Data types;

##### integer type

can be signed or unsigned with sizes spanning from 8 bits to 128 bits and the default being 32 bits

##### floating type

by default they are of size 64 but can be of size 32 when explicitly stated.

##### boolean

##### The character type

Used to declare `char` type variables. They are of size 4 bits and take in alpahabetical values.

### Compund data types.

##### Tuple data type

```Rust
let tup: (f64, i32, u16) =(455.78, -345, 908);
```

They can be accessed by destructing of using dots `.` i.e.

```rust
(x, y, z) = tup;
// or
let x2 = tup.0
let y2 = tup.1
let z2 = tup.2
```

### Arrays

```Rust
let arr = [1, 2, 3, 4]
```

They are prefferable when dealing with fixed arrays but for dynamic arrays using Vectors is recommended.

###### fixed array

```Rust
// Array of length 5.
let arr2 :[i32,  5] =[1, 2, 3, 4, 5];
// or
let arr3 = [3;5];
// this will create and array with length 5 and its elements as 3 similar to
let arr3 = [3, 3, 3, 3, 3];
```

we can access array element using brackets i.e;

```Rust
let el1 = arr[0];
```
