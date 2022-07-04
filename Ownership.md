# Ownership

**Ownership** is a set of rules that govern how a Rust program manages the memory.

> - The stack stores fixed size data. Data of dynamic sizes are stored in the Heap.
> - The heap is less organized and when data is put in the heap you make a request for some amount of space. The memory allocator in turn looks for a big enough space in the heap and returns a _pointer_ which is the address in which the dat is stored. This process is referred to as _allocating in the heap_ or simply _allocation_.
> - since the size of the address in the heap is known this means that you can store the address in the stack.
> - Pushing and retriving from the stack is much faster compared to using the heap because for the heap the allocator must first find the space before allocating and when retriving from the heap you must first use a pointer.

## Ownership rules

Some rules to keep in mind.

- Each value in Rust has an _owner_.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

## The String type

string literals are immutable i.e.

```Rust
let s = "hello";
```

But to have mutable string we use String type uses some other form of memory allocation to provide mutability to strings i.e.

```Rust
let mut s = String::from("hello"): // we use from() to extract type String from string literals
s.push_str(", world"); // This appends string litteral to the String making the s = "hello, world"
```

> ### Memory allocation
>
> With the above example when we use `String::from()` we allocate a memory space in the heap( this is what happens in all programming languages).
> Unlike other programming languages without a garbage collector where you have to allocate and de-allocate the memory, Rust takes an new paradigm where the memory is automatically deallocated once its out of scope i.e.

```Rust
{ // the memory is empty
   let s = String::from("Hello"); // The memory is allocated.
    //    do stuff with s
} // Memory is deallocated.
```

> Deallocation in Rust is reffered to as `drop`

```Rust
let s1 = String::from("Hello");
let s2 = s1;
```

> #### a breakdown of how the `s1` variable works.
>
> s1 is composed of 3 parts:
>
> - pointer - _the memory location in the heap_
> - length - _this is the memory size being used by the variable content in bytes_
> - capacity - _this is the memory allocated by the allocator_

> When we assign `s1` to `s2`, the `String` type is copied meaning we copied the pointer, capacity and length which are stored in the stack but not data in the heap.
> As the rule goes when an owner goes out of scope Rust calls `drop` to deallocate the contents in the heap. But in this given state where `s1` and `s2` are both pointing to the same location, when they go out of scope Rust will try dropping them which will result in _double free_ error.

> So as to prevent such errors when we set ` let s2 = s1` `s1` will no longer be valid (goes out of scope) thus no need of free but just only freeing `s2`.
> The concept of copying a pointer, capacity and length without the data is known as _shallow coppying_ but in Rust since it ivalidates the first value we call it _move_ > _Rust will never automatically create 'deep' copies of our data._

#### Ways in Which Variables nad Data Interact: `clone`

If we want to deeply copy the data of the heap and not just the stack data of `String` we can use common method called `clone`.

```Rust
let s = String::from("string");
let s1 = s.clone();

println!("{} {}", s , s1)
```

This will work perfectly as the heap data is _copied_ and not _moved_.
When you see a call to `clone`, you know that some arbitray code is being executed and the code might be expensive.
It is a visual indicator that something else is going on.

#### Stack-only-data: `Copy`.

`Copy` trait can be placed only on data that is stored in the stack such as integers. You wont use `Copy` trait on types that have implemented the `Drop`trait
When a type implements the `Copy` trait variables that uses it don't move but are trivially coppied, thus still vallid for assigment to another variable.

## Ownership and Functions

Passing values to a function behaves the same way as assigning values; They can be copied or moved. i.e;

```Rust
fn main(){
    let s = String::from("Hello"); // s comes into scope.
    take_ownership(s) // s is moved to s1 and becomes invalid.

    let x = 5;  // x comes into scope.
    copy_it(x) // x is copied to y.
}

fn take_ownership(s1: String){
    println!("{}", s1);
} // s1 goes out of scope and Drop occurs.

fn copy_it(y: i32){
    println!("{}", y);
} // y goes out of scope.
```

#### Return Values and scope.

Returning values can also transfer ownership. i.e.

```Rust
fn main(){
    let s1 = give_ownership(); // n string to s

    let s2 = String::from("data");

    let s3 =take_and_give(s2); // moves s2 to s to s3
}

fn give_ownership() -> String{
    let n_string = String::from("string");
    n_string
}

fn take_and_give(s: String) -> String(){
    s
}
```

# References and borrowing

**Reference** is like a pointer; in that its an address we can follow to access data stored in that address; only that the data is owned by another variable.

```Rust
fn main(){
    let s1 = String::from("data");
    calc_len(&s1); //&s1 makes a reference of s1 without taking ownership meaning
    // s is a reference of s1
}

fn calc_len(s: &String) -> usize{ // it takes a reference of type String.
    s.len()
}
```

**_Borrowing_**: This is the act of referencing.

> We are not allowed to mutate what has been borrowed just like in variables.

###### How to create mutable references

```Rust
fn main(){
let mut s = String::from("data"); // Make s mutable

 change(&mut s); // reference a mutable string
}
fn change(s1: &mut String) -> String{ // take mutable reference.
    s1.push_str(", string"); // mutate the reference.
}
```

Mutable references have one big downside such that you can only refer to it only once meaning:

> You can have as many immutable references as you want but you can only have one mutable reference.

I.e.

```Rust
let mut s = String::from("data");

let a = &mut s;
let b = &mut s;

```

👆🏻 doing this will result to error `You cannot borrow 's' as mutable more than once.

> The benefit of having such a restriction is that it prevent _data races_ at compile time.
> When does a data race occur:
>
> - Two or more pointers access the same data at a time.
> - At least one of the pointers is being used to write the data
> - There is no mechanism used to synchronize access to data.

You can have multiple mutable refrences but in different scopes.
If you borrow a value as immutable you cannot borrow it as a muttable. (and virce versa).

> A reference's scope starts from when it's introduced and ends where it's last used. For instance:

```Rust
let mut r = String::from("data");
let a = &r;
let b = &r;
println!("{} {}", a, b); //The variables a and b are out of scope

let c = &mut r;
```

> The ability of a compiler to know when a reference is no longer in use is called _Non-lexical Lifetimes_ (NLL).

### Dangling Reference

_Dangling pointer_ this is a pointer that references a location in memory that might have been given to some one else by freeing some memory while preserving a pointer to that memory.
The Rust compiler prevents such instance: if you have a reference to some data the compiler will ensure that the data will not go out of scope before the reference to the data does.

```Rust
fn dangle() -> &String{ // Returns a String referenc
    let s = String::from("data"); //create String
    &s // Reterun reference of s.
} // s goes out of scope and its droped.
//compiler complains.
```

So as to resolve this we return `String` thus no reference will be made bot the value will be moved (no droping).

> - **NOTE** _References must always be valid_

## The slice type.

_Slices_: they let you reference a contigous sequence of elements in a collection rather than the whole collection.

#### String slices

This is a reference to part of a `String`.

```Rust
let s = String::from("data types");
let data = &s[0..4];
let types = &s[5..9];

```

Rather than reference to the whole string, we reference only some part of the string using the extra `[0..5]`.

###### String literals are slices

```Rust
let s = "some data"
```

The type of `s` is `&str`; its a slice pointing to a specific point of the binary. This is why string literals are immutable:`&str` is an immutable reference.

> We can use `&str` as parameters in functions taking `&String` since it offers flexibity to receive both types. This is due to a feature known as _deref coercions_

### Other slices

slices are also aplicable to arrays; i.e.

```Rust
let a = [1, 2, 3, 4, 5]

```

we can refer to some part of the array by:

```Rust
let b = &a[2..4]
assert_eq!(b,[3,4])
```
