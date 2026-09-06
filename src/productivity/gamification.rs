#![allow(clippy::new_without_default)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(unexpected_cfgs)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::type_complexity)]
use std::string::{String, ToString};
use std::vec::Vec;
// SigmaOS Gamified Productivity System
// Built-in goal trackers, Pomodoro timers, and progress dashboards

use crate::klib::HashMap;
use core::time::Duration;
// Instant not in no_std

/// Achievement type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AchievementType {
    FocusTime,
    TasksCompleted,
    GoalsReached,
    StreakDays,
    ProductivityScore,
}

/// Achievement
#[derive(Debug, Clone)]
pub struct Achievement {
    pub id: String,
    pub name: String,
    pub description: String,
    pub achievement_type: AchievementType,
    pub target_value: u64,
    pub current_value: u64,
    pub unlocked: bool,
    pub icon: String,
}

impl Achievement {
    pub fn new(
        id: String,
        name: String,
        achievement_type: AchievementType,
        target_value: u64,
    ) -> Self {
        Self {
            id,
            name,
            description: String::new(),
            achievement_type,
            target_value,
            current_value: 0,
            unlocked: false,
            icon: "🏆".to_string(),
        }
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = description;
        self
    }

    pub fn with_icon(mut self, icon: String) -> Self {
        self.icon = icon;
        self
    }

    pub fn update_progress(&mut self, value: u64) {
        self.current_value = value;
        if self.current_value >= self.target_value {
            self.unlocked = true;
        }
    }

    pub fn get_progress_percentage(&self) -> f64 {
        if self.target_value == 0 {
            return 100.0;
        }
        (self.current_value as f64 / self.target_value as f64 * 100.0).min(100.0)
    }
}

/// Pomodoro timer state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PomodoroState {
    Work,
    ShortBreak,
    LongBreak,
    Paused,
    Stopped,
}

/// Pomodoro timer
pub struct PomodoroTimer {
    pub state: PomodoroState,
    pub work_duration: Duration,
    pub short_break_duration: Duration,
    pub long_break_duration: Duration,
    pub current_duration: Duration,
    pub elapsed: Duration,
    pub pomodoros_completed: u32,
    pub running: bool,
    pub start_time: Option<u64>,
}

impl PomodoroTimer {
    pub fn new() -> Self {
        Self {
            state: PomodoroState::Stopped,
            work_duration: Duration::from_secs(25 * 60),
            short_break_duration: Duration::from_secs(5 * 60),
            long_break_duration: Duration::from_secs(15 * 60),
            current_duration: Duration::from_secs(25 * 60),
            elapsed: Duration::from_secs(0),
            pomodoros_completed: 0,
            running: false,
            start_time: None,
        }
    }

    pub fn start_work(&mut self) {
        self.state = PomodoroState::Work;
        self.current_duration = self.work_duration;
        self.elapsed = Duration::from_secs(0);
        self.running = true;
        self.start_time = Some(0u64);
    }

    pub fn start_short_break(&mut self) {
        self.state = PomodoroState::ShortBreak;
        self.current_duration = self.short_break_duration;
        self.elapsed = Duration::from_secs(0);
        self.running = true;
        self.start_time = Some(0u64);
    }

    pub fn start_long_break(&mut self) {
        self.state = PomodoroState::LongBreak;
        self.current_duration = self.long_break_duration;
        self.elapsed = Duration::from_secs(0);
        self.running = true;
        self.start_time = Some(0u64);
    }

    pub fn pause(&mut self) {
        self.state = PomodoroState::Paused;
        self.running = false;
    }

    pub fn resume(&mut self) {
        if self.state == PomodoroState::Paused {
            self.running = true;
            self.start_time = Some(0u64);
        }
    }

    pub fn stop(&mut self) {
        self.state = PomodoroState::Stopped;
        self.running = false;
        self.elapsed = Duration::from_secs(0);
        self.start_time = None;
    }

    pub fn update(&mut self) {
        if !self.running {
            return;
        }

        if let Some(_start) = self.start_time {
            self.elapsed = core::time::Duration::from_millis(0);

            if self.elapsed >= self.current_duration {
                self.complete_pomodoro();
            }
        }
    }

    fn complete_pomodoro(&mut self) {
        self.running = false;
        self.start_time = None;

        match self.state {
            PomodoroState::Work => {
                self.pomodoros_completed += 1;
                // Auto-start break
                if self.pomodoros_completed.is_multiple_of(4) {
                    self.start_long_break();
                } else {
                    self.start_short_break();
                }
            }
            PomodoroState::ShortBreak => {
                self.start_work();
            }
            PomodoroState::LongBreak => {
                self.start_work();
            }
            _ => {}
        }
    }

    pub fn get_remaining_time(&self) -> Duration {
        if self.elapsed >= self.current_duration {
            return Duration::from_secs(0);
        }
        self.current_duration - self.elapsed
    }

    pub fn get_progress_percentage(&self) -> f64 {
        if self.current_duration.as_secs() == 0 {
            return 100.0;
        }
        (self.elapsed.as_secs_f64() / self.current_duration.as_secs_f64() * 100.0).min(100.0)
    }
}

impl Default for PomodoroTimer {
    fn default() -> Self {
        Self::new()
    }
}

/// Goal
#[derive(Debug, Clone)]
pub struct Goal {
    pub id: String,
    pub title: String,
    pub description: String,
    pub target_value: u64,
    pub current_value: u64,
    pub deadline: Option<u64>,
    pub completed: bool,
    pub category: String,
}

impl Goal {
    pub fn new(id: String, title: String, target_value: u64) -> Self {
        Self {
            id,
            title,
            description: String::new(),
            target_value,
            current_value: 0,
            deadline: None,
            completed: false,
            category: "General".to_string(),
        }
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = description;
        self
    }

    pub fn with_deadline(mut self, deadline: u64) -> Self {
        self.deadline = Some(deadline);
        self
    }

    pub fn with_category(mut self, category: String) -> Self {
        self.category = category;
        self
    }

    pub fn update_progress(&mut self, value: u64) {
        self.current_value = value;
        if self.current_value >= self.target_value {
            self.completed = true;
        }
    }

    pub fn get_progress_percentage(&self) -> f64 {
        if self.target_value == 0 {
            return 100.0;
        }
        (self.current_value as f64 / self.target_value as f64 * 100.0).min(100.0)
    }
}

/// Productivity score
#[derive(Debug, Clone)]
pub struct ProductivityScore {
    pub focus_score: f64,
    pub task_completion_score: f64,
    pub consistency_score: f64,
    pub overall_score: f64,
    pub streak_days: u32,
    pub last_activity_date: Option<u64>,
}

impl ProductivityScore {
    pub fn new() -> Self {
        Self {
            focus_score: 0.0,
            task_completion_score: 0.0,
            consistency_score: 0.0,
            overall_score: 0.0,
            streak_days: 0,
            last_activity_date: None,
        }
    }

    pub fn update_scores(&mut self, focus: f64, task_completion: f64, consistency: f64) {
        self.focus_score = focus.clamp(0.0, 100.0);
        self.task_completion_score = task_completion.clamp(0.0, 100.0);
        self.consistency_score = consistency.clamp(0.0, 100.0);
        self.overall_score =
            (self.focus_score + self.task_completion_score + self.consistency_score) / 3.0;
    }

    pub fn update_streak(&mut self, current_date: u64) {
        if let Some(last_date) = self.last_activity_date {
            // Check if consecutive day (within 24 hours)
            if current_date - last_date <= 86400 {
                self.streak_days += 1;
            } else if current_date - last_date > 86400 * 2 {
                // Streak broken
                self.streak_days = 1;
            }
        } else {
            self.streak_days = 1;
        }
        self.last_activity_date = Some(current_date);
    }
}

impl Default for ProductivityScore {
    fn default() -> Self {
        Self::new()
    }
}

/// Gamified productivity system
pub struct GamifiedProductivity {
    pub achievements: HashMap<String, Achievement>,
    pub goals: HashMap<String, Goal>,
    pub pomodoro_timer: PomodoroTimer,
    pub productivity_score: ProductivityScore,
    pub level: u32,
    pub experience_points: u64,
}

impl GamifiedProductivity {
    pub fn new() -> Self {
        let mut system = Self {
            achievements: HashMap::new(),
            goals: HashMap::new(),
            pomodoro_timer: PomodoroTimer::new(),
            productivity_score: ProductivityScore::new(),
            level: 1,
            experience_points: 0,
        };

        system.add_default_achievements();
        system
    }

    fn add_default_achievements(&mut self) {
        let first_pomodoro = Achievement::new(
            "first_pomodoro".to_string(),
            "First Pomodoro".to_string(),
            AchievementType::FocusTime,
            1,
        )
        .with_description("Complete your first Pomodoro session".to_string())
        .with_icon("🍅".to_string());

        let focus_master = Achievement::new(
            "focus_master".to_string(),
            "Focus Master".to_string(),
            AchievementType::FocusTime,
            100,
        )
        .with_description("Complete 100 Pomodoro sessions".to_string())
        .with_icon("🎯".to_string());

        let task_achiever = Achievement::new(
            "task_achiever".to_string(),
            "Task Achiever".to_string(),
            AchievementType::TasksCompleted,
            50,
        )
        .with_description("Complete 50 tasks".to_string())
        .with_icon("✅".to_string());

        let goal_getter = Achievement::new(
            "goal_getter".to_string(),
            "Goal Getter".to_string(),
            AchievementType::GoalsReached,
            10,
        )
        .with_description("Reach 10 goals".to_string())
        .with_icon("🎖️".to_string());

        self.achievements
            .insert(first_pomodoro.id.clone(), first_pomodoro);
        self.achievements
            .insert(focus_master.id.clone(), focus_master);
        self.achievements
            .insert(task_achiever.id.clone(), task_achiever);
        self.achievements
            .insert(goal_getter.id.clone(), goal_getter);
    }

    pub fn add_goal(&mut self, goal: Goal) {
        self.goals.insert(goal.id.clone(), goal);
    }

    pub fn update_goal(&mut self, id: &str, value: u64) {
        let goal_opt: Option<&mut Goal> = self.goals.get_mut(id);
        if let Some(goal) = goal_opt {
            let goal: &mut Goal = goal;
            goal.update_progress(value);
            if goal.completed {
                self.add_experience(100);
            }
        }
    }

    pub fn add_achievement(&mut self, achievement: Achievement) {
        self.achievements
            .insert(achievement.id.clone(), achievement);
    }

    pub fn update_achievement(&mut self, id: &str, value: u64) {
        let achievement_opt: Option<&mut Achievement> = self.achievements.get_mut(id);
        if let Some(achievement) = achievement_opt {
            let was_unlocked = achievement.unlocked;
            achievement.update_progress(value);

            if !was_unlocked && achievement.unlocked {
                self.add_experience(500);
            }
        }
    }

    pub fn add_experience(&mut self, points: u64) {
        self.experience_points += points;

        // Level up every 1000 XP
        while self.experience_points >= self.level as u64 * 1000 {
            self.level += 1;
        }
    }

    pub fn get_level_progress(&self) -> f64 {
        let current_level_xp = self.level as u64 * 1000;
        let next_level_xp = (self.level + 1) as u64 * 1000;
        let progress = self.experience_points - current_level_xp;
        let total_needed = next_level_xp - current_level_xp;

        if total_needed == 0 {
            return 100.0;
        }

        (progress as f64 / total_needed as f64 * 100.0).min(100.0)
    }

    pub fn get_pomodoro_timer(&mut self) -> &mut PomodoroTimer {
        &mut self.pomodoro_timer
    }

    pub fn update_pomodoro(&mut self) {
        self.pomodoro_timer.update();

        if self.pomodoro_timer.pomodoros_completed > 0 {
            self.update_achievement(
                "first_pomodoro",
                self.pomodoro_timer.pomodoros_completed as u64,
            );
            self.update_achievement(
                "focus_master",
                self.pomodoro_timer.pomodoros_completed as u64,
            );
        }
    }

    pub fn get_goals(&self) -> Vec<&Goal> {
        self.goals.values().collect()
    }

    pub fn get_achievements(&self) -> Vec<&Achievement> {
        self.achievements.values().collect()
    }
}

impl Default for GamifiedProductivity {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_pomodoro_creation() {
        let timer = PomodoroTimer::new();
        assert_eq!(timer.state, PomodoroState::Stopped);
        assert_eq!(timer.work_duration, Duration::from_secs(25 * 60));
    }

    #[test]
    fn test_pomodoro_start() {
        let mut timer = PomodoroTimer::new();
        timer.start_work();
        assert_eq!(timer.state, PomodoroState::Work);
        assert!(timer.running);
    }

    #[test]
    fn test_achievement_progress() {
        let mut achievement = Achievement::new(
            "test".to_string(),
            "Test".to_string(),
            AchievementType::FocusTime,
            10,
        );
        achievement.update_progress(5);
        assert_eq!(achievement.get_progress_percentage(), 50.0);
        assert!(!achievement.unlocked);

        achievement.update_progress(10);
        assert!(achievement.unlocked);
    }

    #[test]
    fn test_goal_creation() {
        let goal = Goal::new("test".to_string(), "Test Goal".to_string(), 100);
        assert_eq!(goal.target_value, 100);
        assert!(!goal.completed);
    }

    #[test]
    fn test_productivity_system() {
        let system = GamifiedProductivity::new();
        assert_eq!(system.achievements.len(), 4);
        assert_eq!(system.level, 1);
    }

    #[test]
    fn test_experience_and_leveling() {
        let mut system = GamifiedProductivity::new();
        system.add_experience(500);
        assert_eq!(system.experience_points, 500);
        assert_eq!(system.level, 1);

        system.add_experience(500);
        assert_eq!(system.level, 2);
    }
}
