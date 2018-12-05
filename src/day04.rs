use super::*;

use std::str::FromStr;
use std::collections::{HashMap};

use chrono::{NaiveDateTime, DateTime, Date, Utc};
use regex::Regex;
use itertools::{Itertools};

pub struct Part1<T>(::std::marker::PhantomData<T>);

impl<T> Solve<T> for Part1<T>
    where T: AsRef<str> {
    type Output = Result<u64, Error>;

    fn solve(input: T) -> <Self as Solve<T>>::Output {
        let mut events: Vec<Event> = read_events(input.as_ref())?;

        // sort by guard id
        events.sort_by_key(|e| e.guard_id.unwrap());

        let mut asleep: HashMap<u64, [usize; 60]> = HashMap::new();

        for (key, val) in &events.into_iter()
            .group_by(|event| event.guard_id.unwrap()) {

            let mut times = [0_usize; 60];

            let events: Vec<Event> = val.collect();

            let event_map: HashMap<DateTime<Utc>, EventType> = events.iter()
                .map(|e| (e.time, e.event_type.clone()))
                .collect();

            let mut dates: Vec<Date<Utc>> = events.iter()
                .map(|e| e.time.date())
                .dedup()
                .collect();
            dates.sort();

            for date in dates.iter() {
                let mut is_asleep = false;

                for time in 0_usize..60 {
                    is_asleep = match event_map.get(&date.and_hms(0, time as u32, 0)) {
                        Some(EventType::FallsAsleep) => true,
                        Some(EventType::WakesUp) => false,
                        Some(_) | None => is_asleep,
                    };

                    if is_asleep {
                        times[time] = times[time] + 1;
                    }
                }
            }

            asleep.insert(key, times);
        }

        let (guard_id, _max_asleep) = asleep.iter()
            .map(|(id, times)| (id, times.iter().sum::<usize>()))
            .max_by_key(|(_id, count)| *count)
            .ok_or("Not enough events")?;

        let max_minute = asleep.get(guard_id).unwrap()
            .iter()
            .map(|x| *x)
            .enumerate()
            .max_by_key(|i| i.1)
            .unwrap_or_else(|| (0, 0)).0;

        Ok(*guard_id * max_minute as u64)
    }
}

pub struct Part2<T>(::std::marker::PhantomData<T>);

impl<T> Solve<T> for Part2<T>
    where T: AsRef<str> {
    type Output = Result<u64, Error>;

    fn solve(input: T) -> <Self as Solve<T>>::Output {
        let mut events: Vec<Event> = read_events(input.as_ref())?;

        // sort by guard id
        events.sort_by_key(|e| e.guard_id.unwrap());

        let mut asleep: HashMap<u64, [usize; 60]> = HashMap::new();

        for (key, val) in &events.into_iter()
            .group_by(|event| event.guard_id.unwrap()) {

            let mut times = [0_usize; 60];

            let events: Vec<Event> = val.collect();

            let event_map: HashMap<DateTime<Utc>, EventType> = events.iter()
                .map(|e| (e.time, e.event_type.clone()))
                .collect();

            let mut dates: Vec<Date<Utc>> = events.iter()
                .map(|e| e.time.date())
                .dedup()
                .collect();
            dates.sort();

            for date in dates.iter() {
                let mut is_asleep = false;

                for time in 0_usize..60 {
                    is_asleep = match event_map.get(&date.and_hms(0, time as u32, 0)) {
                        Some(EventType::FallsAsleep) => true,
                        Some(EventType::WakesUp) => false,
                        Some(_) | None => is_asleep,
                    };

                    if is_asleep {
                        times[time] = times[time] + 1;
                    }
                }
            }

            asleep.insert(key, times);
        }

        let (guard_id, (max_idx, _max)) = asleep.iter()
            .map(|(id, times)| (id, times.iter()
                .enumerate()
                .max_by_key(|(_i, x)| *x)
                .unwrap_or((0_usize, &0_usize))))
            .max_by_key(|(_id, (_max_idx, max))| *max)
            .ok_or("Not enouch events")?;

        Ok(*guard_id * max_idx as u64)
    }
}

fn read_events(input: &str) -> Result<Vec<Event>, Error> {
    let mut events: Vec<Event> = input.lines()
        .map(|line| Event::from_str(line))
        .collect::<Result<_, _>>()?;

    events.sort_by(|a, b| a.time.cmp(&b.time));

    // Fill down guard ids
    let mut current_guard_id = match events[0].event_type {
        EventType::BeginsShift(id) => id,
        _ => return Err(From::from("Invalid set of events")),
    };

    for event in events.iter_mut() {
        event.guard_id = Some(match event.event_type {
            EventType::BeginsShift(id) => {
                current_guard_id = id;
                id
            },
            _ => current_guard_id,
        });
    }

    Ok(events)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    time: DateTime<Utc>,
    guard_id: Option<u64>,
    event_type: EventType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventType {
    BeginsShift(u64),
    FallsAsleep,
    WakesUp,
}

impl Event {
    pub fn new(time: DateTime<Utc>, event_type: EventType) -> Self {
        Self {
            time,
            guard_id: None,
            event_type,
        }
    }
}

impl FromStr for Event {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\[(.*?)\] (.*)").unwrap();
        }

        let caps = RE.captures(s).ok_or("Invalid line")?;

        let raw_time = caps.get(1).ok_or("Invalid capture")?.as_str();
        let raw_event = caps.get(2).ok_or("Invalid capture")?.as_str();

        let time: DateTime<Utc> = DateTime::from_utc(
            NaiveDateTime::parse_from_str(raw_time, "%Y-%m-%d %H:%M")?, Utc);

        let event_type = EventType::from_str(raw_event)?;

        Ok(Event::new(time, event_type))
    }
}

impl FromStr for EventType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        match s {
            "falls asleep" => return Ok(EventType::FallsAsleep),
            "wakes up" => return Ok(EventType::WakesUp),
            _ => (),
        };

        lazy_static! {
            static ref RE: Regex = Regex::new(r"Guard #(\d*?) begins shift").unwrap();
        }

        let caps = RE.captures(s).ok_or("Invalid line")?;

        let guard_id = caps.get(1).ok_or("Invalid line")?.as_str().parse::<u64>()?;

        Ok(EventType::BeginsShift(guard_id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use chrono::{TimeZone};

    #[test]
    fn test_parse_event_type() {
        assert_eq!(EventType::WakesUp, EventType::from_str("wakes up").unwrap());
        assert_eq!(EventType::FallsAsleep, EventType::from_str("falls asleep").unwrap());
        assert_eq!(EventType::BeginsShift(23), EventType::from_str("Guard #23 begins shift").unwrap());
    }

    #[test]
    fn test_parse_event() {
        let event1 = Event::new(Utc.ymd(1518, 11, 1).and_hms(0, 0, 0), EventType::BeginsShift(10));
        let event2 = Event::new(Utc.ymd(1518, 11, 1).and_hms(0, 5, 0), EventType::FallsAsleep);
        let event3 = Event::new(Utc.ymd(1518, 11, 1).and_hms(0, 25, 0), EventType::WakesUp);

        assert_eq!(event1, Event::from_str("[1518-11-01 00:00] Guard #10 begins shift").unwrap());
        assert_eq!(event2, Event::from_str("[1518-11-01 00:05] falls asleep").unwrap());
        assert_eq!(event3, Event::from_str("[1518-11-01 00:25] wakes up").unwrap());
    }

    #[test]
    fn test_part1() {
        let raw_input = "[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up";

        assert_eq!(240, Part1::solve(raw_input).unwrap());
    }

    #[test]
    fn test_part2() {
        let raw_input = "[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up";

        assert_eq!(4455, Part2::solve(raw_input).unwrap());
    }
}