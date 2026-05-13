use std::io::{stdin,Write,stdout};
use std::collections::{HashMap, hash_map};

fn main() {
    println!("Stage 3");

    let mut Bills: HashMap<String, u64> = HashMap::new();
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
                    Bills.insert(bill_key_trimmed.clone(),value);
                    break;
                },
                Err(_) => {
                    println!("Retry (Enter a number):");
                    stdin().read_line(&mut bil_value).expect("Failed to input amount");

                    let amount_parsed = bil_value.trim().parse::<u64>().expect("Not a number");

                    Bills.insert(bill_key_trimmed.clone(), amount_parsed);


                }
            }
            
        }

        entry_counter+=1;

    }

    println!("\nList of Bills");

    let Bills_Adapted = Bills.iter().map(|(key,value)|(key.clone(),*value)).collect::<Vec<(String,u64)>>();

    let mut Print_List = String::new();

    for _bill in Bills_Adapted.iter() {

        Print_List.push_str(&format!("Exp: {:?} | Amount:{}",_bill.0.trim(),_bill.1));
    }

    std::io::stdout().write_all(Print_List.as_bytes()).expect("Failed to write to console");

    println!("Type 'D' to delete an entry or 'E' to edit an entry or 'F' to accept the list");
    let mut choice = String::new();
    stdin().read_line(&mut choice).expect("Failed to read input");

    match choice.trim() {
        "D" | "d" => {
            println!("Enter entry name: ");
            let mut entry_name = String::new();
            stdin().read_line(&mut entry_name).expect("Failed to read input");
            Bills.remove(entry_name.trim());
            
            let Bills_Adapt = Bills.iter().map(|(key,value)|(key.clone(),*value)).collect::<Vec<(String,u64)>>();
            
            for _bill in Bills_Adapt.iter() {
            Print_List.push_str(&format!("Exp: {:?} | Amount:{}",_bill.0.trim(),_bill.1));
            }

            std::io::stdout().write_all(Print_List.as_bytes()).expect("Failed to write to console");
        },

        "E" | "e" => {

            println!("Enter entry name: ");
            let mut entry_name = String::new();
            let mut new_value = String::new();
            stdin().read_line(&mut entry_name).expect("Failed to read input");
            println!("Enter new value: ");
            stdin().read_line(&mut new_value).expect("Failed to read input");
            if let Some(value) = Bills.get(entry_name.trim()){
                *value = new_value.trim().parse::<u64>().expect("conversion failed");
            };
            
            let Bills_Adapt = Bills.iter().map(|(key,value)|(key.clone(),*value)).collect::<Vec<(String,u64)>>();
            
            for _bill in Bills_Adapt.iter() {
            Print_List.push_str(&format!("Exp: {:?} | Amount:{}",_bill.0.trim(),_bill.1));
            }

            std::io::stdout().write_all(Print_List.as_bytes()).expect("Failed to write to console");

        }

        "F" | "f" => {
            println!("Accepted");
        },

        _ => {
            println!("Invalid Input");
        }
    }

}


