use crate::entities::capacity::Capacity;

pub fn create() -> Vec<Capacity> {
    let mut capacities: Vec<Capacity> = Vec::new();

    capacities.push(Capacity {
        id: 0,
        name: String::from("Sword Attack"),
        power: 4,
        description: String::from("A devastating sword stroke"),
    });

    capacities.push(Capacity {
        id: 1,
        name: String::from("Archery Attack"),
        power: 2,
        description: String::from("A precise shot between the eyes"),
    });

    return capacities;
}
