use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, plugins::hello::HelloPlugin))
        .run();
}

mod plugins {
    pub(crate) mod hello {
        use bevy::prelude::*;
        use derive_more::*;

        pub struct HelloPlugin;

        #[derive(Resource)]
        struct GreetTimer(Timer);

        #[derive(Component)]
        struct Person;

        #[derive(Component, Display)]
        struct Name(&'static str);

        impl Plugin for HelloPlugin {
            fn build(&self, app: &mut App) {
                app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
                    .add_systems(Startup, add_people)
                    .add_systems(Update, (update_people, greet_people).chain());
            }
        }

        fn add_people(mut commands: Commands) {
            commands.spawn((Person, Name("Elaina Proctor")));
            commands.spawn((Person, Name("Jeoffry Lanister")));
            commands.spawn((Person, Name("Stanza Stark")));
        }

        fn update_people(mut query: Query<&mut Name, With<Person>>) {
            if let Some(mut name) = query.iter_mut().find(|name| name.0 == "Stanza Stark") {
                name.0 = "Stanza Lanister".into();
            }
        }

        fn greet_people(
            time: Res<Time>,
            mut timer: ResMut<GreetTimer>,
            query: Query<&Name, With<Person>>,
        ) {
            if timer.0.tick(time.delta()).just_finished() {
                for name in &query {
                    println!("hello {}!", name);
                }
            }
        }
    }
}
