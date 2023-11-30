use std::fs;
use std::collections::HashMap;

mod person;
mod group;
mod picker;

use person::Person;
use group::Group;
use picker::Picker;
use serde::Deserialize;
use toml;

#[derive(Deserialize)]
struct MemberConfig {
    name: String,
    email: String,
}

#[derive(Deserialize)]
struct GroupConfig {
    title: String,
    members: Vec<MemberConfig>,
}

#[derive(Deserialize)]
struct Config {
    groups: Vec<GroupConfig>,
}

fn main() {
    let toml_content = fs::read_to_string("input.toml")
        .expect("Failed to read TOML file");

    let config: Config = toml::from_str(&toml_content)
        .expect("Failed to parse TOML file");

    let mut picker = Picker::new();

    for group_config in config.groups {
        let mut group = Group::new(&group_config.title);
        for member in group_config.members {
            let person = Person::new(&member.name, &member.email);
            group.add_person(person.id.clone());
            picker.add_person(person);
        }
        picker.add_group(group);
    }

    match picker.make_assignments() {
        Some(assignments) => {
            let max_email_length = assignments.iter()
                .map(|(giver_id, _)| picker.lookup_person(giver_id).unwrap().get_email().len())
                .max()
                .unwrap_or(0);
    
            for (giver_id, receiver_id) in assignments {
                let giver = picker.lookup_person(&giver_id).unwrap();
                let receiver = picker.lookup_person(&receiver_id).unwrap();
                println!("{:width$} -> {}", giver.get_email(), receiver.get_email(), width = max_email_length + 4); // +4 for padding
            }
        }
        None => println!("No valid assignments could be made."),
    }
    
}
