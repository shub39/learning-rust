mod fibonacci;
mod collatz;
mod matrix;
mod vectors;
mod elevator;
mod expression_evaluater;

fn main() {
    let n = 11;
    println!("fib {n} = {}", fibonacci::fibonacci(n));
    println!("Collatz Length of {n} = {}", collatz::collatz_length(n));

    let matrix = [
        [101, 102, 103],
        [201, 202, 203],
        [301, 302, 303],
    ];
    
    dbg!(matrix); // dirty debugging
    let transpose = matrix::transpose(matrix);
    dbg!(transpose);

    let mut v = [1.0, 2.0, 9.0];
    println!("Magnitude of {v:?}: {}", vectors::magnitude(&v));
    vectors::normalize(&mut v);
    println!("Magnitude of {v:?} after normalization: {}", vectors::magnitude(&v));

    println!(
        "A ground floor passenger has pressed the up button: {:?}",
        elevator::lobby_call_button_pressed(0, elevator::Direction::Up)
    );
    println!("The car has arrived on the ground floor: {:?}", elevator::car_arrived(0));
    println!("The car door opened: {:?}", elevator::car_door_opened());
    println!(
        "A passenger has pressed the 3rd floor button: {:?}",
        elevator::car_floor_button_pressed(3)
    );
    println!("The car door closed: {:?}", elevator::car_door_closed());
    println!("The car has arrived on the 3rd floor: {:?}", elevator::car_arrived(3));
}