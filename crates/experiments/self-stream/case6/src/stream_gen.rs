use std::marker::PhantomData;
use std::pin::Pin;
use std::task::{Context, Poll};

use actix::prelude::*;
use futures_util::ready;
use futures_util::stream::{StreamExt, Take};
use tokio::time;
use tokio_stream::wrappers::IntervalStream;

enum StreamGenState<S> {
    Polling(S),
    Waiting(S),
}

pub struct StreamGen<S, V> {
    stream: Option<StreamGenState<S>>,
    interval: Take<IntervalStream>,
    _phantom: PhantomData<V>,
}

impl<S, V> StreamGen<S, V> {
    pub fn new(stream: S, count: usize) -> Self {
        let interval = time::interval(time::Duration::from_millis(200));

        Self {
            stream: Some(StreamGenState::Waiting(stream)),
            interval: IntervalStream::new(interval).take(count),
            _phantom: PhantomData,
        }
    }

    pub fn stream_mut(&mut self) -> &mut S {
        match self.stream.as_mut() {
            Some(StreamGenState::Waiting(stream)) => stream,
            Some(StreamGenState::Polling(stream)) => stream,
            _ => panic!("invalid state"),
        }
    }
}

impl<S: Unpin, V> Unpin for StreamGen<S, V> {}

impl<S, V> Stream for StreamGen<S, V>
where
    S: Stream<Item: Into<V>> + Unpin,
{
    type Item = V;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let is_polling = matches!(self.stream, Some(StreamGenState::Polling(_)));

        if !is_polling {
            let Some(_) = ready!(self.interval.poll_next_unpin(cx)) else {
                return Poll::Ready(None);
            };

            let Some(StreamGenState::Waiting(stream)) = self.stream.take() else {
                panic!("invalid state");
            };

            self.stream = Some(StreamGenState::Polling(stream));
        }

        let Some(StreamGenState::Polling(stream)) = self.stream.as_mut() else {
            panic!("invalid state");
        };

        let res = ready!(stream.poll_next_unpin(cx));

        let Some(StreamGenState::Polling(stream)) = self.stream.take() else {
            panic!("invalid state");
        };

        self.stream = Some(StreamGenState::Waiting(stream));

        Poll::Ready(res.map(|value| value.into()))
    }
}

impl<S, V> Drop for StreamGen<S, V> {
    fn drop(&mut self) {
        println!("dropping the swarm");
    }
}
