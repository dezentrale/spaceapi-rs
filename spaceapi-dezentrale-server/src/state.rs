use std::{
    sync::Arc,
    time::{Duration, SystemTime},
};
use tokio::sync::RwLock;

pub enum LastOpenRequest {
    Open,
    KeepOpen(SystemTime),
}

pub struct SpaceState {
    pub open: bool,
    pub last_open_request: LastOpenRequest,
    pub keep_open_interval: Duration,
}

#[derive(Clone)]
pub struct SpaceGuard(Arc<RwLock<SpaceState>>);

impl SpaceGuard {
    pub fn new(keep_open_interval: Duration) -> Self {
        SpaceGuard(Arc::new(RwLock::new(SpaceState {
            open: false,
            last_open_request: LastOpenRequest::Open,
            keep_open_interval,
        })))
    }

    pub async fn open(&self) {
        let mut space = self.0.write().await;
        space.open = true;
        space.last_open_request = LastOpenRequest::Open;
        log::debug!("Space set open");
    }

    pub async fn close(&self) {
        let mut space = self.0.write().await;
        space.open = false;
        log::debug!("Space set closed");
    }

    pub async fn is_open(&self) -> bool {
        let space = self.0.read().await;
        log::trace!("Space status requested and is {}", space.open);
        space.open
    }

    pub async fn keep_open(&self) -> SystemTime {
        let mut space = self.0.write().await;
        space.open = true;
        let now = SystemTime::now();
        let open_till = now.checked_add(space.keep_open_interval).unwrap();
        space.last_open_request = LastOpenRequest::KeepOpen(open_till);
        log::trace!("Space requested to keep open and it will till {open_till:?}");
        open_till
    }

    pub async fn check_keep_open(&self, now: SystemTime) {
        log::trace!("Checking keep open status at {now:?}");
        let space = self.0.read().await;
        if !space.open {
            return;
        }
        if let LastOpenRequest::KeepOpen(open_till) = space.last_open_request {
            if now <= open_till {
                return;
            }
        } else {
            return;
        }
        // drop to free lock
        drop(space);
        self.close().await;
    }

    pub async fn start_scheduler(&self, tick_interval: Duration) {
        let instance = self.clone();
        tokio::spawn(async move {
            loop {
                instance.check_keep_open(SystemTime::now()).await;
                tokio::time::sleep(tick_interval).await;
            }
        });
    }
}

impl Default for SpaceGuard {
    fn default() -> Self {
        SpaceGuard::new(Duration::from_secs(300))
    }
}
