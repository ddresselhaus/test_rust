

pub fn fizzbuzz() {
   
    let mut fizz: Vec<String> = vec![];

    for i in (1..100) {
        let x: String = i.to_string();
        if (i % 5 == 0) & (i % 3 == 0) {
            fizz.push("fizzbuzz".to_string());
        } else if i % 5 == 0 {
            fizz.push("buzz".to_string());
        } else if i % 3 == 0 {
            fizz.push("fizz".to_string());
        } else {
            fizz.push(x.to_string());
        }
    }

    for i in fizz {
        println!("{}", i);
    }

}
