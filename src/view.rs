<<<<<<< HEAD
=======
use super::func::verify_prerequisites;
>>>>>>> 6d68cc2... add warning
pub fn header() {
    print!("
:::'###:::::'######:::'######::'########::'########::'########::::'###::::'##:::'##:
::'## ##:::'##... ##:'##... ##: ##.... ##: ##.... ##: ##.....::::'## ##::: ##::'##::
:'##:. ##:: ##:::..:: ##:::..:: ##:::: ##: ##:::: ##: ##::::::::'##:. ##:: ##:'##:::
'##:::. ##:. ######::. ######:: ########:: ########:: ######:::'##:::. ##: #####::::
:#########::..... ##::..... ##: ##.... ##: ##.. ##::: ##...:::: #########: ##. ##:::
:##.... ##:'##::: ##:'##::: ##: ##:::: ##: ##::. ##:: ##::::::: ##.... ##: ##:. ##::
:##:::: ##:. ######::. ######:: ########:: ##:::. ##: ########: ##:::: ##: ##::. ##:
..:::::..:::......::::......:::........:::..:::::..::........::..:::::..::..::::..:: \n");
    println!("-------------------------------------------------------------------------------------");
    println!("Coded by L14ms1");
<<<<<<< HEAD
    println!("version : 0.1.5b");
=======
    println!("version : 0.1.6b");
>>>>>>> 6d68cc2... add warning
    println!("Small disclaimer, you must use this project on your sites not on other people's sites, it only protects your site.");
    println!("-------------------------------------------------------------------------------------");
    println!("  ");
    println!("1 - input[type]: text && password && submit        2 - input[type]: email && password && submit");
    println!("3 - input[type]: text && password && <button>      4 - input[type]: email && password && <button>");
    println!("5 - Personnalize (Default)");
    println!("  ");
    println!("-------------------------------------------------------------------------------------");
<<<<<<< HEAD
    println!("  ");
=======
    if verify_prerequisites() == false {
        println!("[!] [WARNING] Please put selenium.jar and chromedriver.exe in C:/webdrivers/");
    }
>>>>>>> 6d68cc2... add warning
}