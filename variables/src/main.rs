fn main() {
    const X: u8 = 5;
    println!("The constant has a value of {X}");

    // Working with tuples
    let patient_data = ("Patient 1", 20, "Flu");
    let (name, age, disease) = patient_data;
    println!("The patient's name is {name}: They are {age} years old and are sick with {disease}");
}
