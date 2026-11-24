#[derive(Clone, Debug, Default)]
pub struct Animator<S> {
    time: f32,
    state: S,
}

// ── Associated Functions ─────────────────────────────────────────────────────────────────

impl<S> Animator<S> {
    pub fn new(state: S) -> Self {
        Self { state, time: 0.0 }
    }
}

// ── Methods ─────────────────────────────────────────────────────────────────

impl<S> Animator<S> {
    pub fn time(&self) -> f32 {
        self.time
    }

    pub fn state(&self) -> &S {
        &self.state
    }

    pub fn change_to(&mut self, new_state: S) {
        self.reset();
        self.state = new_state;
    }

    pub fn reset(&mut self) {
        self.time = 0.0;
    }
}

// ── Trait Definitions ─────────────────────────────────────────────────────────────────

pub trait AnimationClip {
    fn duration(&self) -> f32;
}

// ── Trait Implementations ─────────────────────────────────────────────────────────────────

impl<S: AnimationClip> Animator<S> {
    pub fn advance(&mut self, dt: f32) -> f32 {
        let duration = self.state.duration();
        self.time += dt;

        if duration > 0.0 {
            self.time %= duration;
        }

        self.time
    }
}
