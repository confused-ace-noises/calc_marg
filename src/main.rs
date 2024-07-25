use core::panic;
use std::{borrow::BorrowMut, io::{self, Split}};

trait Check {
    fn check(&self) -> Vec<u32>;
}

impl Check for String {
    fn check(&self) -> Vec<u32>{
        return self.trim()
        .split(",")
        .filter(|s| s.trim()
            .chars()
            .all(|s| s.is_digit(10)))
        .map(|s| s.parse().unwrap())
        .collect::<Vec<u32>>();
    }
}


use self::Check as OtherCheck;
fn main() {
    println!("seleziona una modalità: \n\t1: MARGINE <-- prezzo e costo. \n\t2: PREZZO <-- costo e margine. \n\t3: manuale");
    let mut input_start = String::new();
    io::stdin().read_line(&mut input_start).expect("something went worng");
    let input_start = input_start.check()[0];

    if input_start == 1 {
        println!("inserire il PREZZO e il COSTO separati da una virgola: \n\t\tPREZZO,COSTO");
        let mut input_mod_1 = String::new();
        io::stdin().read_line(&mut input_mod_1).expect("somethign went wrong");
        let input_mod_1 = input_mod_1.check();

        match input_mod_1.len(){
            2 => {
                let prez = input_mod_1[0] as f32;
                let cost = input_mod_1[1] as f32;

                let margin: f64 = (((prez - cost) / prez) * 100.0) as f64;

                println!("il margine è: {} (costo: {}, prezzo: {})", margin, cost, prez);
            },
            
            n if n > 2 => {
                panic!("too many args")
            },

            n if n < 2 => {
                panic!("not enough args")
            }

            // ? idek why this is needed. 
            _ => panic!("shouldn't even happen")
        }
    };

    if input_start == 2 {
        println!("inserire il COSTO e il MARGINE separati da una virgola: \n\t\tCOSTO,MARGINE");
        let mut input_mod_1 = String::new();
        io::stdin().read_line(&mut input_mod_1).expect("somethign went wrong");
        let input_mod_1 = input_mod_1.check();

        match input_mod_1.len(){
            2 => {
                let cost = input_mod_1[0] as f32;
                let marg = input_mod_1[1] as f32;

                let prez: f64 = ((cost /(1.0 - (marg / 100.0)))* 100.0) as f64;

                println!("il margine è: {}", prez);
            },
            
            n if n > 2 => {
                panic!("too many args")
            },

            n if n < 2 => {
                panic!("not enough args")
            }

            // ? idek why this is needed. 
            _ => panic!("shouldn't even happen")
        } 
    }
}
