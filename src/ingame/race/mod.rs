use bevy::{prelude::*, ecs::system::{Command, SystemState}, };
use crate::ingame::{points, race::placement_sensor::Place};
use bevy_xpbd_3d::{math::*, prelude::*};

pub mod placement_sensor;

pub struct RacePlugin;
impl Plugin for RacePlugin {
    fn build(&self, app: &mut App) {
        //app.add_systems(Update, detect_finish_line.run_if(in_state(AppState::InGame)));
        app.add_plugins(placement_sensor::PlacementSensorPlugin);
    }
}


#[derive(Component)]
pub struct WayPoint(pub WayPoints);

#[derive(Debug, PartialEq)]
pub enum WayPoints {
    Start, Finish, Quarter, Half
}

#[derive(Component)]
pub struct NextWayPoint(pub WayPoints);

#[derive(Component)]
pub struct LapCounter(pub usize);

pub struct WayPointSpawner {
    pub entity: Entity,
    pub name: String,
    pub mesh: Mesh,
}
impl Command for WayPointSpawner {
    fn apply(self, world: &mut World) {
        let waypoints = if self.name.contains("start") {
            Some(WayPoints::Start)
        } else if self.name.contains("finish") {
            Some(WayPoints::Finish)
        } else if self.name.contains("quarter") {
            Some(WayPoints::Quarter)
        } else if self.name.contains("half") {
            Some(WayPoints::Half)
        } else {
            None
        };

        if let Some(waypoints) = waypoints {
            world.entity_mut(self.entity)
                .insert((
                    WayPoint(waypoints),
                    Collider::trimesh_from_mesh(&self.mesh).unwrap(), 
                    Visibility::Hidden,
                ));
        }
    }
}

pub struct WayPointHitHandler {
    pub entity: Entity,
}

impl Command for WayPointHitHandler {
    fn apply(self, world: &mut World) {
        let mut system_state: SystemState<(
            Query<(&mut NextWayPoint, &mut LapCounter, &mut points::Points, &Place)>,
            
        )> = SystemState::new(world);

        let (mut next_waypoints, ) = system_state.get_mut(world);

        if let Ok((mut next_waypoint, mut lap_counter, mut points, place)) = next_waypoints.get_mut(self.entity) {
            next_waypoint.0 = match next_waypoint.0 {
                WayPoints::Start => WayPoints::Quarter,
                WayPoints::Quarter => WayPoints::Half,
                WayPoints::Half => WayPoints::Finish,
                WayPoints::Finish => {
                    lap_counter.0 += 1;
                    points.0 += 9 - place.0;

                    WayPoints::Start
                },
            };
        }
    }
}
