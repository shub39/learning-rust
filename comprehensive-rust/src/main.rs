fn main() {
    let n = 11;
    println!("fib {n} = {}", fibonacci(n));
    println!("Collatz Length of {n} = {}", collatz_length(n));

    let matrix = [
        [101, 102, 103],
        [201, 202, 203],
        [301, 302, 303],
    ];
    
    dbg!(matrix); // dirty debugging
    let transpose = transpose(matrix);
    dbg!(transpose);
    
}

// fibonacci using recursion
fn fibonacci(n: u32) -> u32 {
    if n < 2 {
        n
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

/*
The Collatz Sequence is defined as follows, for an arbitrary n1 greater than zero:

    If ni is 1, then the sequence terminates at ni.
    If ni is even, then ni+1 = ni / 2.
    If ni is odd, then ni+1 = 3 * ni + 1.

For example, beginning with n1 = 3:

    3 is odd, so n2 = 3 * 3 + 1 = 10;
    10 is even, so n3 = 10 / 2 = 5;
    5 is odd, so n4 = 3 * 5 + 1 = 16;
    16 is even, so n5 = 16 / 2 = 8;
    8 is even, so n6 = 8 / 2 = 4;
    4 is even, so n7 = 4 / 2 = 2;
    2 is even, so n8 = 1; and
    the sequence terminates.

 */
fn collatz_length(mut n: u32) -> u32 {
    let mut len = 1;
    while n > 1 {
        n = if n % 2 == 0 { n / 2 } else { n * 3 + 1 };
        len += 1;
    }
    len
}

// Transposes the matrix
fn transpose(matrix: [[u32; 3]; 3]) -> [[u32; 3]; 3] {
    let mut result = [[0; 3]; 3]; // another way to initialize arrays
    for i in 0..3 {
        for j in 0..3 {
            result[j][i] = matrix[i][j];
        }
    }
    result
}