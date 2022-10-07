// In this code sample we will implement a prime number game
// Rules of the Game:
// - System displays a number between predefined LB and UB.
// - LB and UB are defined as per the level currently user is playing.
// - User have to guess the next prime number of the displayed number.
// - Number of retries is defined in the system and depends on the level.
// - When the number is retry is exhausted the game is over.
// - Level 1: LB = 1000, UB = 2000, retry = 5;
// - Level 2: LB = 2000, UB = 3000, retry = 7 + (level 1 leftovers)
// - Level 3: LB = 3000, UB = 4000, retry = 9 + (level 2 leftovers)
// - Level N: LB = N*1000, UB = (N+1)*1000, retry = (2N + 3) + level[N-1] leftovers
// Special input to quit the game anytime
// Special input the restart the game anytime

mod game;

fn main() {}
