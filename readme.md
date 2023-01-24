# About SMPL
SMPL is primitive math programming language under development.

# Syntax
All expressions end with a semicolon.

## `Variable Declaration`
You can declare a variable by using `define` keyword:
```js
define a = 2;
```
## `If Statement`
Conditional constructs in SMPL are similar to conditional constructs in JavaScript:
```js
if (cond) {
    // body
}
```
Also you can use `else` branch:
```js
if (cond) {
    // body
} else {
    // else body
}
```
Warning! Language does not yet support else if, attempting to use this syntax will result in an error.

## `While loop`
While loop is also similar to while loop in js
```js
while (cond) {
    // body
}
```

## `Function Declaration`
You can declare a function by using `function` keyword:
```js
function name(arg1, arg2) {
    // body
}
```

Use return statement for exit from function:

```js
function add(a, b) {
    return a + b;
}
```

you can see more examples [here](https://github.com/demetryf/smpl/tree/main/examples)