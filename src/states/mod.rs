pub mod states;

use bevy::prelude::*;
use bevy::utils::HashMap;

use statig::state_machine::InitializedStatemachine;

pub struct StateMachinePlugin;

impl Plugin for StateMachinePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(StateMachines::default())
            .add_system_to_stage(CoreStage::PreUpdate, remove_state_machine)
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(add_state_machine)
            );
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct StateMachine;

#[derive(Default)]
pub struct StateMachines(HashMap<StateMachine, InitializedStateMachine<PlayerStateMachine>>);


fn add_state_machine(
    state_machines: Res<StateMachines>,
    query: Query<Entity, (With<StateMachine>, Added<StateMachine>)>
) {
    for entity in &query {
        state_machines.0
            .insert(entity, states::PlayerStateMachine::default()
            .state_machine()
            .init());
    }
}

fn remove_state_machine(
    state_machines: Res<StateMachines>,
    query: RemovedComponents<StateMachine>
) {
    for entity in query.iter() {
        state_machines.0
            .remove(entity);
    }
}
