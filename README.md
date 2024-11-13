TODOs:
- [ ] Write real README 
- [x] Fix Github Actions
- [ ] Make Win/Lose box a real dialog that's pretty

Features:
    - [x] Edit This Puzzle
    - [ ] Settings(?) -> Restart
    - [ ] Timer(?)
    - [ ] Try a Random Connection
    - [ ] Responsive Mobile

Design changes:
 - [ ] remove paramatrization of Color
 - [ ] better interface for references to puzzles & representation of location
     - [ ] ideally one that lends itself better to how we access it
 - [ ] put `dom.rs` behind more robust interface 
 - [ ] abstract frontend behind a trait so `dom.rs` is swappable
 - [ ] use statics to move more javascript into rust 
 - [ ] replace game.js and index.js with ES6 modules

BugFixes: 
    - [x] Try Again
    - [x] One Away
