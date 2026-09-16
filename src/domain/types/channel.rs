pub type Sender<T> = sal_sync::sync::channel::Sender<T>;
pub type Receiver<T> = sal_sync::sync::channel::Receiver<T>;
pub type RecvTimeoutError = sal_sync::sync::channel::RecvTimeoutError;
pub type SendError = sal_sync::sync::channel::SendError;

///
/// Creates a new sync bounded channel with the requested buffer size,
/// and returns Sender and Receiver of the channel for type T,
pub fn channel_bounded<T>(size: usize) -> (Sender<T>, Receiver<T>) {
    sal_sync::sync::channel::bounded(size)
}
///
/// Creates a new sync bounded channel with the requested buffer size,
/// and returns Sender and Receiver of the channel for type T,
pub fn channel_unbounded<T>() -> (Sender<T>, Receiver<T>) {
    sal_sync::sync::channel::unbounded()
}