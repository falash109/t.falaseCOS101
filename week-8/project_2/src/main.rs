fn main(){
    let dev1 = ("Onyekachi", 8);
    let dev2 = ("Chidindu", 6);
    let dev3 = ("Tunde", 7);

    let mut most_years = dev1;

    if dev2.1 > most_years.1{
        most_years = dev2;
    }
    if dev3.1 > most_years.1{
        most_years = dev3;
    }
    println!("The developer with the highest years is {} with {} years", most_years.0,most_years.1 );
}