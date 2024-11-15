use dioxus::{html::input_data::keyboard_types::Key, prelude::*};

#[component]
pub fn Counter(
    count: u32,
) -> Element {
    rsx! {
        div {
            class: "pr-2 flex items-center shrink",
            div {
                class: "rounded-full bg-gray-700 text-xs min-w-[20px] h-[20px] flex items-center justify-center",
                "{count}"
            }
        }
    }
}

pub fn HelloWorld() -> () {
    // do some stuff
    let a = 1;
    let b = 2;
    dbg!(a);
    let animals = vec![
        ("Elephant", 5000),
        ("Horse", 800),
        ("Dog", 40),
        ("Cat", 4),
        ("Mouse", 5),
    ];

    let mut sorted_animals = animals.clone();
    sorted_animals.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    for (animal, weight) in sorted_animals {
        println!("{} weighs {} kg", animal, weight);
    }

    let mut total_weight = 0;
    for (_, weight) in &animals {
        total_weight += weight;
    }

    // reorder them from smallest to biggest
    let mut animals = animals.clone();
    animals.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    // do some more stuff
    for (animal, weight) in animals {
        println!("{} weighs {} kg", animal, weight);
    }

    // write a loop that changes each animal into a related animal
    
}
