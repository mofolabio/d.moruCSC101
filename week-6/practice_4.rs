fn main() {
    let fullname = "Chibudom John Umeh";
    let department = "Computer Science";
    let uni = "Pan-Atlantic University";

    let mut school = "School of Science".to_string();
    // Push string
    school.push_str(" and Technology");

    println!("My name is: {}", fullname);
    // Check length
    println!("The length of my fullname is: {}", fullname.len());
    println!("I am a student of {} Department", department);
    println!("{}", school);
    println!("{}", uni);
}
