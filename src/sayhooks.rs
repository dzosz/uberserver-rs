use log::{debug, error, info};
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::{Duration, Instant};

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub struct SayHooks {
    bad_word_dict: HashMap<String, String>,
    bad_site_list: HashSet<String>,
    bad_nick_list: HashSet<String>,
}

impl SayHooks {

    fn load(&mut self) {
        self.load_bad_words("bad_words.txt");
        self.load_bad_sites("bad_sites.txt");
        self.load_bad_nicks("bad_nicks.txt");
    }

    fn load_bad_words(&mut self, file_name: &str) {
        match read_lines(file_name) {
            Ok(lines) => {
                for line in lines {
                    if let Ok(line) = line {
                        let line = line.trim();
                        if line.is_empty() {
                            continue;
                        }
                        match line.split_once(' ') {
                            Some((left, right)) => {
                                self.bad_word_dict.insert(left.to_string(), right.to_string());
                            }
                            None => {
                                self.bad_word_dict.insert(line.to_string(), "***".to_string());
                            }
                        }
                    }
                }
                info!("Profanity list loaded with {} entries", self.bad_word_dict.len());
            }
            Err(e) => {
                error!("Error parsing profanity list: File {}: {}", file_name, e);
            }
        }
    }

    fn load_bad_sites(&mut self, file_name: &str) {
        match read_lines(file_name) {
            Ok(lines) => {
                for line in lines {
                    if let Ok(line) = line {
                        let line = line.trim().to_lowercase();
                        if ! line.is_empty() {
                            self.bad_site_list.insert(line);
                        }
                    }
                }
            }
            Err(e) => {
                error!("Error parsing shock site list: File {}: {}", file_name, e);
            }
        }
    }
    fn load_bad_nicks(&mut self, file_name: &str) {
        match read_lines(file_name) {
            Ok(lines) => {
                for line in lines {
                    if let Ok(line) = line {
                        let line = line.trim().to_lowercase();
                        if ! line.is_empty() {
                            self.bad_nick_list.insert(line);
                        }
                    }
                }
            }
            Err(e) => {
                error!("Error parsing bad nick list: File {}: {}", file_name, e);
            }
        }
    }
}

struct HistoricalMessage {
    time : Instant,
    message_hash : u64,
    message_length : usize
}

impl HistoricalMessage {
    fn new(message : &str) -> Self {
        let mut s = DefaultHasher::new();
        message.hash(&mut s);
        let hash = s.finish();
        HistoricalMessage { time: Instant::now(), message_hash : hash, message_length : message.len()}
    }
}

pub struct SpamHandler {
    lastsaid : HashMap<String, VecDeque<HistoricalMessage>>
}

impl SpamHandler {
    fn spamrec(&mut self, channel : &str, message : &str) {
        let msg = HistoricalMessage::new(message);
        let chan_history = self.lastsaid.entry(channel.to_string()).or_default();
        chan_history.push_back(msg);
    }

    // TODO this is so much work for each message...
    // TODO write test for spamenum
    fn spam_enum(&mut self, channel : &str) -> bool {
        let mut already = HashMap::new(); // TODO reuse existing chan_history from beginning to current
                                      // iter instead
        let chan_history = self.lastsaid.entry(channel.to_string()).or_default();
        let now = Instant::now();
        chan_history.retain(|elem| now - elem.time < Duration::from_secs(5));

        let mut bonus : f32 = 0.0;


        for msg in chan_history.iter() {
            let counter = already.entry(&msg.message_hash).or_insert(0);
            bonus += (2 * *counter) as f32;
            if msg.message_length > 50 {
                bonus += std::cmp::min(msg.message_length, 200) as f32 * 0.01;
            }
            bonus += 1.0; // something was said
            *counter += 1;
        }

        let mut last_time = chan_history.get(0).unwrap().time; 
        for msg in chan_history.iter().skip(1) {
			let diff = msg.time - last_time;
            if diff < Duration::from_secs(1) {
				bonus += (1.0 - diff.as_secs_f32()) * 1.5;
            }
            last_time = msg.time;
        }
        bonus > 7.0
    }
}

