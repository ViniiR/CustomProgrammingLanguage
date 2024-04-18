# Recommended conventions for bline

Indentation
```
all indentation should be done with 4 spaces, not tabs and not 2 spaces
```

Empty spaces
```
all functions should have one empty line between them
```

Spaces between expressions
```
wrong:

let foo:i32=10;
if true!=false{
  //
};
func bar ():i32{
ret ( 10 + 10 );
};

correct:

let foo: i32 = 10;
if true != false {
    //
};
func bar(): i32 {
    ret (10 + 10);
};
```
