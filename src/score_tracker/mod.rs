use std::time::{SystemTimeError, SystemTime};

pub struct ScoreTracker {
    score: i32,
    start_time: SystemTime
}

impl ScoreTracker {
    pub fn new() -> Self {
        Self {
            score: 0,
            start_time: SystemTime::now()
        }
    }
    pub fn increase_score(&mut self, points: i32) {
        println!("Adding to score: {}", points);
        self.score += points;
    }
    pub fn get_time_elapsed(&self) -> Result<(u64, u64), SystemTimeError>{
        match self.start_time.elapsed() {
            Ok(duration) => {
                let seconds = duration.as_secs();
                let minutes = seconds / 60;
                Ok((minutes, seconds))
            },
            Err(err) => Err(err)
        }
    }
    pub fn get_final_score(&self) -> i32 {
        self.score
    }
}
