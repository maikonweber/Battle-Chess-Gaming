use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)        
        .add_plugin(PeoplePlugin)
        .run();
}

#[derive(Component)]
pub struct Person {
    pub name: String,
}

// Usamos commando para adicionar ou remove components
pub fn setup(mut commands: Commands) {
    commands.spawn((
        Person {
            name: "Alex".to_string(),
        },
        Employed {
            job: Job::Doctor
            }
    ));
}

pub struct  PeoplePlugin;

impl Plugin for PeoplePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(print_name)
        .add_system(people_with_jobs);
    }

    fn setup(&self, _app: &mut App) {
        // do nothing
        _app.add_startup_system(setup);
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }

    fn is_unique(&self) -> bool {
        true
    }
}


pub fn print_name(person_query: Query<&Person>) {
    for person in person_query.iter() {
        println!("Name:  {}", person.name);
    }
}

pub fn people_with_jobs (
    person_query: Query<&Person, With<Employed>>
) {
    for person in person_query.iter() {
        println!("{} has job", person.name)
    }
}

pub fn people_ready_for_hire(
    person_query: Query<&Person, With<Employed>>
) {
    for person in person_query.iter() {
        println!("{} is ready to hire", person.name)
    }
}

#[derive(Component)]
pub struct Employed {
    pub job: Job,
}

pub enum Job {
    Doctor,
    Fire,
    Lawyer,
}
