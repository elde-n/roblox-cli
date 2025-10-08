use std::{fs::File, io::Write, process::Command};

use crate::config::Account;

const SOBER_ROBLOX_CLIENT: &str = "org.vinegarhq.Sober";

fn launch_url(id: u64, job_id: Option<&str>, private_server_code: Option<&str>) -> String {
    let auth_ticket = "";
    let browser_id = 0;

    let game_id = if let Some(job_id) = job_id {
        &format!("&gameId={job_id}")
    } else {
        ""
    };

    let link_code = if let Some(link_code) = private_server_code {
        &format!("&linkCode={link_code}")
    } else {
        ""
    };

    let place_launcher_url: String = url::form_urlencoded::byte_serialize(
        [
            "https://www.roblox.com/Game/PlaceLauncher.ashx?request=RequestGame",
            &format!("&browserTrackerId={browser_id}"),
            &format!("&placeId={id}"),
            "&isPlayTogetherGame=false",
            "&joinAttemptOrigin=PlayButton",
            game_id,
            link_code,
        ]
        .concat()
        .as_bytes(),
    )
    .collect();

    [
        "roblox-player:1",
        "launchmode:play",
        &format!("gameinfo:{auth_ticket}"),
        "launchtime:0",
        &format!("placelauncherurl:{place_launcher_url}"),
        "baseUrl:https://www.roblox.com/",
        "channel:",
        "robloxLocale:en_us",
        "gameLocale:en_us",
        "launchexp:InApp",
    ]
    .join("+")
}

pub(crate) fn run(account: &Account, id: u64, job_id: Option<&str>) {
    #[cfg(target_family = "windows")]
    todo!("Make a pull request, I'm not sure which xdg utils windows has");

    // Assuming we are using sober, we need to manually update the cookie file
    let roblox_client = xdg_utils::query_default_app("x-scheme-handler/https");
    if roblox_client
        .expect("error: failed to find `Sober` roblox client")
        .starts_with(SOBER_ROBLOX_CLIENT)
    {
        const SOBER_COOKIES_PATH: &str = ".var/app/org.vinegarhq.Sober/data/sober/cookies";

        let home = dirs::home_dir().unwrap();
        let mut cookie_file = File::options()
            .write(true)
            .open(home.join(SOBER_COOKIES_PATH))
            .unwrap();

        cookie_file
            .write_all(format!(".ROBLOSECURITY={}", account.cookie).as_bytes())
            .unwrap();
    }

    // TODO: lock file

    #[cfg(target_family = "unix")]
    Command::new("xdg-open")
        .arg(launch_url(id, job_id, None))
        .spawn()
        .expect("error: failed to launch roblox")
        .wait()
        .unwrap();
}
