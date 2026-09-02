fn main() {
    // Sales amounts
    let sales = [450000.0, 1500000.0, 750000.0, 2850000.0, 250000.0];

    // Calculate the sum
    let sum: f64 = sales.iter().sum();

    // Calculate the average
    let average = sum / sales.len() as f64;

    // Display the results
    println!("Sum of sales = {:.2}", sum);
    println!("Average sales = {:.2}", average);
}