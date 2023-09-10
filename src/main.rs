use anyhow::Result;
use chrono::{Local, TimeZone};
use clap::Parser;
use libc::utmpx;
use libc::{endutxent, getloadavg, getutxent, setutxent, USER_PROCESS};
use std::io::Write;
use std::process::exit;

#[derive(Parser)]
#[command(
    author,
    version,
    about="Tell how long the system has been running.",
    long_about = None
)]
struct Cli {
    /// show uptime in pretty format
    #[arg(short('p'), long)]
    pretty: bool,

    /// system up since
    #[arg(short('s'), long)]
    since: bool,
}

fn main() -> Result<()> {
    let args: Cli = Cli::parse();

    // setutxent(): utmpx ファイルを開き、最初のエントリにリセット
    // getutxent(): utmpx ファイルから次のエントリを取得
    // endutxent(): utmpx ファイルを閉じる
    unsafe {
        // utmpx ファイルを開き、開始時刻を取得する
        setutxent();
        let entry = getutxent();
        // エントリが取得できなかった場合はエラー
        if entry.is_null() {
            writeln!(std::io::stderr(), "{}", "entry is null")?;
            endutxent();
            exit(1);
        }
        let utmpx_entry: &utmpx = &*entry;
        let boot_time = utmpx_entry.ut_tv.tv_sec;
        // 取得できなかった場合はエラー
        if boot_time == 0 {
            writeln!(std::io::stderr(), "{}", "couldn't get boot time")?;
            endutxent();
            exit(1);
        }
        // unixtimeをdatetimeに変換
        let dt = Local.timestamp_opt(boot_time as i64, 0);
        match dt.single() {
            Some(date_time) => {
                let uptime = Local::now().timestamp() - boot_time as i64;

                // 起動時刻を表示
                if args.since {
                    let since_datetime = date_time.format("%Y-%m-%d %H:%M:%S").to_string();
                    writeln!(std::io::stdout(), "{}", since_datetime)?;
                // 起動してからの経過時間を表示
                } else if args.pretty {
                    let pretty_uptime =
                        format!("up {} hours, {} minutes", uptime / 3600, uptime / 60 % 60);
                    writeln!(std::io::stdout(), "{}", pretty_uptime)?;
                } else {
                    let currenttime = Local::now().format("%H:%M:%S").to_string();
                    let hm_uptime = format!("{}:{}", uptime / 3600, uptime / 60 % 60);
                    // ユーザー数を取得
                    let mut user_count: i32 = 0;
                    loop {
                        let entry_ptr = getutxent();
                        if entry_ptr.is_null() {
                            break;
                        }
                        let entry2: &utmpx = &*entry_ptr;
                        if entry2.ut_type == USER_PROCESS {
                            user_count = user_count + 1;
                        }
                    }
                    let mut user_count_str = format!("{} user", user_count);
                    if user_count > 1 {
                        user_count_str = format!("{} users", user_count);
                    }
                    // ロードアベレージを取得
                    let mut loadavg: [f64; 3] = [0.0; 3];
                    getloadavg(loadavg.as_mut_ptr(), 3);
                    let loadavg_str = format!(
                        "load average: {:.2}, {:.2}, {:.2}",
                        loadavg[0], loadavg[1], loadavg[2]
                    );
                    writeln!(
                        std::io::stdout(),
                        " {} up  {},  {}, {}",
                        currenttime,
                        hm_uptime,
                        user_count_str,
                        loadavg_str
                    )?;
                }
            }
            None => {
                writeln!(std::io::stderr(), "{}", "Invalid timestamp")?;
            }
        }

        // utmpx ファイルを閉じる
        endutxent();
    }

    Ok(())
}
