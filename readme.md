# SMPL

SMPL (saturn math programming language) is an open source educational compiled language.

You can see examples [there](https://github.com/DemetryF/smpl/tree/main/examples).

# Quick Start

You need to have rust toolchain to use smplc, if you don't, go [there](https://rustup.rs/) and install it.

Use cargo install to install it from the repo to your system

```sh
cargo install --git https://github.com/demetryf/smpl
```

Or build and run it from the source:

```sh
git clone https://github.com/demetryf/smpl
cd smpl
cargo run --release -- <filename>
```

# Language overview

## Entry point

Since this is a compiled language, you should have an entry point to your program. It's what the \`main\` function is used for:

```rs
fn main() {
    // ...
}
```

A program in SMPL is built from items - constant or function declarations.

The following syntax lets you define a function:

```rs
fn <name>( {<arg> : <ty>} ) [-> <ret_ty>] {
    // body
}
```

Also, you can define constants:

```rs
const <name> : <ty> = <expr>;
```

In the expression of a constant, you can use previously defined constants, but cannot call functions

There's several statements that can be used everywhere in function blocks in SMPL:

## Declaring a variable

```rs
let <id> [: <ty>] [= <expr>];
```

Like any language, in SMPL you can declare your variable, specify it type and value.

Specifying of the type is unnecessary because of type inference. If the compiler couldn't infer the type, you'll should specify it explicit.

You can set the value of a variable later, but if you won't, you'll get an error within attempt to use it.

## Conditional statement

```rs
if <cond> {
    // then branch
}
```

And with else branch:

```rs
if <cond> {
    // then branch
} else {
    // else branch
}
```

## Loops

```rs
while <cond> {
    // body
}
```

In the language, loops are presented only by while loop, that firstly check the condition and then run the body code.

## Return

You can exit from the function and return some value:

```rs
return [<expr>];
```

## Built-in types

There's several built-in types:

- `bool` - boolean type that supports `&`, `|`, `!` operations.
- `int` - integer number type representing i32 and supporting arithmetic (`+`, `-`, `*`, `/`) and ordering (`>`, `>=`, `<`, `<=`) operations.
- `real` - floating point number type representing f32 and supporting the same operations as `int`.
- `complex` - complex number type represented as two of f32 and supporting the same operations as `real` except the ordering operations. As an imaginary postfix is used `i`.
- `vec2`, `vec3`, `vec4` - vector types represented as corresponding count of f32 and supporting `+`, `-` and multiplying/dividing on a scalar. To construct a vector, use function with the same name as vec type that you want your variable to have, e.g. `vec2(x, y)`. Also you can use swizzling to get access to a component or transform vector, e.g. `:x`, `:zyx`.

For each type there is a built-in function to print a value of it:
`printb`, `printi`, `printr`, `printc`, `printvec2`, `printvec3`, `printvec4` accordingly.
