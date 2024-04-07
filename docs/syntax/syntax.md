#Welcome to bline's syntax explained

Variables can be defined in 2 ways
let creates a constant variable and mut a mutable one
```
let foo: Int = 10;
mut foo: Int = 10;
```
to mutate a variable
```
foo = 20;
```

Ints can be defined in 2 ways
```
let foo: Int = 10;
let foo: Int = 100_000;
```

Flos can be defined in 2 ways
```
let foo: Flo = 1.0;
let foo: Int = .1;
```

Strs can be defined in 2 ways
```
let foo: Str = "Hello, world!";
let foo: Str = 'Hello, world!';
```

Boos can be defined in 2 ways
```
let spooky: Boo = true;
let spooky: Boo = false;
```

Arrs can be defined in 1 way and you must specify its type
```
let foo: Arr<Int> = [1, 2, 3];
let foo: Arr<Str> = ["Hello", "world", "!"];
```

Nuls can only be defined in 1 way
```
let foo: Nul = null;
```

Functions can be defined in 1 way
```
func foo(param: Int): Int {
    ret param * 2;
}
```

Functions can be called in 1 way
```
foo(10);
```

While loops can be defined in 1 way
```
while !false {
    if true {
        brk;
    }
}
```

For loops can be defined in 1 way
```
for let i = 0; i < 10; i += 1; {
    if true {
        brk;
    }
}
```

If statements can be defined in 3 ways
```
if true & false {
    //
} elseif true == true | true != false {
    //
} else {
    //
}
```

Here are all reserved keywords:
```
func
brk
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
Int
Str
Flo
Boo
Arr
Nul
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
Int: Integer
Flo: Float
Boo: Boolean
Str: String
Arr: Array // Arrays are generic Arr<T>
Nul: Null
```

