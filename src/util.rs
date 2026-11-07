//! vokoscreen model — GTK screen recorder.

#[derive(Debug, Clone)]
pub struct Recording {
    pub id: String,
    pub path: String,
    pub region: String,
    pub start: u64,
    pub duration: u64,
    pub status: String,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub recordings: Vec<Recording>,
    pub next_id: u64,
}

impl Default for Model {
    fn default() -> Self { Self { recordings: vec![], next_id: 1 } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn start(&mut self, path: &str, region: &str, start: u64) -> String {
        let id = format!("{:x}", self.next_id);
        self.next_id += 1;
        self.recordings.push(Recording { id: id.clone(), path: path.into(), region: region.into(), start, duration: 0, status: "recording".into() });
        id
    }

    pub fn stop(&mut self, id: &str) -> bool {
        if let Some(r) = self.recordings.iter_mut().find(|r| r.id == id) { r.status = "stopped".into(); true } else { false }
    }

    pub fn list(&self) -> &[Recording] { &self.recordings }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn recording_start_stop() {
        let mut v = Model::new();
        let id = v.start("/tmp/rec.mp4", "full", 1000);
        assert!(v.stop(&id));
    }
}
