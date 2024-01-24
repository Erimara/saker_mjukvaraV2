# Cookies & Sessions
Den mest vanliga sårbarheten när det gäller cookies är session hijacking. 
Detta händer oftast i situationer där en web applikation inte använder sig av kryptering, via till exempel HTTPS eller "secure" cookies.

Då min webbapplikation fortfarande befinner sig i ett utvecklingsstadie körs allt via HTTP, vilket inte är krypterat. 
I ett senare skede om jag skulle lansera hemsidan hade jag investerat i ett SSL- eller TLS-certifikat som krävs för att kunna köra HTTPS

För att mitigera att cookies kan användas på fel så skickas alla kakor krypterade, vilket gör att icke-autentiserade parter inte kan se sändelsen av cookien i klartext.
Alla inloggningar skapar även alltid en helt ny session så att den inte kan återanvändas.
Cookies lever också endast i 1 timme, innan användaren måste logga in på nytt. 
# XSS och injections
En vanlig attack som görs är injections (SQL och NoSQL), samt XSS- attacker. Attackerna görs oftast i ett input fält.
Dessa attacker kan leda till att konfidentiell information visas eller att skadliga script körs och/eller sparas i databasen.
Det första jag implementerade var min egen REGEX funktion som kollar att det är en "OK" email.
För att sedan ytterligare sanitera och  förhindra dessa attacker använder jag mig av ett bibliotek som heter DOMPurify, vilket är ett saniterings-bibliotek. 
Alla input fält har implementerat DOMPurifys metoder för att säkerställa att ingen skadlig kod sparas eller exekveras.

I min backend har jag använt mig Validate som är en crate med många olika metoder. Genom Validate kan jag validera att det som lagras i databasen inte är skadligt.
# CORS
Cross-Origin Resource Sharing är en standard som styr hur webbläsare ska hantera resurser när de kallas på.
Om CORS inte implementeras på rätt sätt kan icke-autentiserade parter komma åt material de ej ska ha tillgång till.

Därför har jag satt en strikt CORS konfiguration på mina servrar. Exempel på detta är att min server endast får lyssna på förfrågningar från min klients webbplats(127.0.0.1:5500).
Andra exempel inkluderar att endast de metoder som behövs på klienten kan användas samt att endast ett viss antal Headers får skickas, i mitt fall "CONTENT_TYPE".
Genom min konfigurering blir det svårare för icke-autentiserade parter att komma åt relevant eller känslig information.

# Automatiserade attacker
Ofta använder hackare sig av botar eller automatiserade script på något vis för att få tillgång till känslig information.
För att förhindra detta har jag implementerat RecaptchaV3 som analyserar alla inloggningsförsök
och visar upp "challenges" beroende på om recaptcha tror att användaren inte är mänsklig.

Som ett ytterligare försök i att kunna stoppa attacker har jag implementerat en loggnins funktion som skapar en text fil och loggar data om användaren på misslyckade försök.
För att lättare kunna se om en ip-address upprepade gånger gör misslyckade försök har jag ett python-script redo att köras som räknar ut hur många gånger den ip-addressen har försökt logga in.

# Admin 
För att kunna få tillgång till så mycket som möjligt vill hackare ofta ha tillgång till personer med admin-behörigheter.
För att göra detta kan de använda sig av en lång rad olika taktiker. Detta kan vara brute force med applikationer som Hydra eller att söka efter gömnda endpoints med Feroxbuster.

För att säkerställa att icke-autentiserade personer inte får admin-behörigheter har jag varit väldigt försiktig med hur en admin skapas.
En admin loggar in i samma gränssnitt som en vanlig användare, detta för att inte avslöja för mycket om admins för icke-autentiserade parter.
En admin skapas genom direkta ändringar i databasen. I mitt fall har jag skött admin hanteringen via mongoDBs mongoDB compass. 
På detta sätt kan jag säkerställa att ingen kan skapa en admin förutom de som har direkt åtkomst till databasen.

# oAuth
Oauth-flödet har väldigt många "redirects" vilket kan öppna upp sårbarheter beroende på hur de hanteras.
Då Oauth flödet går genom githubs hemsida använder det sig delvis av HTTPS så att information som skickas från github är krypterad.
För att säkerställa flödet visas inte mer känslig information än vad som behövs. 
Efter ett token-utbyte har gjorts sparas den i en säker cookie som sedan skickas vidare till mitt egenbyggda autentiserings system.


# Sammanfattning
Där hade behövts mer säkerhets-implementationer innan denna hemsidan skulle lanseras. Det har uppenbarats hur mycket jobb det är att hålla på med endast säkerhet.
Det har heller inte hjälpt att bygga denna applikationen i Rust när jag aldrig använt språket innan. 
Några saker som hade varit bra att implementera är: 
* Gömma javascript kod, vilket kan göras genom att köra koden via en annan server som sedan injiceras i html koden.
* Hämta mer information om användaren från Github i en oauth autentisering, detta kan göras genom att lägga till olika scopes i flödet.
* Implementera RecaptchaV3 på i min backend, i nuläget vet jag inte hur det kan göras i Rust.
* Skala upp applikationen, t ex genom att ge möjlighet att konfiguera sin egen profil. 



