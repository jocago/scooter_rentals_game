use rand::{random};
use rand::seq::SliceRandom;
use crate::{DAYS_PER_SEASON, FORECAST_ACCURACY};

#[derive(PartialEq, Debug, Clone, Copy)]
pub(crate) enum WeatherType {
    Sunny,
    Cloudy,
    Rainy,
    Stormy,
    Snowy,
}

impl WeatherType {
    pub(crate) fn describe(&self) -> String {
        match self {
            WeatherType::Sunny => "sunny".to_string(),
            WeatherType::Cloudy => "cloudy".to_string(),
            WeatherType::Rainy => "rainy".to_string(),
            WeatherType::Stormy => "stormy".to_string(),
            WeatherType::Snowy => "snowy".to_string(),
        }
    }

    pub(crate) fn from_str(desc: &str) -> WeatherType {
        match desc {
            "sunny" => WeatherType::Sunny,
            "cloudy" => WeatherType::Cloudy,
            "rainy" => WeatherType::Rainy,
            "stormy" => WeatherType::Stormy,
            "snowy" => WeatherType::Snowy,
            _ => WeatherType::Sunny,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Copy, Clone, Ord, PartialOrd)]
pub(crate) enum Temperature {
    Scorching,
    Hot,
    Warm,
    Cool,
    Cold,
    Freezing,
}

impl Temperature {
    pub(crate) fn describe(&self) -> String {
        match self {
            Temperature::Scorching => "scorching".to_string(),
            Temperature::Hot => "hot".to_string(),
            Temperature::Warm => "warm".to_string(),
            Temperature::Cool => "cool".to_string(),
            Temperature::Cold => "cold".to_string(),
            Temperature::Freezing => "freezing".to_string(),
        }
    }

    pub(crate) fn from_str(temperature: &str) -> Temperature {
        match temperature {
            "scorching" => Temperature::Scorching,
            "hot" => Temperature::Hot,
            "warm" => Temperature::Warm,
            "cool" => Temperature::Cool,
            "cold" => Temperature::Cold,
            "freezing" => Temperature::Freezing,
            _ => Temperature::Warm,
        }
    }
}

pub(crate) enum ForecastTime { Today, Tomorrow }

impl ForecastTime {
    pub(crate) fn describe(&self) -> String {
        match self {
            ForecastTime::Today => "today".to_string(),
            ForecastTime::Tomorrow => "tomorrow".to_string(),
        }
    }
}

#[derive(PartialEq, Eq, Debug, Copy, Clone, Ord, PartialOrd)]
pub(crate) enum Season {
    Spring,
    Summer,
    Fall,
    Winter,
}

impl Season {
    pub(crate) fn describe(&self) -> String {
        match self {
            Season::Spring => "spring".to_string(),
            Season::Summer => "summer".to_string(),
            Season::Fall => "fall".to_string(),
            Season::Winter => "winter".to_string(),
        }
    }

    pub(crate) fn from_str(season: &str) -> Season {
        match season {
            "spring" => Season::Spring,
            "summer" => Season::Summer,
            "fall" => Season::Fall,
            "winter" => Season::Winter,
            _ => Season::Spring,
        }
    }
}

impl Season {
    fn temperature_choice(&self) -> Temperature {
        let mut vals: Vec<Temperature> = Vec::new();
        match self {
            Season::Spring => {
                vals.push(Temperature::Cool);
                vals.push(Temperature::Warm);
                vals.push(Temperature::Hot);
            },
            Season::Summer => {
                vals.push(Temperature::Warm);
                vals.push(Temperature::Hot);
                vals.push(Temperature::Scorching);
            },
            Season::Fall => {
                vals.push(Temperature::Warm);
                vals.push(Temperature::Cool);
                vals.push(Temperature::Cold);
            },
            Season::Winter => {
                vals.push(Temperature::Cool);
                vals.push(Temperature::Cold);
                vals.push(Temperature::Freezing);
            },
        }
        vals.choose(&mut rand::thread_rng()).copied().unwrap()
    }

    fn weather_choice(&self) -> WeatherType {
        let rnd = random::<f32>();
        if rnd < 0.3 { WeatherType::Sunny }
        else if rnd < 0.6 { WeatherType::Cloudy }
        else if rnd < 0.8 {
            if *self == Season::Winter { WeatherType::Snowy }
            else { WeatherType::Rainy }
        }
        else { WeatherType::Stormy }
    }
}

#[derive(Clone, Debug, Copy)]
pub(crate) struct Weather {
    current: WeatherType,
    forecast: WeatherType,
    temperature: Temperature,
    season: Season,
    days_of_season: u8,
}

impl Weather {
    pub(crate) fn new() -> Weather {
        Weather {
            current: WeatherType::Sunny,
            forecast: WeatherType::Sunny,
            temperature: Temperature::Warm,
            season: Season::Spring,
            days_of_season: 0,
        }
    }

    pub(crate) fn with_curent_weather(mut self, current: &str) -> Self
    { self.current = WeatherType::from_str(current); self }
    pub(crate) fn with_forecast(mut self, forecast: &str) -> Self
    { self.forecast = WeatherType::from_str(forecast); self }
    pub(crate) fn with_temperature(mut self, temperature: &str) -> Self
    { self.temperature = Temperature::from_str(temperature); self }
    pub(crate) fn with_season(mut self, season: &str) -> Self
    { self.season = Season::from_str(season); self }
    pub(crate) fn with_days_of_season(mut self, days: u8) -> Self
    { self.days_of_season = days; self }

    pub(crate) fn get_temperature(self) -> Temperature { self.temperature }
    pub(crate) fn get_current(self) -> WeatherType { self.current }
    pub(crate) fn get_forecast(self) -> WeatherType { self.forecast }
    pub(crate) fn get_season(self) -> Season { self.season }
    pub(crate) fn get_days_of_season(self) -> u8 { self.days_of_season }

    // TODO: Create tests
    pub(crate) fn new_day(&mut self) {
        // Current
        if random::<f32>() > FORECAST_ACCURACY {
            // Forecast sucked, get new one
            self.new_forecast();
        }
        self.current = self.forecast.clone();
        self.temperature = self.season.temperature_choice();
        // Forecast
        self.new_forecast();
        // Days Tracking
        self.days_of_season += 1;
        if self.days_of_season > DAYS_PER_SEASON {
            self.days_of_season = 1;
            self.season = match self.season {
                Season::Spring => Season::Summer,
                Season::Summer => Season::Fall,
                Season::Fall => Season::Winter,
                Season::Winter => Season::Spring,
            }
        }
    }

    // TODO: Create tests
    pub(crate) fn describe(&self, forecast_time: ForecastTime) -> String {
        format!("It {} a {} {} {} day, {}.",
                match forecast_time {
                    ForecastTime::Today => "is",
                    ForecastTime::Tomorrow => "might be",
                },
                self.temperature.describe(),
                match forecast_time {
                    ForecastTime::Today => self.current.describe(),
                    ForecastTime::Tomorrow => self.forecast.describe(),
                },
                self.season.describe(),
                forecast_time.describe(),
        )
    }

    // TODO: Create tests
    fn new_forecast(&mut self)  {
        self.forecast = self.season.weather_choice()
    }

}

#[cfg(test)]
mod season_tests {

    #[test]
    fn choose_weather() {
        ()
    }

}