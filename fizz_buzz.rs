
fn print_fizz_buzz(v: i32) -> bool {
    if v % 3 == 0 && v % 5 == 0 {
        println!("FizzBuzz!");
        return true;
    }

    return false;
}

fn print_buzz(v: i32) {
    if v % 5 == 0 {
        println!("Buzz!");
    }
}

fn print_fizz(v: i32) {
    if v % 3 == 0 {
        println!("Fizz!");
    }
}

fn print_all(v: i32) {
    
    if print_fizz_buzz(v) == false {
        print_fizz(v);
        print_buzz(v);
    }  
}

fn main() {
    println!("Let's do some FizzBuzz!");

    for i in 1..100 {
	println!("Checking number {}", i);
	print_all(i);
    }
}
