use bevy::prelude::*;
pub struct HelloPlugin;
struct GreetTimer(Timer);

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, true)))
        .add_startup_system(add_people)
        .add_system(greet_people)
        .add_system(print_position_system);
    }
}

#[derive(Component)]
struct Position { x: f32 }


#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);


fn add_people(mut commands: Commands) {
    commands.spawn().insert(Person).insert(Name("Elaina Proctor".to_string())).insert(Position{ x: 100f32 });
    commands.spawn().insert(Person).insert(Name("Renzo Hume".to_string()));
    commands.spawn().insert(Person).insert(Name("Zayna Nieves".to_string()));
}

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in query.iter() {
            println!("hello {}!", name.0);
        }
    }
}

fn print_position_system(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Position, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for position in query.iter() {
            println!("position: {:?}", position.x);
        }
    }
}


fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugin(HelloPlugin)
        .run();
}
