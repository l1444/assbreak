// intern
use super::func::*;



pub fn header() {
    let version_app:String = String::from("0.2.1a");
    color_terminal(":::'###:::::'######:::'######::'########::'########::'########::::'###::::'##:::'##:
::'## ##:::'##... ##:'##... ##: ##.... ##: ##.... ##: ##.....::::'## ##::: ##::'##::
:'##:. ##:: ##:::..:: ##:::..:: ##:::: ##: ##:::: ##: ##::::::::'##:. ##:: ##:'##:::
'##:::. ##:. ######::. ######:: ########:: ########:: ######:::'##:::. ##: #####::::
:#########::..... ##::..... ##: ##.... ##: ##.. ##::: ##...:::: #########: ##. ##:::
:##.... ##:'##::: ##:'##::: ##: ##:::: ##: ##::. ##:: ##::::::: ##.... ##: ##:. ##::
:##:::: ##:. ######::. ######:: ########:: ##:::. ##: ########: ##:::: ##: ##::. ##:","green");
    color_terminal("************************************************************************************\n*                                                                           v0.2.1 *", "green");
    color_terminal("*  Author : L14ms1                                                                 *", "green");
    color_terminal("*  Github : https://github.com/L14ms111/assbreak                                   *", "green");
    color_terminal("*  Discord: https://discord.gg/km8jggx43c                                          *\n*                                                               coding for pa$$ion *", "green");
    color_terminal("************************************************************************************", "green");
    #[cfg(windows)]
    if verify_prerequisites() == false {
        println!("{}", "[!] [WARNING] Please put selenium.jar and chromedriver.exe in C:/webdrivers/");
    }

    if verify_update(version_app.clone(), String::from("https://raw.githubusercontent.com/L14ms111/assbreak/master/VERSION")) == true {
        let version: String = download_string(String::from("https://raw.githubusercontent.com/L14ms111/assbreak/master/VERSION"));
        #[cfg(target_os = "windows")]
        println!("[!] [WARNING] A new version is available : {}, the software update is available on this link : https://github.com/L14ms111/assbreak/releases", filter(version.clone()).replace(" ", "").to_string());

        #[cfg(target_os = "macos")]
        color_terminal(&*format!("[!] [WARNING] A new version is available : {}. You must type this 'git pull' command to update the tool.", filter(version.clone()).replace(" ", "")), "yellow");

        #[cfg(target_os = "linux")]
        color_terminal(&*format!("[!] [WARNING] A new version is available : {}, please update.", filter(version.clone()).replace(" ", "")), "yellow");
    }

    color_terminal("[!] [NOTICE] For help, type $help.", "yellow");
}