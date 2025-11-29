fn main() {
    
    //creating a new vector
    let _v: Vec<i32> = Vec::new();

    let mut v = vec![11, 12, 13];

    //updating the vector
    v.push(14);
    v.push(15);

    //Reading elements from the vector
    let second: &i32 = &v[1];

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is : {}", third),
        None => println!("There is no third element")
    }

    //printing the vector
    println!("{:?}", v);
    println!("{:?}", second);
    println!("{:?}", third);


    let mut a = vec![2, 3, 4, 5];
    a.push(6);
    

    let first =  &a[0];


    println!("The first element is {} and vector is {:?}", first, a);

    let b = vec![100, 32, 57];
    for i in &b {
        println!("{}", i);
    };

    let mut c = vec![10, 20, 30, 40, 50];
    for i in &mut c {
        *i *= 50;
        println!("{}", i);
    }

    let row = vec![
        Excelsheet::Int(32),
        Excelsheet::Float(45.6),
        Excelsheet::Text(String::from("Hello")),
    ];

    for i in &row {
        match i {
            Excelsheet::Int(value) => println!("Integer: {}", value),
            Excelsheet::Float(value) => println!("Float: {}", value),
            Excelsheet::Text(value) => println!("Text: {}", value),
        }
    }   

    let mut s = String::new();
    s.push_str("Hello");
    s.push(',');
    s.push_str(" World!");
    println!("{}", s);

    let payload = "This is a sample string";
    let mut s1 = payload.to_string();
    s1.push_str(" with more content.");
    println!("{}", s1);

    let data = String::from("Initial content");
    println!("{}", data);

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s2 + &s1;
    println!("{}", s3);
    println!("{}", s1);
  //  println!("{}", s2);  error because s2 has been moved

  let s1 = String::from("tic");
  let s2 = String::from("tac");
  let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);  

    //indexing into strings

    let hello = "Здравствуйте";
    //let s = hello[0];  // this will give error because string is not indexed by integer
    let s = &hello[0..6]; //slicing is allowed
    println!("{}", s);

    for c in hello.chars() {
        println!("{}", c);
    }

    for c in hello.bytes() {
        println!("{}", c);
    }

    //creating hashmap
    use std::collections::HashMap;

    let mut fruits = HashMap::new();
    fruits.insert(String::from("Apple"), 3);
    fruits.insert(String::from("Banana"), 5);

    //accessing values
    let apple_count = fruits.get("Apple");
    match apple_count {
        Some(&count) => println!("Number of apples: {}", count),
        None => println!("No apples found"),
    }
    //iterating over hashmap
    for (key, value) in &fruits {
        println!("{}: {}", key, value);
    }

    let fruit_name = String::from("Apple");
    let count = fruits.get(&fruit_name).copied().unwrap_or(0);
    println!("Count for {}: {}", fruit_name, count);

    let name = String::from("hello world");
    let value1 = String::from("jira");
     
    let mut scar = HashMap::new();
     scar.insert(name, value1);
   //  println!(name);  This will cause a compile-time error because 'name' has been moved
     // println!(value1);  This will cause a compile-time error because 'value1' has been moved

     for (key, value) in &scar {
        println!("{}: {}", key, value);
     }

     let mut take = HashMap::new();
        take.insert(String::from("one"), 1);
        take.insert(String::from("one"), 2);
        println!("{:?}", take);

    let mut wake = HashMap::new();
    wake.insert(String::from("blue"), 10);

    wake.entry(String::from("yellow")).or_insert(50);
    wake.entry(String::from("blue")).or_insert(50);

    println!("{:?}", wake);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);


}

    //using enum to store multiple types
enum Excelsheet {
    Int(i32),
    Float(f64),
    Text(String),
}
