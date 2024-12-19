#[derive(Clone, Copy)]
pub enum EnemyType {
    Regular,
    Fast,
    Strong,
}

#[derive(Clone, Copy)]
pub struct EnemySpawnInfo {
    pub enemy_type: EnemyType,
    pub spawn_interval: f64,  // スポーン間隔（ミリ秒単位）
    pub last_spawn_time: f64, // 最後にスポーンした時間
}