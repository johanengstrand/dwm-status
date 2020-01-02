use crate::error::*;
use battery;
use uom::si::f32::Ratio;
use uom::si::f32::Time;
//use uom::si::length::meter;
//use uom::si::ratio::percent;
//use uom::si::time::hour;
//use uom::si::time::minute;
use uom::si::time::second;

#[derive(Debug)]
pub(crate) enum Battery {
    Charging {
        percentage: Ratio,
        time_to_full: Time,
    },
    Discharging {
        percentage: Ratio,
        time_to_empty: Time,
    },
    Empty,
    Full,
    Unknown,
}

pub(crate) fn all_batteries() -> Result<Vec<battery::Result<Battery>>> {
    test().wrap_error("battery", "abc")
}

fn test() -> battery::Result<Vec<battery::Result<Battery>>> {
    let manager = battery::Manager::new()?;

    Ok(manager
        .batteries()?
        .map(|maybe_battery| match maybe_battery {
            Ok(battery) => match battery.state() {
                battery::State::Charging => Ok(Battery::Charging {
                    percentage: battery.state_of_charge(),
                    time_to_full: battery.time_to_full().unwrap_or_else(|| Time::new::<second>(0.)),
                }),
                battery::State::Discharging => Ok(Battery::Discharging {
                    percentage: battery.state_of_charge(),
                    time_to_empty: battery.time_to_empty().unwrap_or_else(|| Time::new::<second>(0.)),
                }),
                battery::State::Empty => Ok(Battery::Empty),
                battery::State::Full => Ok(Battery::Full),
                _ => Ok(Battery::Unknown),
            },
            Err(err) => Err(err),
        })
        .collect::<Vec<_>>())

    // for (idx, maybe_battery) in manager.batteries()?.enumerate() {
    //     let battery = maybe_battery?;
    //     println!("Battery #{}:", idx);
    //     println!(
    //         "Percentage: {}%",
    //         battery
    //             .state_of_charge()
    //             .round::<percent>()
    //             .get::<percent>()
    //     );
    //     println!("State: {:?}", battery.state());
    //     println!("Time to full charge: {:?}", battery.time_to_full());
    //     println!(
    //         "Time to empty: {:02}:{:02}",
    //         battery
    //             .time_to_empty()
    //             .unwrap()
    //             .floor::<hour>()
    //             .get::<hour>(),
    //         battery
    //             .time_to_empty()
    //             .unwrap()
    //             .fract::<hour>()
    //             .floor::<minute>()
    //             .get::<minute>()
    //     );
    //     println!();
    // }
    //
    // Ok(vec![])
}
