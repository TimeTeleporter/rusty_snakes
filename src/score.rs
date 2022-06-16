



















































/*
#[derive(Clone,Copy)]
pub struct Player {
    wins: i32,
    mean: f32,
    times_hit: i32,
}

impl Player {
    pub fn new() -> Player {
        Player {wins: 0, mean: 0., times_hit: 0}
    }

    pub fn has_won(&mut self, turns: i32) {
        let old_wins = self.wins as f32;
        self.wins += 1;
        self.mean = (old_wins * self.mean + turns as f32)/(old_wins+1.);
    }

    pub fn got_hit(&mut self) {
        self.times_hit += 1;
    }

    pub fn print(&self, player_nr: usize, games: u32) {
        println!("Player {} has won {} times ({:.2}%) with an average of {} turns. He got hit {} times.", 
                player_nr, 
                self.wins, 
                (self.wins as f32) / (games as f32) * 100., 
                self.mean, 
                self.times_hit
            );
    }
}
*/