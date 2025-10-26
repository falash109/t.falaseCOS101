use std::io;

//values inputing
fn main() {
    let mut a = String::new(); 
    let mut b = String::new();
    let mut c = String::new();

    println!("a = ");
    io::stdin().read_line(&mut a).expect("Not a valid string");
    let a:f64 = a.trim().parse().expect("An error occured");

    println!("b = ");
    io::stdin().read_line(&mut b).expect("Not a valid string");
    let b:f64 = b.trim().parse().expect("An error occured");

    println!("c = ");
    io::stdin().read_line(&mut c).expect("Not a valid string");
    let c:f64 = c.trim().parse().expect("An error occured");

    //Finding the discriminant
     let discriminant = b*b - (4.0 * a * c);

    //quadratic equation
    let root1 = -b + discriminant.sqrt() - (4.0 * a * c)/2.0 * a;
    let root2 = -b - discriminant.sqrt() - (4.0 * a * c)/2.0 * a;
    let rroot = -b / 2.0 * a;

    

    //number of distinct roots
    if discriminant > 0.0{
        println!("There are two roots which are {}, {}",root1,root2);
    }
    else if discriminant == 0.0{
        println!("There is one real root which is{}",rroot);
    }
    else{
        println!("There are no real roots");
    }
}