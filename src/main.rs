use std::io;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};


// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - >
// Pseudo-random unsigned 32-bit number generator
struct PseudoRNG {
    seed: u32,
}

impl PseudoRNG {
    // New instance of PRNG with current time in ms as the seed
    fn new() -> Self {
        let mut prng = Self { seed: 0 };
        prng.randomize();
        prng
    }

    fn randomize(&mut self) {
        let millis = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Looks like time went backwards!")
            .as_millis();
        self.seed = millis as u32;
    }

    // Return a pseudorandom value in the range [0, 2147483647].
    fn random_u32(&mut self) -> u32 {
        self.seed = self.seed.wrapping_mul(1_103_515_245).wrapping_add(12_345);
        self.seed %= 1 << 31;
        self.seed
    }

    // Return a pseudorandom value in the range [0.0, 1.0).
    fn random_f64(&mut self) -> f64 {
        let f = self.random_u32() as f64;
        f / (2147483647.0 + 1.0)
    }

    // Return a pseudorandom value in the range [min, max).
    fn random_i32(&mut self, min: i32, max: i32) -> i32 {
        let range = (max - min) as f64;
        let result = min as f64 + range * self.random_f64();
        result as i32
    }
}


// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - >


fn not_empty<T>(vec: &Vec<T>) -> bool {
    vec.len() > 0
}


// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - >
// Print at most num_items items from an input vector of i32
fn print_vec(vec: &Vec<i32>, num_items: i32) {
    let max = std::cmp::min(vec.len(), num_items.try_into().unwrap());
    let mut string = String::new();
    string.push_str("[");

    if not_empty(vec) {
        string.push_str(&vec[0].to_string());
    }

    for i in 1usize..max {
        string.push_str(" ");
        string.push_str(&vec[i].to_string());
    }

    string.push_str("]");
    println!("{string}");
}


// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - >


fn make_random_vec(capacity: usize, min_value: i32, max_value: i32) -> Vec<i32> {
    let mut rng: PseudoRNG = PseudoRNG::new();
    let mut vec: Vec<i32> = Vec::with_capacity(capacity);

    for _ in 0..capacity {
        vec.push(rng.random_i32(min_value, max_value));
    }

    vec
}


// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - >


fn get_i32(prompt: &str) -> i32 {
    // Prompt the user for input and clear the output stream
    print!("{prompt}");
    io::stdout().flush().unwrap();

    // Read the input from the user into a string
    let mut str_value = String::new();
    io::stdin()
        .read_line(&mut str_value)
        .expect("Error reading input");

    // Remove any leading / trailing whitespace and attempt to
    // parse the input as a 32-bit integer
    let trimmed = str_value.trim();
    trimmed.parse::<i32>().expect("Error parsing integer")
}


// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - >
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - >


fn bubble_sort(vec: &mut Vec<i32>) {
    let mut performed_swap: bool = false;
    let mut unsorted: bool = true;

    // Continue looping as long as the vector remains unsorted
    while unsorted {
        // Iterate through the values in the vector, beginning at 1
        for i in 1..vec.len() {
            // Check if the values are in the wrong order - i.e., whether
            // a value is larger than the value to its right
            let left: i32 = vec[i - 1];
            let right: i32 = vec[i];

            // If it is, then we need to swap these elements (simply use the built-in .swap() method)
            // Track the fact that we had to perform a swap here
            if left > right {
                vec.swap(i - 1, i);
                performed_swap = true;
            }
        }

        // If we finished looping over all elements without swapping any, then we know
        // that the vector must be sorted. In that case, set unsorted to false which
        // will exit the while-loop
        if !performed_swap {
            unsorted = false;
        }

        // Otherwise we did have to swap something on the previous iteration and so need
        // to loop again - so, reset performed_swap to false
        performed_swap = false;
    }
}


// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - >


fn verify_sorted(vec: &Vec<i32>) -> bool {
    // If there are fewer than 2 elements, then the vector is technically sorted
    if vec.len() < 2 {
        return true;
    }

    // At least two or more elements: iterate through the vector and check whether any
    // elements are greater than the element to their right. If such a value is found,
    // then the vector is not sorted, so return false immediately (no point checking
    // the rest of the vector)
    for i in 1..vec.len() {
        if vec[i - 1] > vec[i] {
            return false;
        }
    }

    // If we exited the previous loop without finding out-of-order elements then
    // we're sorted, so return true
    true
}


// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - >
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - >


fn main() {
    let max: i32 = 999;
    let prompt: String = String::from("Enter number of iterations: ");
    let capacity: i32 = get_i32(&prompt);
    let mut values: Vec<i32> = make_random_vec(capacity as usize, 0, max);
    
    // Some other test values - can be removed now
    //let testvals = [2, 3, 3, 5, 6, 8, 9, 10, 12, 20];
    //let testvec: Vec<i32> = testvals.iter().map(|&x| x as i32).collect();

    print_vec(&values, 10);
    bubble_sort(&mut values);
    let is_sorted: bool = verify_sorted(&values);
    print_vec(&values, 10);
    println!("\nIs this vector sorted?\n  > {is_sorted}");
}


// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - >