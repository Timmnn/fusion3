# FusionLang

## Features

### Semicolons

Semicolons arent needed, only when you want to write multipl expressions in the same line. Same with commas.

```fusion
a := 5; b:= 6; println("Hello World")
c := 1
d := 2
```

### Variables

Variables are declared with ':='. Allowed names are similar like in other languages. The variable type can be infered. With casting it can be specified manually. Variables are assigned with '='. Only mutable variables prefixed with 'mut' can be changed after declaration. Variables can be constant which is only possible for global variables. constant variables have their value assigned at compile time.

```fusion
a := 5
const GLOBAL_VAR := "Hello World"
mut b := 78 as i64
a = 6 // This won't compile
b = 999 // This works
```

### Functions

Functions work like in the most languages. The return type can be infered but can also be pinned to a specific type. A value is returned with the 'return' keyword. A function can be marked as const, which makes every call run just run once at compile time. They can only be called with parameters which can be evaluated at compile time.

```fusion
fn hello_world() {
    println("Hello World")
}

const fn fib() i32 {
    //omitted
}
```

### Traits

Traits define functions that a type can implement. Each trait function may have a default implementation that is used, if the implementation doesnt define another one.

```fusion
trait ToString {
    toString(&self) string
    debug(&self) string {
        return "debug"
    }
}

struct test = {
    value: string
}

impl ToString for test {
    toString(&self) {
        return self.value
    }
}

```

### Structs

Structs are just like classes but without inheritance. Instead they can be composed with Traits.

```fusion

struct test = {
    value: string,
    x = 5 // Default value
}

x := test{
    value: ""
    x = 10
}

y := test{
    value: ""
}
```

### Enums

### Generics

#### Functions

#### Structs

#### Enums

#### Traits
