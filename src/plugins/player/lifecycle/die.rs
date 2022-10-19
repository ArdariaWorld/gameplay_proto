fn kill_player(
    mut ev_kill_player: EventReader<KillPlayerEvent>,
    mut state: ResMut<State<GameState>>,
    mut player_query: Query<&mut Location, With<Player>>,
) {
    let mut closure = || {
        for _ in ev_kill_player.iter() {
            // Set GameState as GameOver
            state.set(GameState::GameOver)?;
            let mut location = player_query.get_single_mut()?;
            location.destination = None;
        }
        Ok::<(), ErrorMessage>(())
    };

    if let Err(error) = closure() {
        println!("{}", error);
    }
}
