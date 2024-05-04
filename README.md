# FunJS
FunJS is a simple JavaScript runtime for educational purposes. It's based on `deno_core`.

## `funjs` module

### File I/O
The `funjs.fs` module contains functions for performing basic file I/O.
```js
var content = funjs.fs.readFile("README.md");
funjs.fs.writeFile("README.md.copy", content);

// List files in a directory
var items = funjs.fs.listDir("src");
console.log(items);
```

### Math
```js
var nums = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

funjs.math.min(nums); // 1
funjs.math.max(nums); // 10
funjs.math.sum(nums); // 55
```

### Random
```js
funjs.random.range(1, 10);       // exclusive range
funjs.random.range(1, 10, true); // inclusive range
```

### Runtime
```js
console.log(funjs.runtime.version()); // Displays the version of this crate
```