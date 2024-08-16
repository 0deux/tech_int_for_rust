use std::thread;
use std::time::Duration;

fn main() {
    for i in 17..52 {
        match (i % 3, i % 5) {
            (0, 0) => println!("FizzBuzz"),
            (0, _) => println!("Fizz"),
            (_, 0) => println!("Buzz"),
            (_, _) => println!("{}", i),
        }
    }

    let name = "Barbaros";
    let surname = "Alagöz";

    println!("Adım {} {}! Memnun oldum", name, surname);

    let employee: HuaweiEmployee = HuaweiEmployee {
        name: "Stratos".to_string(),
        surname: "Stravos".to_string(),
        office_number: 46,
    };

    println!(
        "Selamlar, {} {}, ofis numaram {}!",
        employee.name, employee.surname, employee.office_number
    );

    let a: i32 = 5;
    let b: i32 = 10;

    let result = max(&a, &b);

    println!("Max value: {}", result);

    let original_owner: String = String::from("google");

    let new_owner: &String = &original_owner; // Borrowing occurs here

    println!("{}", original_owner);

    let current_signal = TrafficLight::Red;

    let current_status = can_cross(current_signal);

    println!("The current status is: {}", current_status);

    let fruit: Fruit = Fruit::Apple;

    match fruit {
        Fruit::Apple => println!("It's an apple!"),
        Fruit::Orange => println!("It's an orange!"),
        Fruit::Banana => println!("It's a banana!"),
    }

    let handle1 = thread::spawn(|| {
        for i in 1..5 {
            println!("Thread 1: {}", i);
            thread::sleep(Duration::from_millis(1)); // Thread'i 1 milisaniye beklet
        }
    });

    // İkinci thread
    let handle2 = thread::spawn(|| {
        for i in 1..5 {
            println!("Thread 2: {}", i);
            thread::sleep(Duration::from_millis(1)); // Thread'i 1 milisaniye beklet
        }
    });

    // Thread'lerin bitmesini bekle
    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("Main thread tamamlandı!");
}

struct HuaweiEmployee {
    name: String,
    surname: String,
    office_number: i32,
}

fn max<'a>(x: &'a i32, y: &'a i32) -> &'a i32 {
    if x > y {
        x
    } else {
        y
    }
}

enum TrafficLight {
    Red,
    Yellow,
    Green,
}

fn can_cross(signal: TrafficLight) -> &'static str {
    match signal {
        TrafficLight::Red => "Stop",
        TrafficLight::Yellow => "Ready",
        TrafficLight::Green => "Go",
    }
}

enum Fruit {
    Apple,
    Orange,
    Banana,
}
