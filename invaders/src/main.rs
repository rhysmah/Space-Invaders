use std::{error::Error, io, time::Duration};
use crossterm::{
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen}, 
    ExecutableCommand, cursor::{Hide, Show}, 
    event::{Event, KeyCode}};
use rusty_audio::Audio;


fn main() -> Result<(), Box<dyn Error>> {
    
    // Import audio files
    // Uses the rusty_audio library (Nathan Stocks)
    let mut audio = Audio::new();
    audio.add("explode", "explode.wav");
    audio.add("lose", "lose.wav");
    audio.add("move", "move.wav");
    audio.add("pew", "pew.wav");
    audio.add("startup", "startup.wav");
    audio.add("win", "win.wav");
    
    audio.play("startup");

    //////////////
    // TERMINAL //
    //////////////
    // Get access to standard out
    let mut stdout = io::stdout();

    // Get keyboard input
    // The "?" will return an error if there is one; if there
    // isn't, then it will proceed with the unwrapped value.
    terminal::enable_raw_mode()?;

    // Enter alternate screen to play game
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?; // Hide cursor

    ///////////////
    // GAME LOOP //
    ///////////////
    'game_loop: loop {
        // Input
        while crossterm::event::poll(Duration::default())? {
            if let Event::Key(key_event) = crossterm::event::read()? {
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio.play("lose");
                        break 'game_loop
                    }
                    _ => {}
                }
            }
        }
    }
    
    /////////////
    // CLEANUP //
    /////////////
    /* (a) Wait until all audio is done playing before thread
    is shut down, else sounds won't play. */
    audio.wait();

    /* (b) Show the cursor, leave game screen, disable keyboard input. */
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(()) 
}
