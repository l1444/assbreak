//local
use func::*;
use view::header;

//std
use std::io;
use std::io::Write;
use std::process::exit;
use colored::Colorize;

// mod
mod brute_force;
mod config;
mod form;
mod func;
mod view;



///
/// Coded by L14ms1 <l14ms1@outlook.fr>
/// last update 22:54 - 08/12/2020
/// LICENSE MIT
///

static mut HEADER:bool = false;

pub fn main() {
    unsafe {
        if HEADER == false {
            header();
            rpc();
            HEADER = true;
        }
    }
    print!("{}", "[~] What configuration do you want to use ? : ".green());
    let mut commands = String::new();
    let _ = io::stdout().flush();
    // dispatcher
    match io::stdin().read_line(&mut commands) {
        Ok(_) => {
            match String::as_str(&filter(commands)) {
                "1" => {
                    form::input_text_password_submit();
                }
                "2" => {
                    form::input_email_password_submit();
                }
                "3" => {
                    form::input_text_password_button();
                }
                "4" => {
                    form::input_email_password_button();
                }
                "5" => {
                    form::input_button();
                }
                "6" => {
                    form::input_email_button();
                }
                "7" => {
                    form::default();
                }
                "8" => {
                    form::brute_email_outlook();
                }
                "9" => {
                    form::brute_email_icloud();
                }
                "10" => {
                    form::brute_email_yahoo();
                }
                "11" => {
                    form::brute_email();
                }
                "$help" => {
                    println!("\n");
                    println!("Brute force of a website from a GET or POST link [SOON]");
                    println!("---------------------------------------------------------------------------------------------------------");
                    println!(" Brute-force a password from a website form : [EXPERIMENTAL]");
                    println!("    1 - input[type]: text && password && submit        2 - input[type]: email && password && submit");
                    println!("    3 - input[type]: text && password && <button>      4 - input[type]: email && password && <button>");
                    println!("    5 - input[type]: text && password && button*       6 - input[type]: email && password && button*");
                    println!("    7 - Personnalize                                   *personnalize");
                    println!("---------------------------------------------------------------------------------------------------------");
                    println!(" Brute-force the password of a mailbox : [BETA]");
                    println!("    8 - An Outlook & Hotmail email address             9 - An iCloud email address");
                    println!("   10 - An Yahoo email address:                       11 - Personnalize (imap address)");
                    println!(" Doesn't work for gmail because OAUTH2 ;)");
                    println!("---------------------------------------------------------------------------------------------------------");
                    println!("  $quit - Quit assbreak                               $help - To get help");
                    println!("---------------------------------------------------------------------------------------------------------");
                    println!("Example: you want to brute-force the site and the selectors correspond to one which is proposed (here we are an <input type = 'text'>, <input type = 'password'> and a <button>, I will type 3 in '[~] What configuration do you want to use?:'.");
                    println!("---------------------------------------------------------------------------------------------------------");
                    println!("\n");
                }
                "$quit" => {
                    exit(0);
                }
                "$query" => {
                    form::query();
                }
                "$papounet" => {
                    println!("\n");
                    println!("J'ai sodomisé mon père. Il m'avait fait venir dans le salon (y avait personne) et m'a engueule car \
je squatte la maison et je foutais rien de ma vie. Soudainement, je sais pas ce qui m'a pris \
(surement au fait que je me suis pas astiqué depuis 3 jours), je lui ai pincé son gros cul flasque \
et lui ai dis que c'est moi le nouveau papa de cette maison. Il m'a fixé des yeux puis s'est abaissé au \
niveau de mon pantalon et l'a fait détacher. Il a descendu ensuite mon caleçon, sous mes yeux choqués mais \
pas non plus dégoutés. Il m'a fait une pipe du turf, la meilleure que j'ai eu. Il s'est ensuite mis contre le \
canapé et je luis ai arraché son gros futal d'obèse. Je l'ai sodomise comme jamais, même quand j'espionnais mes \
parents en train de niquer je n'ai jamais autant jouit, c'était magnifique franchement. En plus d'habitude je lui\
lechais juste l'anus après son boulot pour le faire décompresser après sa dure journée de travail, je pensais \
pas que ça allait aussi loin! J'ai ensuite retiré ma bite (couverte de nutella, miam) et du gaz très bruillant \
en est sorti. Il avait le cul rempli ce vieux cochon!!!! \
J'ai ensuite tout lâché sur sa grosse barbe et on s'est rhabillé.\
Il vient de m'envoyer un message, il veut remettre ça a demain sinon il me vire de sa maison, \
je suis devenu son esclave sexuel, mais j'aime ça. Faudrait que je demande a maman d'y participer aussi, \
ça serait cool, même si ça va être galère de la déterrer.");
                    println!("\n");
                }
                _ => {}
            }
        }
        Err(e) => {
            println!("An error has occurred : {}", e);
        }
    }
    restart();
}



