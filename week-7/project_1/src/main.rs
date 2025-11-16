use std::io;

fn main() {
    loop{

        let mut user_choice1 = String::new();
        let mut user_shape1 = String::new();
        let mut trapeziumheight = String::new();
        let mut trapeziumbase1 = String::new();
        let mut trapeziumbase2 = String::new();
        let mut rhombus_diagonal1 = String::new();
        let mut rhombus_diagonal2 = String::new();
        let mut parallelogram_base = String::new();
        let mut parallelogram_altitude = String::new();
        let mut cube_length1 = String::new();
        let mut cube_length2 = String::new();
        let mut cylinder_radius = String::new();
        let mut cylinder_height = String::new();

        println!("What would you like to calculate area or volume?");
        io::stdin().read_line(&mut user_choice1).expect("invalid string");
        let user_choice1 = user_choice1.trim().to_string();

        println!("select a shape:
                1. trapezium
                2. rhombus
                3. parallelogram
                4. cube
                5. cylinder
                ");
        io::stdin().read_line(&mut user_shape1).expect("invalid string");
        let user_shape1 = user_shape1.trim().to_string();

        if user_choice1 == "area" {
            if user_shape1 == "trapezium" {
                println!("The area of a trapezium is (a + b)/2  X h where h is height a is base1 and b is base2
                            enter your value for height");
                io::stdin().read_line(&mut trapeziumheight).expect("invalid string");
                let trapeziumheight:f64 = trapeziumheight.trim().parse().expect("invalid input");

                println!("enter your base1");
                io::stdin().read_line(&mut trapeziumbase1).expect("invalid string");
                let trapeziumbase1:f64 = trapeziumbase1.trim().parse().expect("invalid input");

                println!("enter base2");
                io::stdin().read_line(&mut trapeziumbase2).expect("invalid string");
                let trapeziumbase2:f64 = trapeziumbase2.trim().parse().expect("invalid input");

                let trapezium_area:f64 = trapeziumheight * ((trapeziumbase1 + trapeziumbase2) / 2.00);

                println!("the area of the trapezium is {}", trapezium_area);
                break;

            }else if user_shape1 == "rhombus" {
                println!("The area of a rhombus is 0.5 x diagonal1 x diagonal2 enter your value for diagonal1");
                io::stdin().read_line(&mut rhombus_diagonal1).expect("invalid string");
                let rhombus_diagonal1:f64 = rhombus_diagonal1.trim().parse().expect("invalid input");

                println!("enter your value for diagonal2");
                io::stdin().read_line(&mut rhombus_diagonal2).expect("invalid string");
                let rhombus_diagonal2:f64 = rhombus_diagonal2.trim().parse().expect("invalid input");

                let rhombus_area:f64 = 0.5 * rhombus_diagonal1 * rhombus_diagonal2;

                println!("the area of the rhombus is {}", rhombus_area);
                break;
            }else if user_shape1 == "parallelogram" {
                println!("the formula for a parallelogram is base x altitude, input your value for base");
                io::stdin().read_line(&mut parallelogram_base).expect("invalid string");
                let parallelogram_base:f64 = parallelogram_base.trim().parse().expect("invalid input");

                println!("enter your value for altitude");
                io::stdin().read_line(&mut parallelogram_altitude).expect("invalid string");
                let parallelogram_altitude:f64 = parallelogram_altitude.trim().parse().expect("invalid altitude");

                let parallelogram_area:f64 = parallelogram_base * parallelogram_altitude;
                println!("the area of the parallelogram is {}", parallelogram_area);
                break;

            }else if user_shape1 == "cube" {
                println!("the formula for a cube is 6 x (length)^2 enter your value for length");
                io::stdin().read_line(&mut cube_length1).expect("invalid string");
                let cube_length1:f64 = cube_length1.trim().parse().expect("invalid input");

                let cube_area:f64 = 6.00 * (cube_length1).powf(2.0);
                println!("the area of the cube is {}", cube_area);
                break;

            }else if user_shape1 == "cylinder" {
                println!(" you cant calculate the area for this shape");
            }
        }else if user_choice1 == "volume" {
            println!(" the only shapes with a formula for volume are the cube and the cylinder pick one");
            if user_shape1 == "cube" {
                println!(" the volume of a cube is length^3 enter your value for length");
                io::stdin().read_line(&mut cube_length2).expect("invalid string");
                let cube_length2:f64 = cube_length2.trim().parse().expect("invalid input");

                let cube_volume:f64 = cube_length2.powf(3.0);
                println!("the volume of the cube is {}", cube_volume);
                break;

            }else if user_shape1 == "cylinder" {
                println!("the volume of a cylinder is pi x radius^2 x height, enter your value for radius");
                io::stdin().read_line(&mut cylinder_radius).expect("invalid string");
                let cylinder_radius:f64 = cylinder_radius.trim().parse().expect("invalid input");

                println!("enter your value for height");
                io::stdin().read_line(&mut cylinder_height).expect("invalid string");
                let cylinder_height:f64 = cylinder_height.trim().parse().expect("invalid input");

                let cylinder_volume:f64 = 3.14 * cylinder_radius.powf(2.0) * cylinder_height;
                println!("the volume of the cylinder is {}", cylinder_volume);
                break;

            }else if user_shape1 == "trapezium" {
                println!(" you cant calculate the volume for this shape");
            }else if user_shape1 == "rhombus" {
                println!(" you cant calculate the volume for this shape");
            }else if user_shape1 == "parallelogram" {
                println!(" you cant calculate the volume for this shape");
            }
        }
    }
}