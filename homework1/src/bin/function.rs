pub fn function1() {
    call_me()
}
fn call_me() {
    println!("Call me!")
}


pub fn function2() {
    call_me(3);
}

fn call_me(num:u8) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

pub fn function3() {
    call_me(10);
}

fn call_me(num: u32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

pub fn function4() {
    let original_price = 51;
    println!("Your sale price is {}", sale_price(original_price));
}

fn sale_price(price: i32) -> i32 {
    if is_even(price) {
        price - 10
    } else {
        price - 3
    }
}

fn is_even(num: i32) -> bool {
    num % 2 == 0
}

pub fn function5() {
     let answer = square(3);
    println!("The square of 3 is {}", answer);
}

fn square(num: i32) -> i32 {
    return num * num;
}