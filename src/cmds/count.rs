use std::cmp::Ordering;
use std::time::{Instant, SystemTime};
use rand::prelude::{StdRng, SeedableRng};
use rand::Rng;
use crate::Commandable;
use crate::message::Message;
use crate::util::minigames::{CountPlayers, CountPlayer, COUNT_PLAYERS, Team};

pub struct Command;

impl Commandable for Command {
    fn exec(&self, msg: &Message) -> String {
        let now = Instant::now();
        let mut players = COUNT_PLAYERS.lock();
        let now_coarse = SystemTime::now();
        if should_reset(now_coarse, players.last_use).is_some() {
            players.reset();
        }

        let count = players.count;
        let plus = if count.is_positive() { "+" } else { "" };
        let player = match players.players.get_mut(&msg.id) {
            Some(player) if player.target_sec != 0 => player,
            Some(player) => {
                let mut rng = StdRng::from_entropy();
                let wait_sec = rng.gen_range(CountPlayers::MIN_DELAY..=CountPlayers::MAX_DELAY);
                player.target_sec = wait_sec;
                player.time = Instant::now();
                let team = player.team;
                return format!("You are on team `{team}`. The counter is currently `{plus}{count}`.\n\
                                \n\
                                Wait for `{wait_sec}` seconds, then use this command again to earn points!");
            }
            None => {
                let mut rng = StdRng::from_entropy();
                let team = Team::random(&mut rng);
                let goal = CountPlayers::GOAL;
                let wait_sec = rng.gen_range(CountPlayers::MIN_DELAY..=CountPlayers::MAX_DELAY);
                let player = CountPlayer {
                    team,
                    time: Instant::now(),
                    target_sec: wait_sec,
                };
                let resp = format!(
                    "Welcome! You are on team `{team}`. The counter is currently `{plus}{count}`.\n\
                     Your goal is to get the counter to {team}{goal}\n\
                     \n\
                     Wait for `{wait_sec}` seconds, then use this command again to earn points!");
                players.players.insert(msg.id, player);
                return resp;
            }
        };

        let (points, waited_secs, order) = player.get_points(now);
        let mut resp = if points.is_positive() {
            let mut resp = match order {
                Ordering::Equal => {
                    let target = player.target_sec;
                    format!("You waited exactly `{target}` seconds!\n")
                }
                _ => format!("You waited `{waited_secs:.1}` seconds\n"),
            };
            resp.push_str(&format!("You earned {points} points for your team.\n"));
            resp
        } else {
            let length = if matches!(order, Ordering::Greater) { "long" } else { "short" };
            let lost_x = if points.is_negative() { format!("lost {points}") } else { "didn't gain any".to_owned() };
            format!("You waited for too {length}! ({waited_secs:.1}s)\n\
                     Your team {lost_x} points.\n")
        };
        player.target_sec = 0;
        let team = player.team;
        let game_state = players.offset_points(team, points);
        let count = players.count;
        let plus = if count.is_positive() { "+" } else { "" };

        if game_state.is_break() {
            resp = format!("GAME OVER! The count has reached `{plus}{count}`. \n\
                            Team `{team}` wins!\n\
                            The game will now reset");
            players.reset();
        } else {
            resp.push_str(&format!("The count is now at `{plus}{count}`."));
        }
        resp
    }

    fn usage<'a>(&self) -> &'a str {
        "count"
    }

    fn desc<'a>(&self) -> &'a str {
        "tug of war"
    }

    fn public_channel_only(&self) -> bool {
        true
    }
}

fn should_reset(now: SystemTime, last_use: Option<SystemTime>) -> Option<()> {
    // No need to reset if:
    //   - last_use is None
    //   - duration_since is Err
    //   - duration < 1 day
    let dur = now.duration_since(last_use?).ok()?;
    (dur.as_secs() / 60 >= 15).then_some(())  // reset if 15 minutes without usage
}
