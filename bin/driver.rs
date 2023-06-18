use uni_stud_info::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let usr_name = read_input("Enter your name: ")?;
    let usr_age: u32 = read_input("Enter your age: ")?.parse().unwrap_or_default();
    let usr_uni_name = read_input("Enter your university's name: ")?;
    let usr_gpa: f32 = read_input("Enter your GPA: ")?.parse().unwrap_or_default();
    let usr_inter_stat: bool = read_input("Do you qualify as an international student (yes/no): ")?.to_lowercase() == "yes";

    let student = UniStudent::new(usr_name, usr_age, usr_uni_name, usr_gpa, usr_inter_stat);
    println!("\nHere is the information that you have given me:");
    student.print_info();

    Ok(())
}