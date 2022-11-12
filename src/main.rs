use std::process::{Command, exit};
use std::{thread, time};

use std::io::{stdin, stdout, Write};
//use termion::event::{Event, Key, MouseEvent};
use termion::event::{Key, MouseEvent};
//use termion::event;
use termion::input::{MouseTerminal, TermRead};
use termion::raw::IntoRawMode;

use gilrs::{Gilrs, Button, Event};

fn main() {
    let stdin = stdin();
    let mut stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());
     
    write!(
        stdout,
        "{}{}B button to Exit.",
        termion::clear::All,
        termion::cursor::Goto(1,1),
    ).unwrap();

    let mut active_gamepad = None;

    let mut gilrs = Gilrs::new().unwrap();

    // Iterate over all connected gamepads
    for (_id, gamepad) in gilrs.gamepads() {
        write!(stdout, "{}{} is {:?}", termion::cursor::Goto(1,2), gamepad.name(), gamepad.power_info());
        stdout.flush().unwrap();
    }

    let commands = vec!["kodi", "steamlink", "com.valvesoftware.SteamLink"];
    let mut commands = commands.iter().cycle();

    let mut command = "";

    loop {
        while let Some(Event { id, event, time }) = gilrs.next_event() {
            active_gamepad = Some(id);
        }

        if let Some(gamepad) = active_gamepad.map(|id| gilrs.gamepad(id)) {
            if gamepad.is_pressed(Button::RightTrigger) {
                command = commands.next().unwrap();
                write!(stdout, "{}{}", termion::cursor::Goto(3, 3), "                                               ").unwrap();
                write!(stdout, "{}{}", termion::cursor::Goto(3, 3), command).unwrap();
                stdout.flush().unwrap();
            }
            if gamepad.is_pressed(Button::South) {
                let status = Command::new(command)
                                  .status().unwrap();
                stdout.flush().unwrap();
            }
            if gamepad.is_pressed(Button::East) {
                write!(
                    stdout,
                    "{}{}Thanks for using this Launcher.",
                    termion::clear::All,
                    termion::cursor::Goto(1,1),
                ).unwrap();
                exit(0);
            }
        };

        std::thread::sleep(time::Duration::from_millis(300))
    }
    /*
 */
}

