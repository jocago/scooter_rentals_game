
mod business;
mod weather;
mod cli;
mod io;

use std::string::ToString;
use crate::business::Business;
use crate::weather::{ForecastTime, Weather};
use crate::cli::{output, get_input_string, get_input_u32, get_input_nothing,
                 get_input_f32, say_any_key};
use crate::io::SaveFile;

const USE_ANY_KEY_LABEL: bool = true;
const CHANCE_SCOOTER_BREAKS: f32 = 0.05;
const DAYS_PER_SEASON: u8 = 6;
const FORECAST_ACCURACY: f32 = 0.70;
const OPTIMAL_RENTAL_PRICE: f32 = 15.0;
const PRICE_OF_ADVERTS: f32 = 5.0;
const PRICE_OF_SCOOTERS: f32 = 100.0;
const PRICE_OF_PARTS: f32 = 25.0;
const ADVERT_EFFECT: f32 = 0.1;
const STARTING_CASH: f32 = 100.0;
const STARTING_SCOOTERS: u32 = 10;
enum GameStatus { RUNNING, QUIT }

fn main() {
    let tmp_save_file_path = "scooter_save.ron";
    // Startup
    let mut day_num: u32;
    let mut business: Business;
    let mut weather: Weather;
    output("Scooter Rentals ™️".to_string());

    match SaveFile::load_save_file(tmp_save_file_path) {
        Ok(saved) => {
            business = Business::new(saved.name)
                .with_cash(saved.cash)
                .with_working_scooters(saved.scooters_working)
                .with_broken_scooters(saved.scooters_broken)
                .with_scooter_parts(saved.scooter_parts)
                .with_num_adverts(saved.num_advertisements);

            weather = Weather::new()
                .with_curent_weather(saved.current.as_str())
                .with_forecast(saved.forecast.as_str())
                .with_season(saved.season.as_str())
                .with_temperature(saved.temperature.as_str())
                .with_days_of_season(saved.days_of_season);

            day_num = saved.day_num;
            println!("Restoring saved game:{} on day {}.", business.name(), day_num);
        }
        Err(_) => {
            output("What do you want your business to be called?".to_string());
            let business_name = get_input_string().unwrap_or_else(|_error| {
                output("That doesn't work. Let's use \"Rusty\"".to_string());
                "Rusty Scooters".to_string()
            });
            day_num = 1;
            business = Business::new(business_name);
            weather = Weather::new();
            output(format!("Opened a new Scooter business called {}!!", business.name()));
        }
    }

    // Intro
    output("\n\n\n".to_string());
    if day_num == 1 {
        output("It's your first day.".to_string());
    }

    // Main Loop
    loop {
        // Get basic info
        output(format!("Day {day_num}."));
        output(format!("{}",weather.describe(ForecastTime::Today)));
        // Do the day's rentals
        let mut cost_per = 0.0;
        loop {
            output("How much do you want to charge for each rental today?".to_string());
            match get_input_f32() {
                Ok(val) => {
                    if val < 0.0 {
                        output("Nope. That is not a positive number. Give it another shot.".to_string());
                    } else {
                        cost_per = val;
                        break;
                    }
                }
                Err(_) => {
                    output("Nope. That is not a real number. Give it another shot.".to_string());
                }
            }
        }
        // TODO: Should weather params be passed individually or just as Weather?
        match business.rent_scooters(cost_per, weather.get_temperature(), weather.get_current()) {
            Ok(receipt) => {
                output(format!("You made ${} today!", receipt.profit()));
                output(format!("{} scooters were broken today!", receipt.broken_scooters()));
                say_any_key();
                get_input_nothing();
            },
            // Should never get an error back so PANIC!!!
            Err(_) => panic!("There's an error!!!"),
        }

        // Let the player manage and get ready for tomorrow
        let game_status = main_menu(&mut business, &weather);
        match game_status {
            GameStatus::RUNNING => {
                // New Day
                day_num += 1;
                business.new_day();
                weather.new_day();
            }
            GameStatus::QUIT => break,
        }
    }

    // Save File
    let save_file = SaveFile::new(
        day_num,
        business.name(),
        business.cash(),
        business.working_scooters(),
        business.broken_scooters(),
        business.scooter_parts(),
        business.advertisements(),
        weather.get_current().describe(),
        weather.get_forecast().describe(),
        weather.get_temperature().describe(),
        weather.get_season().describe(),
        weather.get_days_of_season(),
    );
    save_file.write_save_file(tmp_save_file_path);
    // Exit
    let profit = business.cash() - STARTING_CASH;
    if profit > 0.0 {
        output(format!("You made a profit of {}", profit));
    } else if profit < 0.0 {
        output(format!("You had a loss of {}", profit));
    } else {
        output("You broke even on your business. Could be worse.".to_string());
    }

}

fn main_menu(business: &mut Business, weather: &Weather) -> GameStatus {
    loop {
        output("What would you like to do?".to_string());
        // buy
        output("1) Buy scooters or parts for repair?".to_string());
        // sell
        output("2) Sell working scooters".to_string());
        // repair
        output("3) Repair broken scooters".to_string());
        // advertise
        output("4) Buy advertisements for tomorrow".to_string());
        // get business info
        output("5) Get info on your business and the weather".to_string());
        // ready
        output("6) Ready to move on to the next day".to_string());
        // quit
        output("7) Quit the game. ".to_string());
        let input = get_input_u32();
        match input {
            Ok(n) => {
                match n {
                    1 => buy_submenu(business),
                    2 => sell_submenu(business),
                    3 => repair_submenu(business),
                    4 => advert_submenu(business),
                    5 => get_business_info(business, weather),
                    6 => return GameStatus::RUNNING,
                    7 => return GameStatus::QUIT,
                    _ => output("That's not a thing you can do.".to_string()),
                }
            },
            Err(_) => output("Use the numbers.".to_string()),
        }
        output("\n\n\n".to_string());
    }
}

fn get_business_info(business: &mut Business, weather: &Weather) {
    output(format!("{} Scooter shop has:", business.name()));
    // Cash
    output(format!("\t${} cash.", business.cash()));
    // Working Scooters
    output(format!("\t{} working scooters, ready to rent.", business.working_scooters()));
    // Broken Scooters
    output(format!("\t{} broken scooters, unrentable until repaired.", business.broken_scooters()));
    // Scooter Parts
    output(format!("\t{} parts for repairing scooters.", business.scooter_parts()));
    // Num Adverts
    output(format!("\t{} advertisements ready for tomorrow.", business.advertisements()));
    // Weather Today
    output(weather.describe(ForecastTime::Today));
    // Weather Tomorrow
    output(weather.describe(ForecastTime::Tomorrow));
    say_any_key();
    get_input_nothing();
}

fn advert_submenu(business: &mut Business) {
    output(format!("You have ${} cash.", business.cash()));
    output(format!("Each advertisement costs ${}.", PRICE_OF_ADVERTS));
    if business.cash() > PRICE_OF_ADVERTS {
        output("How many advertisements do you want to buy for tomorrow?".to_string());
        let num_res = get_input_u32();
        match num_res {
            Ok(num) => {
                let mut num = num as f32;
                let cost = PRICE_OF_ADVERTS * num;
                if cost > business.cash() {
                    num = multiples_within_f32(business.cash(), PRICE_OF_ADVERTS);
                    output(format!("You can only afford {}.", num));
                }
                business.buy_advertisements(num as u32, PRICE_OF_ADVERTS).unwrap();
                output(format!("Bought {} advertisements for ${} each.", num, PRICE_OF_ADVERTS));
            }
            Err(_) => {
                output("That's not a valid number.".to_string());
            }
        }

    } else {
        output("You don't currently have enough cash to buy an advertisement.".to_string());
    }
    say_any_key();
    get_input_nothing();
}

fn repair_submenu(business: &mut Business) {
    let reparable = u32::min(business.broken_scooters(), business.scooter_parts());
    output(format!("You have enough parts to repair {} of your broken scooters.", reparable));
    output("How many do you want to repair?".to_string());
    match get_input_u32() {
        Ok(mut num) => {
            num = u32::min(num, reparable);
            business.repair_scooters(num).expect("Scooters are repairable");
            output(format!("You repaired {} scooters.", num));
            say_any_key();
            get_input_nothing();
        },
        Err(_) => {
            output("That's not a real number.".to_string());
            say_any_key();
            get_input_nothing();
        },
    }
}

fn sell_submenu(business: &mut Business) {
    output(format!("You have {} working scooters you could sell.", business.working_scooters()));
    let price: f32 = PRICE_OF_SCOOTERS / 2.0;
    output(format!("You can get {} for each one.", price));
    output("How many would you like to sell?".to_string());
    match get_input_u32() {
        Ok(num) => {
            let mut num = num;
            if num > business.working_scooters() {
                num = business.working_scooters();
                output(format!("You can only have {} to sell.", num));
            }
            business.sell_working_scooters(num, price).unwrap();
            output(format!("Sold {} scooters for ${}.", num, price * num as f32));
            say_any_key();
            get_input_nothing();
        },
        Err(_) => {
            output("That's not a real number.".to_string());
            say_any_key();
            get_input_nothing();
        },
    }
}

fn buy_submenu(business: &mut Business) {
    output(format!("You have {} cash on hand.", business.cash()));
    output("Ok, what do you want to buy?".to_string());
    output("1) New Scooters?".to_string());
    output("2) Scooter parts?".to_string());
    output("3) Go back to the main menu.".to_string());
    let input = get_input_u32();
    match input {
        Ok(choice) => {
            match choice {
                1 => {
                    output(format!("Ok, scooters cost {}. How many?", PRICE_OF_SCOOTERS));
                    match get_input_u32() {
                        Ok(num) => {
                            let mut num = num as f32;
                            let cost = PRICE_OF_SCOOTERS * num;
                            if cost > business.cash() {
                                num = multiples_within_f32(business.cash(), PRICE_OF_SCOOTERS);
                                output(format!("You can only afford {}.", num));
                            }
                            business.buy_scooters(num as u32, PRICE_OF_SCOOTERS).unwrap();
                            output(format!("Bought {} scooters.", num));
                            say_any_key();
                            get_input_nothing();
                        },
                        Err(_) => {
                            output(format!("That's not a real number."));
                            say_any_key();
                            get_input_nothing();
                        },
                    }
                },
                2 => {
                    output(format!("Ok, parts cost {}. How many?", PRICE_OF_PARTS));
                    match get_input_u32() {
                        Ok(num) => {
                            let mut num = num as f32;
                            let cost = PRICE_OF_PARTS * num;
                            if cost > business.cash() {
                                num = multiples_within_f32(business.cash(), PRICE_OF_PARTS);
                                output(format!("You can only afford {}.", num));
                            }
                            business.buy_scooter_parts(num as u32, PRICE_OF_PARTS).unwrap();
                            output(format!("Bought {} parts.", num));
                            say_any_key();
                            get_input_nothing();
                        },
                        Err(_) => {
                            output("That's not a real number.".to_string());
                            say_any_key();
                            get_input_nothing();
                        },
                    }
                },
                3 => (),
                _ => {
                    output("That's not a thing you can do.".to_string());
                    say_any_key();
                    get_input_nothing();
                },
            }
        }
        Err(_) => {
            output("Use the numbers.".to_string());
            say_any_key();
            get_input_nothing();
        },
    }
}

fn multiples_within_f32(val_within: f32, val_to_fit: f32) -> f32 {
    (val_within - (val_within % val_to_fit)) / val_to_fit
}



