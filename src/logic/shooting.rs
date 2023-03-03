use {
    bevy::prelude::*,
    std::time::Duration,
};

pub struct ShootingPlugin;

impl Plugin for ShootingPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_shooting_cooldowns);
    }
}

#[derive(Component)]
pub struct ShootingCooldown {
    pub can_shoot: bool,
    timer: Timer,
}

impl ShootingCooldown {
    pub fn new(can_shoot: bool, duration: Duration) -> Self {
        Self {
            can_shoot,
            timer: Timer::new(duration, TimerMode::Once),
        }
    }
}

fn update_shooting_cooldowns(mut cooldowns: Query<&mut ShootingCooldown>, time: Res<Time>) {
    let dt = time.delta();

    for mut cooldown in cooldowns.iter_mut() {
        cooldown.timer.tick(dt);

        if cooldown.timer.just_finished() {
            cooldown.can_shoot = true;
            cooldown.timer.reset();
            cooldown.timer.pause();
        }
    }
}