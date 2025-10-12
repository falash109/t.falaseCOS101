fn main() {
	let t:f64 = 900000.00;
	let m:f64 = 1500000.00;
	let h:f64 = 2250000.00;
	let d:f64 = 8550000.00;
	let a:f64 = 250000.00;
	let s = t + m + h + d + a;
	println!("Total is {}", s);
	let q:f64 = 10.00;
	let av = (t + m + h + d + a ) / q;
	println!("average is {}", av)

}