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

let d = 10;
while (d >= 0) {
  console.log(d);
  d--;
}

d = 10;
do {
  console.log(d);
  d--;
} while (d >= 0);

for (let i = 10; i >= 0; i--) {
  console.log(i);
}
