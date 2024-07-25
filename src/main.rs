use core::panic;
use std::io;
use std::process;

trait Check {
    fn check(&self, checklen: usize) -> Result<Vec<u32>, String>;
}

impl Check for String {
    fn check(&self, checklen: usize) -> Result<Vec<u32>, String> {
        let it = self.trim().split(",").collect::<Vec<&str>>();

        if it.len() > checklen {
            return Err(String::from("sono stati immessi più di")
                + &checklen.to_string()
                + "parametri/o");
        } else if it.len() < checklen {
            return Err(String::from("sono stati immessi meno di")
                + &checklen.to_string()
                + " parametri");
        }

        let mut tracker = 0;
        for i in &it {
            tracker += 1;

            if !i.trim().chars().all(|s| s.is_digit(10)) {
                let error = "il ".to_string()
                    + &tracker.to_string()
                    + "o numero contiene caratteri non-numerici.";
                return Err(error);
            };
        }

        return Ok(it
            .into_iter()
            .map(|s| s.parse().unwrap())
            .collect::<Vec<u32>>());
    }
}

fn main() {
    'starting: loop {
        println!("seleziona una modalità: \n\t1: MARGINE <-- prezzo e costo. \n\t2: PREZZO <-- costo e margine. \n\t3: manuale");
        let mut input_start = String::new();
        io::stdin()
            .read_line(&mut input_start)
            .expect("something went worng");
        let input_start = match input_start.check(1) {
            Ok(r) => r[0],
            Err(e) => process::exit(1),
        };

        if input_start == 1 {
            'inner_1: loop {
                println!(
                    "inserire il PREZZO e il COSTO separati da una virgola: \n\t\tPREZZO,COSTO"
                );
                let mut input_mod_1 = String::new();
                io::stdin()
                    .read_line(&mut input_mod_1)
                    .expect("somethign went wrong");
                let input_mod_1 = match input_mod_1.check(2) {
                    Ok(r) => r,
                    Err(e) => std::process::exit(1),
                };

                match input_mod_1.len() {
                    2 => {
                        let prez = input_mod_1[0] as f32;
                        let cost = input_mod_1[1] as f32;

                        let margin: f64 = (((prez - cost) / prez) * 100.0) as f64;

                        println!(
                            "il margine è: {} (costo: {}, prezzo: {})",
                            margin, cost, prez
                        );
                    }

                    n if n > 2 => {
                        panic!("too many args")
                    }

                    n if n < 2 => {
                        panic!("not enough args")
                    }

                    // ? idek why this is needed.
                    _ => panic!("shouldn't even happen"),
                }
            }
        };

        if input_start == 2 {
            'inner_2: loop {
                println!(
                    "inserire il COSTO e il MARGINE separati da una virgola: \n\t\tCOSTO,MARGINE"
                );
                let mut input_mod_1 = String::new();
                io::stdin()
                    .read_line(&mut input_mod_1)
                    .expect("somethign went wrong");
                let input_mod_1 = match input_mod_1.check(2) {
                    Ok(r) => r,
                    Err(e) => std::process::exit(1),
                };

                match input_mod_1.len() {
                    2 => {
                        let cost = input_mod_1[0] as f32;
                        let marg = input_mod_1[1] as f32;

                        let prez: f64 = ((cost / (1.0 - (marg / 100.0))) * 100.0) as f64;

                        println!("il margine è: {}", prez);
                    }

                    n if n > 2 => {
                        panic!("too many args")
                    }

                    n if n < 2 => {
                        panic!("not enough args")
                    }

                    // ? idek why this is needed.
                    _ => panic!("shouldn't even happen"),
                }
            }
        }
    }
}
