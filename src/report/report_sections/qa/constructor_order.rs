pub fn report_section_content() -> String {
    String::from(
        r##" 
### Constructor is placed after other functions

Constructor definition must be placed after Modifiers definitions and before any other function definitions in order to follow `Style Guide Rules`.

Ex:

Bad
```js
contract A {
    function () public {}
    constructor() {}
}
```

Good
```js
contract A {
    constructor() {}
    function () public {}
}
```
    "##,
    )
}
