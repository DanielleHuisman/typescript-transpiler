const a = 1;
const b = 2;
const c = 3;

if (a === b) {
  console.log("equal to b");
} else if (a === c) {
  console.log("equal to c");
} else {
  console.log("not equal to b or c");
}

console.log("while");
let d = 10;
while (d >= 0) {
  console.log(d);
  d--;
}

console.log("do while");
d = 10;
do {
  console.log(d);
  d--;
} while (d >= 0);

console.log("for");
for (let i = 10; i >= 0; i -= 2) {
  console.log(i);
}
