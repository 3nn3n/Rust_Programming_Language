fn main() {
    let mut counter: i32 = 0;
    let result: i32 = loop{
        counter = counter + 1;
        println!("Hello, world!");
        if counter == 30 {
            println!("The value of counter is {counter}");
            break counter*2;
        }
        };
    println!("The result is {result}");
    looper();
    lift_off();
    array_loop();
    loop_for();
    countdown_for_loop();
    temprature_conversion(100);
    fibonacci_number(9);
}

fn looper() {
    let mut count = 0;
    'upward_counting: loop {
        println!("the count is {count}");
        let mut remaining = 10;

        loop {
            println!("The remaining is {remaining}");

            if remaining == 3 {
                break;
            }

            if count == 5 {
                break 'upward_counting;
            }
            remaining = remaining - 1;
        }

        count = count + 1;
    }

    println!("End count is: {count}")
}


fn lift_off() {
    let mut number = 10;

    while number >= 0 {
        println!("{number}");
        number -= 1;
    }
    println!("ISRO PSLV Successful LIFTOFF >>>>>");
}


fn array_loop() {
    let a = [23, 43, 32, 67, 32, 7, 34];
    let mut index = 0;

    while index < 7 {
        println!("The value of {index}th index is: {}", a[index]);
        index += 1;
    }
}


fn loop_for() {
    let b = [45, 3, 25, 63, 76, 21, 86, 09];

    for value in b {
        println!("The value is {value}")
    }
}

fn countdown_for_loop() {
    for number in (0..11).rev() {
        println!("{number}");
    }
    println!("Successful PSLV LiftOff....");
}

fn temprature_conversion(f: i32) {
    let fahrenheit: i32 = f; 
    let celcius: i32 = (fahrenheit - 32) * 5/9;
    println!("The temperature in celcius is {celcius}")
}

fn fibonacci_number(n: i32) {
    let mut a1 = 0;
    let mut a2 = 1;
    let number : i32 = n;
    println!("first {number} fibonacci number are...");
    
    loop {
        let mut i = 0;
        while i < number {
            let x:i32 = a1 + a2;
            println!("{x} ");
            a1 = a2;
            a2 = x;
            i = i + 1;
        }
        break;
    }
}