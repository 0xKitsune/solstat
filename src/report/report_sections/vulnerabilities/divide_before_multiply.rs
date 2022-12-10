pub fn report_section_content() -> String {
    String::from(
        r##"
Consider ordering multiplication before division to avoid loss of precision because integer division might truncate.

#### Unsafe Division
```js
n = 5 / 2 * 4; // n = 8 because 5 / 2 == 2 since division truncates.
```
#### Safe Division
```js
n = 5 * 4 / 2; // n = 10
```

"##,
    )
}
