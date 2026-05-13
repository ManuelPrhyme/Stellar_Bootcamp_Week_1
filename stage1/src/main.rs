use std::io::{stdin,Write,stdout};

fn main() {
    println!("Stage 1");

    let mut Bills: Vec<(String,u64)> = Vec::new();
    let mut bill_key: String = String::new();
    let mut bil_value: String = String::new();
    let mut no_of_bills: String = String::new();
    let mut entry_counter: u8 = 0;

    println!("Number of bills:");
    stdin().read_line(&mut no_of_bills).expect("Can not read line");
    no_of_bills = no_of_bills.trim().to_string();
    let no_of_bills: u64 = no_of_bills.parse::<u64>().expect("Not a Number");

    while (entry_counter as u64) < no_of_bills {

        println!("\nBill {}:",entry_counter + 1);
        println!("Bill Name: ");
        stdin().read_line(&mut bill_key).expect("Failed to input bill");
        let bill_key_trimmed = bill_key.trim().to_string();
        println!("\nBill Amount:");
        stdin().read_line(&mut bil_value).expect("Failed to input amount");

        loop {
            bil_value = bil_value.trim().to_string();
            let bil_value_parsed = bil_value.parse::<u64>();

            match bil_value_parsed {
                Ok(value) => {
                    Bills.push((bill_key_trimmed.clone(),value));
                    break;
                },
                Err(_) => {
                    println!("Retry (Enter a number):");
                    stdin().read_line(&mut bil_value).expect("Failed to input amount");

                    let amount_parsed = bil_value.trim().parse::<u64>().expect("Not a number");

                    Bills.push((bill_key_trimmed.clone(), amount_parsed));


                }
            }
            
        }

        entry_counter+=1;

    }

    println!("\nList of Bills");

      let mut Print_List = String::new();

    for _bill in Bills.iter() {

        Print_List.push_str(&format!("Exp: {:?} | Amount:{}",_bill.0.trim(),_bill.1));
    }

    std::io::stdout().write_all(Print_List.as_bytes()).expect("Failed to write to console");

}


