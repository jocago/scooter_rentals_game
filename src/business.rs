use std::cmp::{max, min};
use rand::{random, Rng};
use crate::{ADVERT_EFFECT, CHANCE_SCOOTER_BREAKS, OPTIMAL_RENTAL_PRICE, STARTING_CASH, STARTING_SCOOTERS};
use crate::weather::{Temperature, WeatherType};

type DidItWork = Result<(), ManagementError>;

#[derive(PartialEq, Debug)]
pub(crate) enum ManagementError {
    InsufficientWorkingScooters,
    InsufficientBrokenScooters,
    InsufficientParts,
    NotEnoughMoney,
    InvalidParameter,
}

#[derive(Debug, Clone)]
pub(crate) struct Business {
    name: String,
    cash: f32,
    scooters_working: u32,
    scooters_broken: u32,
    scooter_parts: u32,
    num_advertisements: u32,
}

impl Business {
    pub(crate) fn name(&self) -> String { self.name.clone() }
    pub(crate) fn cash(&self) -> f32 { self.cash }
    pub(crate) fn working_scooters(&self) -> u32 { self.scooters_working }
    pub(crate) fn broken_scooters(&self) -> u32 { self.scooters_broken }
    pub(crate) fn scooter_parts(&self) -> u32 { self.scooter_parts }
    pub(crate) fn advertisements(&self) -> u32 { self.num_advertisements }

    pub(crate) fn new(name: String) -> Self {
        Self {
            name,
            cash: STARTING_CASH,
            scooters_working: STARTING_SCOOTERS,
            scooters_broken: 0,
            scooter_parts: 0,
            num_advertisements: 0,
        }
    }

    pub(crate) fn with_cash(mut self, cash: f32) -> Self { self.cash = cash; self}
    pub(crate) fn with_working_scooters(mut self, num: u32) -> Self { self.scooters_working = num; self }
    pub(crate) fn with_broken_scooters(mut self, num: u32) -> Self { self.scooters_broken = num; self }
    pub(crate) fn with_scooter_parts(mut self, num: u32) -> Self { self.scooter_parts = num; self }
    pub(crate) fn with_num_adverts(mut self, num: u32) -> Self { self.num_advertisements = num; self }

    pub(crate) fn buy_scooters(&mut self, num: u32, cost_per: f32) -> DidItWork {
        if cost_per < 0_f32 { return  Err(ManagementError::InvalidParameter) }
        let cost = num as f32 * cost_per;
        if cost > self.cash { return Err(ManagementError::NotEnoughMoney) }
        self.scooters_working += num;
        self.cash -= cost;
        Ok(())
    }

    pub(crate) fn sell_working_scooters(&mut self, num: u32, cost_per: f32) -> DidItWork {
        if cost_per < 0_f32  { return  Err(ManagementError::InvalidParameter) }
        if num > self.scooters_working { return Err(ManagementError::InsufficientWorkingScooters) }
        let cost = num as f32 * cost_per;
        self.scooters_working -= num;
        self.cash += cost;
        Ok(())
    }

    // TODO: Create tests
    pub(crate) fn repair_scooters(&mut self, num: u32) -> DidItWork {
        if num > self.scooter_parts { return Err(ManagementError::InsufficientParts) }
        if num > self.scooters_broken { return Err(ManagementError::InsufficientBrokenScooters) }
        self.scooters_broken -= num;
        self.scooters_working += num;
        self.scooter_parts -= num;
        Ok(())
    }

    // TODO: Create tests
    pub(crate) fn buy_scooter_parts(&mut self, num: u32, cost_per: f32) -> DidItWork {
        if cost_per < 0_f32 { return Err(ManagementError::InvalidParameter) }
        let cost = num as f32 * cost_per;
        if cost > self.cash { return Err(ManagementError::NotEnoughMoney) }
        self.cash -= cost;
        self.scooter_parts += num;
        Ok(())
    }

    pub(crate) fn new_day(&mut self) {
        self.num_advertisements = 0;
    }

    ///
    ///
    /// # Arguments
    ///
    /// * num: Number of scooters to rent. This must not exceed available working scooters.
    /// * cost_per: Amount to gain per scooter rental.
    /// * temperature: The current temperature
    /// * weather: the current weather type
    ///
    /// returns: Result<Receipt, ManagementError>
    ///
    /// If result is Ok, returns a Receipt object from this rental period.
    /// Temperature and weather are used to determine what percentage of scooters are
    /// actually rented for the day.
    ///
    /// # Examples
    ///
    /// ```
    /// rent_scooters(10, 20.0, Temperature::Cold, WeatherType::Stormy) -> Ok(Receipt{profit: 10.0, broken_scooters: 1})
    ///
    /// rent_scooters(10, -10.0, Temperature::Cold, WeatherType::Stormy) -> Err(ManagementError::InvalidParameter)
    /// ```
    // TODO: Create tests
    pub(crate) fn rent_scooters(
        &mut self,
        cost_per: f32,
        temperature: Temperature,
        weather: WeatherType,
    ) -> Result<Receipt, ManagementError> {
        let num = self.scooters_working;
        if cost_per < 0_f32 { return Err(ManagementError::InvalidParameter) }
        if num > self.scooters_working { return Err(ManagementError::InsufficientWorkingScooters) }
        // Determine actual number of rented scooters
        let combined_mod = &self.combined_modifier(temperature, weather, cost_per);
        let price_mod = OPTIMAL_RENTAL_PRICE / cost_per;
        let rented: u32 = max(0, min((num as f32 * combined_mod * price_mod).floor() as u32, num));
        // Do the transaction
        let profit = rented as f32 * cost_per;
        self.cash += profit;
        // Breaking scooters
        let mut broken_scooters = 0_u32;
        for _ in 0..rented {
            if random::<f32>() < CHANCE_SCOOTER_BREAKS {
                broken_scooters += 1;
            }
        }
        self.scooters_working -= broken_scooters;
        self.scooters_broken += broken_scooters;
        Ok(Receipt::new(profit, broken_scooters))
    }

    // TODO: Create tests
    // TODO: Add this to rent_scooters
    fn combined_modifier(
        &self,
        temperature: Temperature,
        current_weather: WeatherType,
        cost_per: f32,
    ) -> f32 {
        let advert_effect = ADVERT_EFFECT * min(self.num_advertisements, self.scooters_working) as f32;
        let cost_effect = f32::max(-1.0, f32::min(1.0,(OPTIMAL_RENTAL_PRICE - cost_per) / 10.0));
        let mut val = 1.0 + advert_effect + cost_effect;
        let rnd = rand::thread_rng().gen_range(-0.1 .. 0.1);
        match (temperature, current_weather) {
            (Temperature::Scorching, WeatherType::Cloudy) => val *= 0.25,
            (Temperature::Scorching, WeatherType::Sunny) => val *= 0.05,
            (Temperature::Scorching, WeatherType::Rainy) => val *= 0.10,
            (Temperature::Scorching, WeatherType::Stormy) => val *= 0.00,
            (Temperature::Hot, WeatherType::Cloudy) => val *= 0.60,
            (Temperature::Hot, WeatherType::Sunny) => val *= 0.50,
            (Temperature::Hot, WeatherType::Rainy) => val *= 0.10,
            (Temperature::Hot, WeatherType::Stormy) => val *= 0.05,
            (Temperature::Warm, WeatherType::Cloudy) => val *= 0.90,
            (Temperature::Warm, WeatherType::Sunny) => val *= 1.00,
            (Temperature::Warm, WeatherType::Rainy) => val *= 0.20,
            (Temperature::Warm, WeatherType::Stormy) => val *= 0.05,
            (Temperature::Cool, WeatherType::Cloudy) => val *= 0.90,
            (Temperature::Cool, WeatherType::Sunny) => val *= 1.00,
            (Temperature::Cool, WeatherType::Rainy) => val *= 0.20,
            (Temperature::Cool, WeatherType::Stormy) => val *= 0.05,
            (Temperature::Cold, WeatherType::Cloudy) => val *= 0.50,
            (Temperature::Cold, WeatherType::Sunny) => val *= 0.60,
            (Temperature::Cold, WeatherType::Rainy) => val *= 0.10,
            (Temperature::Cold, WeatherType::Stormy) => val *= 0.05,
            (Temperature::Freezing, WeatherType::Cloudy) => val *= 0.10,
            (Temperature::Freezing, WeatherType::Sunny) => val *= 0.25,
            (Temperature::Freezing, WeatherType::Rainy) => val *= 0.05,
            (Temperature::Freezing, WeatherType::Stormy) => val *= 0.00,
            (t,w) => {
                panic!("The temp ({:?}) and weather ({:?}) was not accounted for.",t,w)
            },
        }
        f32::max(0.0_f32, f32::min(1.0_f32, val + rnd))
    }

    pub(crate) fn buy_advertisements(&mut self, num: u32, cost: f32) -> DidItWork {
        if num as f32 * cost > self.cash { return Err(ManagementError::NotEnoughMoney) }
        self.num_advertisements = num;
        self.cash -= num as f32 * cost;
        Ok(())
    }
}

// TODO: Create tests
// Do I really need this? Consider replacing with DidItWork
#[derive(Debug)]
pub struct Receipt {
    profit: f32,
    broken_scooters: u32,
}

impl Receipt {
    pub(crate) fn new(profit: f32, broken_scooters: u32) -> Self {
        Self { profit, broken_scooters }
    }

    pub fn profit(&self) -> f32 { self.profit }
    pub fn broken_scooters(&self) -> u32 { self.broken_scooters }
}

#[cfg(test)]
mod buy_scooters_tests {
    use crate::business::{Business, ManagementError};

    #[test]
    fn buy_some_scooters() {
        let mut business = Business::new("New Scoots, Inc.".to_string());
        if business.buy_scooters(2, 40_f32).is_ok() {
            assert_eq!(business.cash(), 20_f32)
        } else { panic!("Buy was not ok") }
    }
    fn cashflow_problems() {
        let mut business = Business::new("New Scoots, Inc.".to_string());
        assert_eq!(business.buy_scooters(10, 40_f32), Err(ManagementError::NotEnoughMoney))
    }
    fn negative_cost() {
        let mut business = Business::new("New Scoots, Inc.".to_string());
        assert_eq!(business.buy_scooters(10, -40_f32), Err(ManagementError::InvalidParameter))
    }
}


#[cfg(test)]
mod sell_scooters_tests {
    use crate::business::{Business, ManagementError};

    #[test]
    fn sell_working_scooters() {
        let mut business = Business::new("New Scoots, Inc.".to_string());
        if business.sell_working_scooters(1, 10_f32).is_ok() {
            assert_eq!(business.cash(), 110_f32)
        } else { panic!("Sell was not ok") }

    }
    #[test]
    fn not_enough_to_sell() {
        let mut business = Business::new("New Scoots, Inc.".to_string());
        assert_eq!(business.sell_working_scooters(20, 40_f32), Err(ManagementError::InsufficientWorkingScooters))
    }
    #[test]
    fn negative_cost() {
        let mut business = Business::new("New Scoots, Inc.".to_string());
        assert_eq!(business.sell_working_scooters(10, -40_f32), Err(ManagementError::InvalidParameter))
    }
}

#[cfg(test)]
mod rent_scooters_test {
    use crate::business::Business;
    use crate::weather::Weather;

    #[test]
    fn normal_rental() {
        let mut business = Business::new("New Scoots, Inc.".to_string());
        let mut weather = Weather::new();
        weather.new_day();
        let temp = weather.get_temperature();
        let wt = weather.get_current();
        dbg!(business.rent_scooters(15.0_f32, temp, wt).unwrap());
    }
}