use std::collections::{HashMap, HashSet};
use crate::person::{Person, PersonId};
use crate::group::{Group, GroupId};
use rand::seq::SliceRandom; // Ensure rand is added in your Cargo.toml
use uuid::Uuid;

pub struct Picker {
    groups: HashMap<GroupId, Group>,
    people: HashMap<PersonId, Person>,
    graph: HashMap<PersonId, HashSet<PersonId>>,
}

impl Picker {
    pub fn new() -> Self {
        Self {
            groups: HashMap::new(),
            people: HashMap::new(),
            graph: HashMap::new(),
        }
    }

    pub fn add_group(&mut self, group: Group) {
        self.groups.insert(group.id, group);
    }

    pub fn add_person(&mut self, person: Person) {
        self.people.insert(person.id, person);
    }

    pub fn lookup_group(&self, group_id: &GroupId) -> Option<&Group> {
        self.groups.get(group_id)
    }

    pub fn lookup_person(&self, person_id: &PersonId) -> Option<&Person> {
        self.people.get(person_id)
    }

    fn build_graph(&mut self, allow_intra_group: bool) {
        self.graph.clear();
        for (person_id, _) in &self.people {
            let person_group_id = self.find_group_by_person(person_id).unwrap();
            let mut edges = HashSet::new();
            for (other_id, _) in &self.people {
                if person_id != other_id {
                    let other_group_id = self.find_group_by_person(other_id).unwrap();
                    if allow_intra_group || person_group_id != other_group_id {
                        edges.insert(*other_id);
                    }
                }
            }
            self.graph.insert(*person_id, edges);
        }
    }

    fn find_group_by_person(&self, person_id: &PersonId) -> Option<GroupId> {
        self.groups.iter()
            .find(|(_, group)| group.get_people().contains(person_id))
            .map(|(group_id, _)| *group_id)
    }

    fn attempt_hamiltonian_path_search(&self) -> Option<HashMap<PersonId, PersonId>> {
        let mut path: Vec<PersonId> = Vec::new();
        let mut visited: HashSet<PersonId> = HashSet::new();
        let keys: Vec<&PersonId> = self.graph.keys().collect();
        let start = keys.choose(&mut rand::thread_rng())?;

        if self.find_hamiltonian_path(start, &mut path, &mut visited) {
            let assignments = path.windows(2)
                .map(|window| (window[0], window[1]))
                .chain(Some(((*path.last()?).clone(), (*path.first()?).clone())))
                .collect();
            Some(assignments)
        } else {
            None
        }
    }

    fn find_hamiltonian_path(&self, current: &PersonId, path: &mut Vec<PersonId>, visited: &mut HashSet<PersonId>) -> bool {
        visited.insert(*current);
        path.push(*current);

        if path.len() == self.graph.len() {
            return self.graph[current].contains(path.first().unwrap());
        }

        for &next in &self.graph[current] {
            if !visited.contains(&next) {
                if self.find_hamiltonian_path(&next, path, visited) {
                    return true;
                }
            }
        }

        visited.remove(current);
        path.pop();
        false
    }

    fn attempt_subloop_assignments(&self) -> Option<HashMap<PersonId, PersonId>> {
        // Implementation for multiple subloops
        let mut assignments = HashMap::new();
        let mut to_assign: HashSet<PersonId> = self.people.keys().cloned().collect();

        while !to_assign.is_empty() {
            let mut subloop = Vec::new();
            let mut current_id = *to_assign.iter().next().unwrap();

            while let Some(&next_id) = self.graph[&current_id].iter().find(|id| to_assign.contains(id)) {
                to_assign.remove(&current_id);
                assignments.insert(current_id, next_id);
                current_id = next_id;
                subloop.push(current_id);
            }

            // Close the subloop if possible
            if let Some(&first_id) = subloop.first() {
                if self.graph[&current_id].contains(&first_id) {
                    assignments.insert(current_id, first_id);
                    to_assign.remove(&current_id);
                }
            }
        }

        if !assignments.is_empty() {
            Some(assignments)
        } else {
            None
        }
    }

    pub fn make_assignments(&mut self) -> Option<HashMap<PersonId, PersonId>> {
        // First attempt without intra-group assignments
        self.build_graph(false);
        if let Some(assignments) = self.attempt_hamiltonian_path_search() {
            return Some(assignments);
        }

        // Fallback to allowing intra-group assignments
        println!("Fallback: Allowing intra-group assignments.");
        self.build_graph(true);
        if let Some(assignments) = self.attempt_hamiltonian_path_search() {
            return Some(assignments);
        }

        // Final fallback to multiple subloops
        println!("Fallback: Allowing multiple subloops.");
        self.attempt_subloop_assignments()
    }
}
