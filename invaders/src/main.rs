use std::{error::Error, io, time::{Duration, Instant}, sync::mpsc};

use crossterm::{
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen}, 
    ExecutableCommand, cursor::{Hide, Show}, 
    event::{Event, KeyCode}};

use invaders::{
    render, 
    frame::{self, new_frame, Drawable},
    player::Player, invaders::Invaders};

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

    // Render loop in a separate thread
    let (render_tx, render_rx) = mpsc::channel();
    let render_handle = std::thread::spawn(move || {
        let mut last_frame = frame::new_frame();
        let mut stdout = io::stdout();
        render::render(&mut stdout, &last_frame, &last_frame, true);
        loop {
            let curr_frame = match render_rx.recv() {
                Ok(x) => x,
                Err(_) => break,
            };
            render::render(&mut stdout, &last_frame, & curr_frame, false);
            last_frame = curr_frame;
        }
    });

    ///////////////
    // GAME LOOP //
    ///////////////
    let mut player = Player::new();
    let mut instant = Instant::now();
    let mut invaders = Invaders::new();
    'game_loop: loop {
        // Per-frame init
        let delta = instant.elapsed();
        instant = Instant::now();
        let mut curr_frame = new_frame();

        // Input
        while crossterm::event::poll(Duration::default())? {
            if let Event::Key(key_event) = crossterm::event::read()? {
                match key_event.code {
                    KeyCode::Left => player.move_left(),
                    KeyCode::Right => player.move_right(),
                    KeyCode::Char(' ') => {
                        if player.shoot() {
                            audio.play("pew");
                        }
                    }
                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio.play("lose");
                        break 'game_loop
                    }
                    _ => {}
                }
            }
        }

        // Updates
        player.update(delta);
        if invaders.update(delta) {
            audio.play("move");
        }

        if player.detect_hits(&mut invaders) {
            audio.play("explode");
        }

        // Draw and render
        let drawables: Vec<&dyn Drawable> = vec![&player, &invaders];
        for drawable in drawables {
            drawable.draw(&mut curr_frame);
        }
        let _ = render_tx.send(curr_frame);
        std::thread::sleep(Duration::from_millis(1));

        // Win or lose condition
        if invaders.all_killed() {
            audio.play("win");
            break 'game_loop;
        }
        if invaders.reached_bottom() {
            audio.play("lost");
            break 'game_loop;
        }
    }
    
    /////////////
    // CLEANUP //
    /////////////
    
    drop(render_tx);
    render_handle.join().unwrap();

    /* (a) Wait until all audio is done playing before thread
    is shut down, else sounds won't play. */
    audio.wait();

    /* (b) Show the cursor, leave game screen, disable keyboard input. */
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(()) 
}
