// intern
use super::func::*;

pub fn header() {
    let version_app:String = String::from("0.1.15b");
    color_terminal(":::'###:::::'######:::'######::'########::'########::'########::::'###::::'##:::'##:
::'## ##:::'##... ##:'##... ##: ##.... ##: ##.... ##: ##.....::::'## ##::: ##::'##::
:'##:. ##:: ##:::..:: ##:::..:: ##:::: ##: ##:::: ##: ##::::::::'##:. ##:: ##:'##:::
'##:::. ##:. ######::. ######:: ########:: ########:: ######:::'##:::. ##: #####::::
:#########::..... ##::..... ##: ##.... ##: ##.. ##::: ##...:::: #########: ##. ##:::
:##.... ##:'##::: ##:'##::: ##: ##:::: ##: ##::. ##:: ##::::::: ##.... ##: ##:. ##::
:##:::: ##:. ######::. ######:: ########:: ##:::. ##: ########: ##:::: ##: ##::. ##:
..:::::..:::......::::......:::........:::..:::::..::........::..:::::..::..::::..::","green");
    color_terminal("---------------------------------------------------------------------------------------------------------", "green");
    color_terminal("  Coded by L14ms1 for pa$$ion", "green");
    color_terminal(&*format!("  version : {}", version_app), "green");
    color_terminal("  For educational purposes only !", "green");
    color_terminal("---------------------------------------------------------------------------------------------------------", "green");
    #[cfg(windows)]
    if verify_prerequisites() == false {
        println!("{}", "[!] [WARNING] Please put selenium.jar and chromedriver.exe in C:/webdrivers/");
    }

    if verify_update(version_app.clone(), String::from("https://raw.githubusercontent.com/L14ms111/assbreak/main/VERSION")) == true {
        let version: String = download_string(String::from("https://raw.githubusercontent.com/L14ms111/assbreak/main/VERSION"));
        #[cfg(target_os = "windows")]
        println!("[!] [WARNING] A new version is available : {}, the software update is available on this link : https://github.com/L14ms111/assbreak/releases", filter(version.clone()).replace(" ", "").to_string());

        #[cfg(target_os = "macos")]
        color_terminal(&*format!("[!] [WARNING] A new version is available : {}. You must type this 'git pull' command to update the tool.", filter(version.clone()).replace(" ", "")), "yellow");

        #[cfg(target_os = "linux")]
        color_terminal(&*format!("[!] [WARNING] A new version is available : {}, please update.", filter(version.clone()).replace(" ", "")), "yellow");
    }

    color_terminal("[!] [NOTICE] For help, type $help.", "yellow");
}