fn create_person() {
    struct Person {
        name: String,
        age: i32,
        }
    let name = "Alice";
    let age = 30;
    let person = Person { name, age };
    return person;
    }
fn print_person_info() {
    let person = create_person();
    println!("Name: " + person.name);
    println!("Age: " + person.age.to_string());
    }
fn array_test() {
    let numbers = [10, 20, 30, 40, 50];
    for num in numbers {
        println!(num);
        }
    }
