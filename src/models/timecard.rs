use std::fmt::Debug;
use std::time::Duration;
use discord_webhook2::error::DiscordWebhookError;
use discord_webhook2::id::DiscordID;
use discord_webhook2::webhook::DiscordWebhook;
use discord_webhook2::message::Message;
use crate::database::player::Player;
use crate::database::timecard_entry::TimecardEntry;
use crate::models::error::Error;
use crate::state;

#[derive(Debug)]
pub struct Timecard {
    owner: Player,
    entries: Vec<TimecardEntry>,
}

impl Timecard {
    pub async fn clock_in(self) -> Result<Self, Error> {
        let pool = crate::state::get_pool();
        let mut is_errored = false;
        match self.entries.last() {
            Some(entry) => {
                if entry.is_clocked_in() {
                    is_errored = true;
                }
            }
            None => {}
        }
        if is_errored {
            return Err(Error::AlreadyClockedIn);
        }
        let entry = TimecardEntry::create(pool, &*self.owner.get_uuid()).await?;
        let mut entries = self.entries;
        entries.push(entry);
        Ok(Timecard {
            owner: self.owner,
            entries,
        })
    }

    fn format_time(time: &Duration) -> String {
        let hours = time.as_millis() / 3600000;
        let minutes = (time.as_millis() % 3600000) / 60000;
        let seconds = (time.as_millis() % 60000) / 1000;
        format!("{}h {}m {}s", hours, minutes, seconds)
    }

    pub async fn clock_out(mut self, desc_vec: Vec<String>) -> Result<Self, Error> {
        let pool = crate::state::get_pool();
        match self.entries.last() {
            Some(entry) => {
                if !entry.is_clocked_in() {
                    return Err(Error::NotClockedIn);
                }
            }
            None => {
                return Err(Error::NotClockedIn);
            }
        }
        let desc: Option<String>;
        if desc_vec.is_empty() {
            desc = None;
        } else {
            desc = Some(desc_vec.join(" "));
        }
        let entry = self.entries.last_mut(); // Should never be None, if it is, then I should have already returned above
        let entry = entry.unwrap().end(pool,desc.clone()).await?;
        let url = state::get_webhook();
        let client = DiscordWebhook::new(url).expect("Failed to create Discord webhook client");
        let sent: Result<DiscordID, DiscordWebhookError>;
        match desc {
            Some(desc) => {
                sent = client.send(&Message::new(|message| message
                    .username("Test Timeclock")
                    .embed(|embed| embed
                        .title(self.owner.get_name())
                        .description(format!("Clocked in: `{}`\nClocked out: `{}`\nTotal time: {}\nDescription: {}", &entry.start_time, &entry.end_time.unwrap(), Self::format_time(&entry.get_total_time()), desc))
                    )
                )).await;
            }
            None => {
                sent = client.send(&Message::new(|message| message
                    .username("Test Timeclock")
                    .embed(|embed| embed
                        .title(self.owner.get_name())
                        .description(format!("Clocked in: `{}`\nClocked out: `{}`\nTotal time: {}", &entry.start_time, &entry.end_time.unwrap(), Self::format_time(&entry.get_total_time())))
                    )
                )).await;
            }
        }

        match sent {
            Ok(_) => (),
            Err(err) => {
                tracing::error!("Failed to send Discord webhook: {}", err);
            }
        }
        Ok(self)
    }
    pub fn is_clocked_in(&self) -> bool {
        self.entries.last().map_or(false, |entry| entry.is_clocked_in())
    }
    pub async fn get(owner: Player) -> Result<Self, sqlx::Error> {
        let pool = crate::state::get_pool();
        let dbentries = TimecardEntry::get_by_user(pool, &*owner.get_uuid()).await;
        match dbentries {
            Ok(entries) => Ok(Timecard { owner, entries }),
            Err(err) => {
                match err {
                    sqlx::Error::RowNotFound => Ok(Timecard {
                        // If no entries are found, create an empty Timecard.
                        owner,
                        entries: Vec::new(),
                    }),
                    _ => Err(err),
                }
            }
        }
    }
}
