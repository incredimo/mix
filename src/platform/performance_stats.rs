use std::time::{Duration, Instant};

#[derive(Clone, Debug)]
pub struct PerformanceStats {
    pub frame_count: usize,
    pub frame_time: Duration,
    pub frame_times: Vec<Duration>,
    pub last_frame_time: Instant,
    pub fps: f32,
}

impl PerformanceStats {
    pub fn new() -> Self {
        Self {
            frame_count: 0,
            frame_time: Duration::from_secs(0),
            frame_times: Vec::with_capacity(60),
            last_frame_time: Instant::now(),
            fps: 0.0,
        }
    }
    
    pub fn update(&mut self) {
        let now = Instant::now();
        let frame_time = now - self.last_frame_time;
        self.last_frame_time = now;
        
        self.frame_time = frame_time;
        self.frame_count += 1;
        
        self.frame_times.push(frame_time);
        if self.frame_times.len() > 60 {
            self.frame_times.remove(0);
        }
        
        let total_time: Duration = self.frame_times.iter().sum();
        let avg_time = total_time.as_secs_f32() / self.frame_times.len() as f32;
        self.fps = if avg_time > 0.0 { 1.0 / avg_time } else { 0.0 };
    }
}



