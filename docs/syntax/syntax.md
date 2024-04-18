# Welcome to bline's syntax explained

in bline all statements must end if a semicolon

Variables can be defined in 3 ways
let creates a constant variable and mut a mutable one
```
let foo: i32 = 10;
mut foo: i32 = 10;
mut foo: i32;
```
to mutate a variable
```
foo = 20;
```

i32s can be defined in 2 ways
```
let foo: i32 = 10;
let foo: i32 = 100_000;
```

f64s can be defined in 2 ways
```
let foo: f64 = 1.0;
let foo: i32 = .1;
```

strs can be defined in 2 ways
```
let foo: str = "Hello, world!";
let foo: str = 'Hello, world!';
```

bools can be defined in 2 ways
```
let spooky: bool = true;
let spooky: bool = false;
```

vecs can be defined in 1 way and you must specify its type
```
let foo: vec<i32> = [1, 2, 3];
let foo: vec<str> = ["Hello", "world", "!"];
```

nulls can only be defined in 1 way
```
let foo: null = null;
```

Functions can be defined in 1 way
```
func main(argc: i32, argv: vec<str>): i32 {
    ret 0;
};
```

Functions can be called in 2 way
```
foo(10);
let bar: i32 = foo();
```

While loops can be defined in 1 way
```
while !false {
    if true {
        brk;
    };
};
```

For loops can be defined in 1 way
```
for let i = 0; i < 10; i += 1; {
    if true {
        brk;
    };
};
```

If statements can be defined in 3 ways either with or without elseif or else
```
if true & false {
    //
} elseif true == true | true != false {
    //
} else {
    //
};
```

Here are all reserved keywords:
```
func
brk
cnt
ret
let
if
elseif
else
while
for
true
false
null
i32
str
f64
bool
vec
null
```

Here are all operators:
```
= assign
+= assign plus
-= assign minus
*= assign multiplied
/= assign divided
%= assign rest
+ plus
- minus
* multiply
/ division
% rest of division

! not
& and
| or
== equals
!= different
< smaller than
> greater than
<= smaller or equals than
>= greater of equals than
```

Here are all valid punctuation characters:
```
. dot
( left paren
) right paren
[ left square bracket
] right square bracket
{ left curly braces
} right curly braces
" double quotes
' single quotes
; semicolon
: colon
, comma
```

Here are all available DataTypes:
```
i32: integer
f64: Float
bool: Boolean
str: String
vec: vector // vectors are generic vec<T>
null: null
```

