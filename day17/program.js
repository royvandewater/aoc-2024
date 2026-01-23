// 2,4,1,1,7,5,4,4,1,4,0,3,5,5,3,0

let a = ?;
let b = 0;
let c = 0;

do {
  // only keep the 3 bits
  b = b % 8;
  b = b ^ 0b001;
  // drop 0-7 of the last bits
  c = a >> b;
  b = b ^ c;
  b = b ^ 0b010;
  // drop the last 3 bits
  a = a >> 3;
  console.log(b);
} while (a != 0)
