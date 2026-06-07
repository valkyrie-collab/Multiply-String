use std::{
    io::{
        self,
        Write,
    },
};

fn find_product(string_one: String, string_two: String) -> String {
    let mut sum_arr: Vec<u64> = vec![0; string_one.len() + string_two.len() + 1];
    let mut pro_string: String = String::new();
    let mut total: usize = 0;
    let mut carry: u64 = 0;

    for (xti, xtc) in string_two.chars().into_iter().enumerate() {

        for (xoi, xoc) in string_one.chars().into_iter().enumerate() {
            let pro: u64 = ((xoc as u8 - 48) * (xtc as u8 - 48)) as u64 + carry; // 32 + 0, 8 + 3, 36 + 1
            let rem: u64 = pro % 10; // 2 1 7
            carry = pro / 10; // 3 1 3

            total = xoi + xti;
            sum_arr[total] += rem;

            if sum_arr[total] > 9 {
                let sum: u64 = sum_arr[total];
                sum_arr[total] = sum % 10;
                sum_arr[total + 1] = sum_arr[total + 1] + sum / 10;
            }

        }

        sum_arr[total + 1] += carry;
        carry = 0;
        total = 0;
    }

    let mut xi: usize = sum_arr.len() - 1;
    let mut saw_digit: bool = false;

    loop {
        if sum_arr[xi] == 0 && !saw_digit {xi -= 1; continue;}
        saw_digit = true;

        if xi == 0 {
            pro_string.push((sum_arr[xi] as u8 + 48) as char);
            break;
        }

        pro_string.push((sum_arr[xi] as u8 + 48) as char);
        xi -= 1;
    }

    pro_string
}

fn remove_constant(num_one: String, num_two: String) -> String {

    if num_one == "0" {
        return num_one;
    }

    if num_one == "1" {
        return num_two;
    }

    if num_two == "0" {
        return num_two;
    }

    if num_two == "1" {
        return num_one;
    }

   find_product(num_one.chars().rev().collect(), num_two.chars().rev().collect())
}

fn main() {
    let mut buf: String = String::new();

    print!("Enter the first number: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buf).unwrap();
    let num_one: String = buf.clone();
    buf.clear();

    print!("Enter the second number: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buf).unwrap();
    let num_two: String = buf;

    // let num_one: String = String::from("123");
    // let num_two: String = String::from("456");

    let product_string: String = remove_constant(num_one, num_two);
    println!("Product String: {}", product_string);
}
