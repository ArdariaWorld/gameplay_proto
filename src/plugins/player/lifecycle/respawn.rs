fn respawn_player(
    mut ev_respawn_player: EventReader<RespawnPlayerEvent>,
    mut state: ResMut<State<GameState>>,
    mut player_query: Query<(&mut Location, &mut Stats), With<Player>>,
) {
    let mut closure = || {
        for _ in ev_respawn_player.iter() {
            // Set GameState as Playing
            state.set(GameState::Playing)?;

            // Update destination so player stop moving
            let (mut location, mut stats) = player_query.get_single_mut()?;

            // Update player location and stats
            stats.hp = 100.;
            location.position = Some(RandVec2::new());
        }
        Ok::<(), ErrorMessage>(())
    };

    if let Err(error) = closure() {
        println!("{}", error);
    }
}
