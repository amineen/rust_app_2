use rand::Rng;

struct PeriodicCapacity {
    hour: i8,
    power_capacity: f32,
    power_factor: f32,
}
const PERIODIC_CAPACITY: [PeriodicCapacity; 24] = [
    PeriodicCapacity {
        hour: 0,
        power_capacity: 2.0,
        power_factor: 0.85,
    },
    PeriodicCapacity {
        hour: 1,
        power_capacity: 1.5,
        power_factor: 0.85,
    },
    PeriodicCapacity {
        hour: 2,
        power_capacity: 1.2,
        power_factor: 0.86,
    },
    PeriodicCapacity {
        hour: 2,
        power_capacity: 1.0,
        power_factor: 0.83,
    },
    PeriodicCapacity {
        hour: 4,
        power_capacity: 1.5,
        power_factor: 0.80,
    },
    PeriodicCapacity {
        hour: 5,
        power_capacity: 1.7,
        power_factor: 0.78,
    },
    PeriodicCapacity {
        hour: 6,
        power_capacity: 1.8,
        power_factor: 0.75,
    },
    PeriodicCapacity {
        hour: 7,
        power_capacity: 1.9,
        power_factor: 0.75,
    },
    PeriodicCapacity {
        hour: 8,
        power_capacity: 1.0,
        power_factor: 0.75,
    },
    PeriodicCapacity {
        hour: 9,
        power_capacity: 1.0,
        power_factor: 0.75,
    },
    PeriodicCapacity {
        hour: 10,
        power_capacity: 1.1,
        power_factor: 0.75,
    },
    PeriodicCapacity {
        hour: 11,
        power_capacity: 1.2,
        power_factor: 0.75,
    },
    PeriodicCapacity {
        hour: 12,
        power_capacity: 1.4,
        power_factor: 0.75,
    },
    PeriodicCapacity {
        hour: 13,
        power_capacity: 1.5,
        power_factor: 0.75,
    },
    PeriodicCapacity {
        hour: 14,
        power_capacity: 2.6,
        power_factor: 0.75,
    },
    PeriodicCapacity {
        hour: 15,
        power_capacity: 2.6,
        power_factor: 0.75,
    },
    PeriodicCapacity {
        hour: 16,
        power_capacity: 2.6,
        power_factor: 0.75,
    },
    PeriodicCapacity {
        hour: 17,
        power_capacity: 2.6,
        power_factor: 0.75,
    },
    PeriodicCapacity {
        hour: 18,
        power_capacity: 2.7,
        power_factor: 0.75,
    },
    PeriodicCapacity {
        hour: 19,
        power_capacity: 3.2,
        power_factor: 0.75,
    },
    PeriodicCapacity {
        hour: 20,
        power_capacity: 3.7,
        power_factor: 0.75,
    },
    PeriodicCapacity {
        hour: 21,
        power_capacity: 3.9,
        power_factor: 0.75,
    },
    PeriodicCapacity {
        hour: 22,
        power_capacity: 3.0,
        power_factor: 0.75,
    },
    PeriodicCapacity {
        hour: 23,
        power_capacity: 2.7,
        power_factor: 0.75,
    },
];

pub struct PowerConsumption {
    pub datetime: String,
    pub active_power: f32,
    pub energy_used: f32,
    pub power_factor: f32,
}

pub fn create_power_consumptions(date: &str, start_power: f32) -> Vec<PowerConsumption> {
    let mut power_consumption_data = Vec::new();
    let num_of_records: i8 = 96;

    for i in 0..num_of_records {
        let hour: i32 = (i as i32) * 15 / 60;
        let minute = (i as i32) * 15 % 60;
        let datetime = format!("{}T{:02}:{:02}:00", date, hour, minute);
        let power_capacity = &PERIODIC_CAPACITY[hour as usize];

        //get a random number between 0.6 and 1.0
        let capacity_factor: f32 = rand::thread_rng().gen_range(0.6..1.0);

        let active_power = start_power * power_capacity.power_capacity;
        let power_factor = power_capacity.power_factor * capacity_factor;
        let energy_used = active_power * 0.25;
        let power_consumption = PowerConsumption {
            datetime,
            active_power,
            energy_used,
            power_factor,
        };
        power_consumption_data.push(power_consumption);
    }
    power_consumption_data
}
