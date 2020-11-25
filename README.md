# ASSBREAK

![demo](https://github.com/L14ms111/img_project/blob/main/demo.gif?raw=true)

A tool that helps brute-force a website.

**ANY FRAUDULENT USE IS NOT THE FAULT OF THE ONE WHO PROGRAMS IT, BUT OF THE ONE WHO USES IT FOR ILLEGAL PURPOSES.**

## How to install

### Prerequisites
You need to install [chromedriver](https://chromedriver.chromium.org) and when you have it installed, you must put it in the environment variables.

How to put chromedriver in the path :

```
Control Panel > System and Security > System > Advanced System Settings > Environment Variable > System Variable > New > Path > /path/chromedriver.exe
```

And to check if it is well installed, run the command prompt (cmd) and write : 

```
chromedriver
```

And it returns this to you :

```
Starting ChromeDriver 87.0.4280.20 (c99e81631faa0b2a448e658c0dbd8311fb04ddbd-refs/branch-heads/4280@{#355}) on port 9515

Only local connections are allowed.
Please see https://chromedriver.chromium.org/security-considerations for suggestions on keeping ChromeDriver safe.
ChromeDriver was started successfully.
```
Instead of having 87.0.4280.20, you must have the current version of chromedriver.

Then you need to install [Selenium Grid](https://www.selenium.dev/downloads/) version 3.141.59 and rename it to selenium.jar, and before you need to have Java but I think you have it ;)

I advise you to put all your files on the chromedriver configuration on the C:/webdrivers folder you created.

And in this folder, you could put selenium.jar and chromedriver.exe

Then you have to get on the file thanks to :

```batch
cd C:/webdrivers
```

And you will have to write in the console :

```
java -jar selenium.jar
```

And if all goes well, normally you will get this message :
```
15:21:14.131 INFO [GridLauncherV3.parse] - Selenium server version: 3.141.59, revision: e82be7d358

15:21:14.284 INFO [GridLauncherV3.lambda$buildLaunchers$3] - Launching a standalone Selenium Server on port 4444

2020-11-23 15:21:14.383:INFO::main: Logging initialized @785ms to org.seleniumhq.jetty9.util.log.StdErrLog

15:21:14.989 INFO [WebDriverServlet.<init>] - Initialising WebDriverServlet

15:21:15.781 INFO [SeleniumServer.boot] - Selenium Server is up and running on port 4444
```

Now you have to leave only selenium open;) and each time you have to run the program, you have to run selenium.

## Install the program

Directly on this [link](https://github.com/L14ms111/assbreak/releases), and after, you must install go to the executable of your operating system (.exe -> windows).

## How to use the program

You will need some basic CSS, and element inspection on your browser, for example for the *"[~] Enter the username selector:"*  field, you will need to put the id, or the class, or the name as you will have done on your CSS code -> Otherwise there will soon be a tutorial in French but you can translate the subtitles to understand ;)

### Example : 

```
[~] The site you want to try brute-force : https://localhost/admins/login
[~] Enter the username selector : #username
[~] Enter the password selector : #password
[~] Enter the button (login) selector : .btn
[~] Enter the username to brute-force : L14ms1
[~] Enter the path of the password dictionary : C:/Users/L14ms1/Desktop/passlist.txt
[~] Do you want to see chrome in the middle of a brute force operation (yes or no) ? no
You will not see chrome however you will be able to see the logs.
```

## How do I get a password dictionary ?

I will put a small password dictionary of about 111kb on the download link BUT you can download a big 14.5gb one on se [link](https://crackstation.net/crackstation-wordlist-password-cracking-dictionary.htm) (torrent).


