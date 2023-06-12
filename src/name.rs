use bevy::prelude::*;

#[derive(Component)]
pub struct NameComp {
    pub name: String,
}

impl NameComp {
    pub fn new(input_name: String) -> Self {
        NameComp { name: input_name }
    }

    pub fn to_string(&self) -> String {
        format!("{}", self.name)
    }
}

pub fn print_name_system(query: Query<(Entity, &NameComp)>) {
    for (entity,name) in query.iter() {
        println!("Name: {}", name.to_string());
    }
}
