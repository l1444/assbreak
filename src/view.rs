// intern
use super::func::*;

// extern
use colored::*;

pub fn header() {
    let version_app:String = String::from("0.1.14b").bright_green().to_string();
    print!("\n{}\n", ":::'###:::::'######:::'######::'########::'########::'########::::'###::::'##:::'##:
::'## ##:::'##... ##:'##... ##: ##.... ##: ##.... ##: ##.....::::'## ##::: ##::'##::
:'##:. ##:: ##:::..:: ##:::..:: ##:::: ##: ##:::: ##: ##::::::::'##:. ##:: ##:'##:::
'##:::. ##:. ######::. ######:: ########:: ########:: ######:::'##:::. ##: #####::::
:#########::..... ##::..... ##: ##.... ##: ##.. ##::: ##...:::: #########: ##. ##:::
:##.... ##:'##::: ##:'##::: ##: ##:::: ##: ##::. ##:: ##::::::: ##.... ##: ##:. ##::
:##:::: ##:. ######::. ######:: ########:: ##:::. ##: ########: ##:::: ##: ##::. ##:
..:::::..:::......::::......:::........:::..:::::..::........::..:::::..::..::::..::".bright_green());
    println!("{}", "---------------------------------------------------------------------------------------------------------".bright_green());
    println!("  {}", "Coded by L14ms1 for pa$$ion".bright_green());
    println!("  {} {}", "version :".bright_green(), version_app);
    println!("  {}", "For educational purposes only !".bright_green());
    println!("{}", "---------------------------------------------------------------------------------------------------------".bright_green());
    #[cfg(windows)]
    if verify_prerequisites() == false {
        println!("{}", "[!] [WARNING] Please put selenium.jar and chromedriver.exe in C:/webdrivers/");
    }

    if verify_update(version_app.clone(), String::from("https://raw.githubusercontent.com/L14ms111/assbreak/main/VERSION")) == true {
        let version: String = download_string(String::from("https://raw.githubusercontent.com/L14ms111/assbreak/main/VERSION"));
        #[cfg(windows)]
        println!("{}{}{}", "[!] [WARNING] A new version is available : ".bright_yellow() ,filter(version).replace(" ", "").bright_yellow().to_string(), ", the software update is available on this link : https://github.com/L14ms111/assbreak/releases".bright_yellow());
        #[cfg(target_os = "macos")]
        println!("{} {}{}", "[!] [WARNING] A new version is available :".bright_yellow() ,filter(version.clone()).replace(" ", "").bright_yellow().to_string(), ", You must type this 'git pull' command to update the tool.".bright_yellow());
        #[cfg(target_os = "linux")]
        println!("{} {}{}", "[!] [WARNING] A new version is available :".bright_yellow() ,filter(version.clone()).replace(" ", "").bright_yellow().to_string(), ", You must type this 'git pull' command to update the tool.".bright_yellow());
    }

}