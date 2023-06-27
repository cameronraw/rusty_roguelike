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
        self.score += points * 10;
    }
    pub fn get_time_elapsed(&self) -> Result<(String, String), SystemTimeError>{
        match self.start_time.elapsed() {
            Ok(duration) => {
                let seconds = duration.as_secs();
                let minutes = seconds / 60;
                let seconds = seconds - (minutes * 60);
                Ok((minutes.with_leading_zeros(), seconds.with_leading_zeros()))
            },
            Err(err) => Err(err)
        }
    }
    pub fn get_current_score(&self) -> i32 {
        self.score
    }
}

pub trait WithLeadingZeros {
    fn with_leading_zeros(&self) -> String;
}

impl WithLeadingZeros for u64 {
    fn with_leading_zeros(&self) -> String {
        if *self < 9 {
            return format!("0{}", self);
        }
        self.to_string()
    }
}
