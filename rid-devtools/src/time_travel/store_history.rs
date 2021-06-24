#![allow(unused)] // will be used via generated store code
use std::time::SystemTime;

use super::time_stamp_map::TimeStampMap;

/// All timestamps are suffixed `_ts` and except for [start_ts] are in milliseconds
pub struct StoreHistory<TState, TMsg, TReply>
where
    TState: Clone,
    TMsg: Clone,
    TReply: Clone,
{
    /// Recorded states of the store
    pub states: TimeStampMap<TState>,

    /// Recorded messages sent to the store
    pub messages: TimeStampMap<TMsg>,

    /// Recorded replies sent from the store
    pub replies: TimeStampMap<TReply>,

    /// Time when this StoreHistory was created
    start: SystemTime,

    /// Minimum millisecond delta between two recorded states
    state_delta: u16,

    /// Tracks when the last state was recorded.
    last_state_ts: Option<u128>,

    /// Minimum millisecond delta between two recorded replies.
    /// For replies to user messages this will not be throttling, but for replies created by a
    /// worker on another threads, i.e. ticks it might.
    reply_delta: u16,

    /// Tracks when the last reply was recorded.
    last_reply_ts: Option<u128>,

    /// Milliseconds from App Start to the last recorded history item
    pub last_recording_ts: Option<u128>,

    /// Milliseconds at which we currently are in the history, by default this is equal
    /// [last_recording_ts]
    pub cursor_ts: Option<u128>,
}

impl<TState: Clone, TMsg: Clone, TReply: Clone>
    StoreHistory<TState, TMsg, TReply>
{
    // -----------------
    // Constructor
    // -----------------

    /// Creates a [StoreHistory] with the following properties.
    ///
    /// - `state_resolution` - max allowed state records per second
    /// - `reply_resolution` - max allowed reply records per second
    pub fn new(state_resolution: u16, reply_resolution: u16) -> Self {
        assert!(
            state_resolution < 1000,
            "max state_resolution is 1000 states/second"
        );
        assert!(
            reply_resolution < 1000,
            "max reply_resolution is 1000 replys/second"
        );

        let state_delta_millis: u16 = 1000 / state_resolution;
        let reply_delta_millis: u16 = 1000 / reply_resolution;

        Self {
            states: TimeStampMap::new(),
            messages: TimeStampMap::new(),
            replies: TimeStampMap::new(),
            start: SystemTime::now(),
            state_delta: state_delta_millis,
            last_state_ts: None,
            reply_delta: reply_delta_millis,
            last_reply_ts: None,
            last_recording_ts: None,
            cursor_ts: None,
        }
    }

    // -----------------
    // Recording
    // -----------------

    pub fn record_state(&mut self, state: &TState) {
        let time_stamp = self.time_stamp();
        let should_record = self.last_state_ts.map_or(true, |last_state_ts| {
            let dt = time_stamp - last_state_ts;
            dt >= self.state_delta.into()
        });

        if should_record {
            self.remove_recordings_after_cursor();

            self.last_state_ts = Some(time_stamp);
            self.states.insert(time_stamp, state.clone());
            self.last_recording_ts = Some(time_stamp);
            self.cursor_ts = self.last_recording_ts;
        }
    }

    pub fn record_msg(&mut self, msg: &TMsg) {
        self.remove_recordings_after_cursor();

        let time_stamp = self.time_stamp();
        self.messages.insert(time_stamp, msg.clone());
        self.last_recording_ts = Some(time_stamp);
        self.cursor_ts = self.last_recording_ts;
    }

    pub fn record_reply(&mut self, reply: &TReply) {
        let time_stamp = self.time_stamp();
        let should_record = self.last_reply_ts.map_or(true, |last_reply_ts| {
            let dt = time_stamp - last_reply_ts;
            dt >= self.reply_delta.into()
        });

        if should_record {
            self.remove_recordings_after_cursor();

            self.last_reply_ts = Some(time_stamp);
            self.replies.insert(time_stamp, reply.clone());
            self.last_recording_ts = Some(time_stamp);
            self.cursor_ts = self.last_recording_ts;
        }
    }

    // TODO(tt): need to make tick_like _replies_ use a store::write that needs to ignore the tick while
    // the user is scrubbing history.
    // The write used by incoming messages however needs to trigger a recording and thus the below.
    // However what if there are msgs on Dart that are triggered automatically?

    /// When we record a value, that means that the user interacted with the application outside of
    /// moving the time travel slider.
    /// In that case we must assume we want to continue from that spot and thus forget all states,
    /// messages and replies that follow.
    fn remove_recordings_after_cursor(&mut self) {
        match self.cursor_ts {
            Some(cursor_ts) if self.cursor_ts != self.last_recording_ts => {
                self.replies.remove_items_gt(cursor_ts);
                self.messages.remove_items_gt(cursor_ts);
                self.states.remove_items_gt(cursor_ts);
                self.cursor_ts = self.last_recording_ts;
            }
            _ => {}
        }
    }

    // -----------------
    // Mapping functions
    // -----------------
    pub fn state_change_timestamps(&self) -> Vec<u128> {
        self.states.keys().into_iter().map(|x| x.clone()).collect()
    }

    // -----------------
    // Wrappers around [TimeStampMap] queries
    // -----------------

    // State
    pub fn state_ge(&self, ts: u128) -> Option<&TState> {
        self.states.item_ge(ts)
    }

    pub fn state_gt(&self, ts: u128) -> Option<&TState> {
        self.states.item_gt(ts)
    }

    pub fn state_le(&self, ts: u128) -> Option<&TState> {
        self.states.item_le(ts)
    }

    pub fn state_lt(&self, ts: u128) -> Option<&TState> {
        self.states.item_lt(ts)
    }

    pub fn states_up_to(&self, ts: u128) -> Vec<&TState> {
        self.states.items_up_to(ts)
    }

    // Replies
    pub fn reply_ge(&self, ts: u128) -> Option<&TReply> {
        self.replies.item_ge(ts)
    }

    pub fn reply_gt(&self, ts: u128) -> Option<&TReply> {
        self.replies.item_gt(ts)
    }

    pub fn reply_le(&self, ts: u128) -> Option<&TReply> {
        self.replies.item_le(ts)
    }

    pub fn reply_lt(&self, ts: u128) -> Option<&TReply> {
        self.replies.item_lt(ts)
    }

    pub fn replies(&self, ts: u128) -> Vec<&TReply> {
        self.replies.items_up_to(ts)
    }

    // Messages
    pub fn message_ge(&self, ts: u128) -> Option<&TMsg> {
        self.messages.item_ge(ts)
    }

    pub fn message_gt(&self, ts: u128) -> Option<&TMsg> {
        self.messages.item_gt(ts)
    }

    pub fn message_le(&self, ts: u128) -> Option<&TMsg> {
        self.messages.item_le(ts)
    }

    pub fn message_lt(&self, ts: u128) -> Option<&TMsg> {
        self.messages.item_lt(ts)
    }

    pub fn messages_up_to(&self, ts: u128) -> Vec<&TMsg> {
        self.messages.items_up_to(ts)
    }

    // -----------------
    // Utilities
    // -----------------
    fn time_stamp(&self) -> u128 {
        SystemTime::now()
            .duration_since(self.start)
            .unwrap()
            .as_millis()
    }
}

#[cfg(test)]
mod tests {
    use std::{ops::Deref, thread, time::Duration};

    use super::StoreHistory;
    #[derive(Debug, Clone, PartialEq)]
    enum Msg {
        Add,
        Remove,
        Complete,
    }
    #[derive(Debug, Clone, PartialEq)]
    enum Reply {
        Added,
        Removed,
        Completed,
    }

    #[test]
    fn recording_state() {
        let state_resolution = 100; // 10ms each
        let mut hist = StoreHistory::<u8, Msg, Reply>::new(state_resolution, 1);

        hist.record_state(&1);
        hist.record_state(&2);

        let states = hist.states.values().collect::<Vec<&u8>>();
        assert_eq!(states.len(), 1, "records one state if too close together");
        assert_eq!(states[0], &1, "records the first state only");

        thread::sleep(Duration::from_millis(20));
        hist.record_state(&3);

        let states = hist.states.values().collect::<Vec<&u8>>();

        assert_eq!(
            states.len(),
            2,
            "records another state if enough time passed"
        );
        assert_eq!(states[1], &3, "records the second state");
    }

    #[test]
    fn recording_replies() {
        use Reply::*;

        let reply_resolution = 100; // 10ms each
        let mut hist = StoreHistory::<u8, Msg, Reply>::new(1, reply_resolution);

        hist.record_reply(&Added);
        hist.record_reply(&Removed);

        let replies = hist.replies.values().collect::<Vec<&Reply>>();
        assert_eq!(replies.len(), 1, "records one reply if too close together");
        assert_eq!(replies[0], &Added, "records the first reply only");

        thread::sleep(Duration::from_millis(20));
        hist.record_reply(&Completed);

        let replies = hist.replies.values().collect::<Vec<&Reply>>();

        assert_eq!(
            replies.len(),
            2,
            "records another reply if enough time passed"
        );
        assert_eq!(replies[1], &Completed, "records the second reply");
    }
}
