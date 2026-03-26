//! Stubs for dioxus-motion (disabled for dioxus 0.7 upgrade).
use std::time::Duration;

pub mod motion {
    use super::Duration;

    #[derive(Clone, Copy, Debug, Default, PartialEq)]
    pub enum LoopMode {
        #[default]
        None,
        Infinite,
    }

    pub mod animations {
        pub mod utils {
            pub use crate::stubs::motion::LoopMode;
        }
    }

    #[derive(Clone, Copy, Debug)]
    pub struct Tween {
        pub duration: Duration,
        pub easing: fn(f32, f32, f32, f32) -> f32,
    }

    impl Default for Tween {
        fn default() -> Self {
            Self {
                duration: Duration::from_millis(0),
                easing: |time, begin, _change, _duration| begin + time,
            }
        }
    }

    #[derive(Clone, Copy, Debug, Default)]
    pub struct Spring {
        pub stiffness: f32,
        pub damping: f32,
    }

    #[derive(Clone, Copy, Debug)]
    pub enum AnimationMode {
        Tween(Tween),
        Spring(Spring),
    }

    impl Default for AnimationMode {
        fn default() -> Self {
            Self::Spring(Spring::default())
        }
    }

    #[derive(Clone, Copy, Debug, Default)]
    pub struct AnimationConfig {
        pub mode: AnimationMode,
        pub loop_mode: Option<LoopMode>,
    }

    impl AnimationConfig {
        pub fn new(mode: AnimationMode) -> Self {
            Self {
                mode,
                loop_mode: None,
            }
        }

        pub fn with_loop(mut self, loop_mode: LoopMode) -> Self {
            self.loop_mode = Some(loop_mode);
            self
        }
    }

    #[derive(Clone, Copy, Debug, Default)]
    pub struct Transform {
        pub x: f32,
        pub y: f32,
        pub scale: f32,
        pub rotation: f32,
        pub opacity: f32,
    }

    impl Transform {
        pub fn new(x: f32, y: f32, scale: f32, rotation: f32) -> Self {
            Self {
                x,
                y,
                scale,
                rotation,
                opacity: 1.0,
            }
        }

        pub fn identity() -> Self {
            Self::new(0.0, 0.0, 1.0, 0.0)
        }
    }

    pub trait AnimationManager<T> {
        fn animate_to(&mut self, target: T, config: AnimationConfig);
        fn get_value(&self) -> T;
    }

    #[derive(Clone, Debug, Default)]
    pub struct AnimationSequence<T> {
        steps: Vec<(T, AnimationConfig)>,
    }

    impl<T> AnimationSequence<T> {
        pub fn new() -> Self {
            Self { steps: Vec::new() }
        }

        pub fn then(mut self, value: T, config: AnimationConfig) -> Self {
            self.steps.push((value, config));
            self
        }
    }

    #[derive(Clone, Copy, Debug)]
    pub struct MotionHandle<T> {
        value: T,
    }

    pub fn use_motion<T: Copy>(initial: T) -> MotionHandle<T> {
        MotionHandle { value: initial }
    }

    impl<T: Copy> MotionHandle<T> {
        pub fn animate_sequence(&mut self, sequence: AnimationSequence<T>) {
            if let Some((value, _)) = sequence.steps.last() {
                self.value = *value;
            }
        }
    }

    impl<T: Copy> AnimationManager<T> for MotionHandle<T> {
        fn animate_to(&mut self, target: T, _config: AnimationConfig) {
            self.value = target;
        }

        fn get_value(&self) -> T {
            self.value
        }
    }

    pub mod prelude {
        pub use crate::stubs::motion::{
            use_motion, AnimationConfig, AnimationManager, AnimationMode, AnimationSequence,
            MotionHandle, Spring, Transform, Tween,
        };
    }
}
