fn main() {
    // Tuples
    let tup = (500, 6.4, 1);
    let (_, y, _) = tup;
    println!("The value of y is: {} and x is: {}", y, tup.0);

    // Arrays
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    println!("The first month is: {}.", months[0]);
}
