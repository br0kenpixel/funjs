console.log("hi");

var items = funjs.fs.listDir("src");
console.log(items);

var min = funjs.math.min([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
console.log(min);

var max = funjs.math.max([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
console.log(max);

var sum = funjs.math.sum([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
console.log(sum);

var rnd = funjs.random.range(1, 10);
console.log(rnd);

rnd = funjs.random.string(12);
console.log(rnd);

var ver = funjs.runtime.version();
console.log(ver);