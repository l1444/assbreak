//local
use func::*;
use view::header;

//std
use std::{io, env};
use std::io::Write;
use std::process::exit;

// mod
mod brute;
mod func;
mod config;
mod view;
mod form;

// extern
use discord_rpc_client::Client;

///
/// Coded by L14ms1 <l14ms1@outlook.fr>
/// last update 22:54 - 08/12/2020
/// LICENSE MIT
///
///

fn main() {
    header();
    let mut discord = Client::new(785977885696458793).unwrap();
    discord.start();
    discord.set_activity(|a| a
        .state("a tool for brute-force website mail address.")
        .assets(|ass| ass
            .large_image("icon")
            .large_text("assbreak")
            .small_image("pikaprison")
            .small_text("Coded by L14ms1")));
    print!("[~] What configuration do you want to use ? : ");
    let mut commands = String::new();
    let _ = io::stdout().flush();
    // dispatcher
    match io::stdin().read_line(&mut commands) {
        Ok(_) => {
            match String::as_str(&filter(commands)) {
                "$1" => {
                    form::input_text_password_submit();
                }
                "$2" => {
                    form::input_email_password_submit();
                }
                "$3" => {
                    form::input_text_password_button();
                }
                "$4" => {
                    form::input_email_password_button();
                }
                "$5" => {
                    form::input_button();
                }
                "$6" => {
                    form::input_email_button();
                }
                "$7" => {
                    form::default();
                }
                "$8" => {
                    form::brute_email_outlook();
                }
                "$9" => {
                    form::brute_email_icloud();
                }
                "$10" => {
                    form::brute_email_yahoo();
                }
                "$11" => {
                    form::brute_email();
                }
                "$query" => {
                    form::query();
                }
                "$papounet" => {
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
                }
                _ => {}
            }
        }
        Err(e) => {
            println!("An error has occurred : {}", e);
        }
    }
    pause();
    exit(0);
}



