use std::cell::UnsafeCell;

use tokio::sync::{Mutex, MutexGuard, Semaphore, SemaphorePermit};

pub struct RWAReadGuard<'a> {
    permit: SemaphorePermit<'a>,
    data: &'a [u8],
}

pub struct RWAWriteGuard<'a> {
    permit: SemaphorePermit<'a>,
    data: &'a mut [u8],
}

pub struct RWAAppendGuard<'a> {
    lock: MutexGuard<'a, ()>,
    data: &'a mut [u8],
    in_use_val: &'a mut usize,
    // TODO collect this somehow (probably by turning data into an asyncwriter)
    data_appended: usize,
}

impl<'a> Drop for RWAAppendGuard<'a> {
    fn drop(&mut self) {
        *self.in_use_val += self.data_appended;
    }
}

pub struct RWAInner {
    data: Box<[u8]>,
    in_use: usize,
}

impl RWAInner {
    fn active_data(&self) -> &[u8] {
        &self.data[..self.in_use]
    }

    fn active_data_mut(&mut self) -> &mut [u8] {
        &mut self.data[..self.in_use]
    }

    fn reserve_data_mut(&mut self) -> (&mut [u8], &mut usize) {
        (&mut self.data[self.in_use..], &mut self.in_use)
    }
}

pub struct RWABuffer {
    rwlock: Semaphore,
    alock: Mutex<()>,
    num_permits: u32,
    data: UnsafeCell<RWAInner>,
}

impl RWABuffer {
    fn new(buffer: Box<[u8]>, in_use: usize) -> Self {
        let max_permits = Semaphore::MAX_PERMITS as u32;
        RWABuffer {
            rwlock: Semaphore::new(max_permits as usize),
            alock: Mutex::new(()),
            num_permits: max_permits,
            data: UnsafeCell::new(RWAInner {
                data: buffer,
                in_use,
            }),
        }
    }

    async fn read(&self) -> error::Result<RWAReadGuard<'_>> {
        let permit = self.rwlock.acquire().await?;

        // SAFETY: We acquired the permit to access this data only if it is immutable
        let data = unsafe { &*self.data.get() };

        Ok(RWAReadGuard {
            permit,
            data: data.active_data(),
        })
    }

    async fn write(&self) -> error::Result<RWAWriteGuard<'_>> {
        let permit = self.rwlock.acquire_many(self.num_permits).await?;
        // SAFETY: We acquired the permit to access this data only if it is immutable
        let data = unsafe { &mut *self.data.get() };

        Ok(RWAWriteGuard {
            permit,
            data: data.active_data_mut(),
        })
    }

    /// Get append guard
    async fn append(&self) -> error::Result<RWAAppendGuard> {
        let lock = self.alock.lock().await;
        let data = unsafe { &mut *self.data.get() };
        let (data, in_use_val) = data.reserve_data_mut();

        Ok(RWAAppendGuard {
            lock,
            data,
            in_use_val,
            data_appended: 0,
        })
    }
}
