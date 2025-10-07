pub type Sender<T> = kanal::Sender<T>;
pub type Receiver<T> = kanal::Receiver<T>;
pub type RecvTimeoutError = kanal::ReceiveErrorTimeout;
pub type SendError = kanal::SendError;

///
/// Creates a new sync bounded channel with the requested buffer size,
/// and returns Sender and Receiver of the channel for type T,
pub fn channel_bounded<T>(size: usize) -> (Sender<T>, Receiver<T>) {
    kanal::bounded(size)
}
///
/// Creates a new sync bounded channel with the requested buffer size,
/// and returns Sender and Receiver of the channel for type T,
pub fn channel_unbounded<T>() -> (Sender<T>, Receiver<T>) {
    kanal::unbounded()
}