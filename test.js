const css = `
.abc1 {
  background:url("https://www.baidu.com");
  background-color: rgba(253, 254, 255, 0.1);
}
.abc2 {
  background:url("https://www.baidu.com");
  background-color: rgba(253, 254, 255, 0.1);
}
`

console.time("postcss-rust js");
require(".").parse(css);
console.timeEnd("postcss-rust js");

console.time("postcss");
require("postcss").parse(css);
console.timeEnd("postcss");

