use rand::prelude::*;
use std::io;
use std::time::Instant;

fn main() {
    println!("\nSort comparison\n");

    loop {
        // Get list size
        println!("Please enter list size.\n");
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");

        let input = input.trim();

        match input {
            "exit" => break,
            &_ => match input.parse::<i32>() {
                Ok(num) => {
                    if num < 0 {
                        println!("Invalid input, list length must be at least 2.")
                    } else {
                        let base_list: Vec<i32> = gen_list(num);
                        {
                            let mut set = base_list.to_vec();
                            selection_sort(&mut set);
                        }

                        {
                            let mut set = base_list.to_vec();
                            insertion_sort(&mut set);
                        }

                        {
                            let mut set = base_list.to_vec();
                            let start_time = Instant::now();
                            set.sort();

                            println!(
                                "Inbuilt stable sort time: {}",
                                start_time.elapsed().as_millis()
                            )
                        }

                        {
                            let mut set = base_list.to_vec();
                            let start_time = Instant::now();
                            set.sort_unstable();

                            println!(
                                "Inbuilt unstable sort time: {}",
                                start_time.elapsed().as_millis()
                            )
                        }
                    }
                }
                Err(_) => {
                    println!("That's not a number!\n");
                    continue;
                }
            },
        }
    }
}

fn gen_list(size: i32) -> Vec<i32> {
    let mut rng = rand::thread_rng();

    let mut nums: Vec<i32> = (1..size).collect();
    nums.shuffle(&mut rng);
    nums
}

fn selection_sort<T: Ord>(list: &mut [T]) {
    let start_time = Instant::now();

    for i in 0..list.len() {
        // let mut small = i;
        // for j in (i + 1)..list.len() {
        //     if list[j] < list[small] {
        //         small = j;
        //     }
        // }
        // list.swap(small, i);

        if let Some((j, _)) = list.iter().enumerate().skip(i).min_by_key(|x| x.1) {
            list.swap(i, j);
        }
    }

    println!("Selection sort time: {}", start_time.elapsed().as_millis())
}

fn insertion_sort<T: Ord>(list: &mut [T]) {
    let start_time = Instant::now();

    for i in 1..list.len() {
        for j in (1..i + 1).rev() {
            if list[j - 1] <= list[j] {
                break;
            }
            list.swap(j - 1, j)
        }
    }

    println!("Insertion sort time: {}", start_time.elapsed().as_millis())
}
