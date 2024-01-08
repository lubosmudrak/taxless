# Hlavný panel

Hlavný panel privíta používateľa, umožní registráciu, prihlásenie a dá mu prehľad o tom, čo si pre ktorý úrad potrebuje pripraviť.


## B_UO Uvítacia obrazovka

Toto je prvá obrazovka, ktorú neprihlásený používateľ uvidí.

endpoint obrazovky: /


### Požiadavky

Prvky, ktoré obrazovka musí obsahovať

| ID      | Požiadavka                                                                                                            | Mapovanie na UC |
| ------- | --------------------------------------------------------------------------------------------------------------------- | --------------- |
| R_UO_01 | Stručné informácie o aplikácii                                                                                        | UC_B_UO_01      |
| R_UO_02 | Tlačidlo na vstup do sprievodcu žiadosťou o zúčtovanie dane z prijmu                                                  | UC_B_UO_01 UC_B_UO_02 |
| R_UO_03 | Tlačidlo na vstup do prihlasovacieho a registračného formulára                                                        | R_UO_01 UC_B_UO_03 |
| R_UO_04 | Legálne vyžadované informácie o používaní cookies a GDPR formulár. Fomulár musí obsahovať informácie o tom, na čo všetko používame cookies. Formulár musí informovať používateľa, že cookies používame iba na zabezpečenie správnej funkčnosti stránky. Formulár musí mať tlačidlo na jeho zatvorenie | UC_B_UO_04 |
| R_UO_05 | Tlačidlo pre zobrazenie licencií na použité knižnice                                                                  | UC_B_UO_01 UC_B_UO_06 |


### Use cases

| UC_B_UO_01        | Zobrazenie úvodnej obrazovky                                                                                                  |
| ----------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| Popis             | Zobrazenie prvkov R_UO_01, R_UO_02, R_UO_03 a R_UO_05  na úvodnej obrazovke.                                                  |
| Vstupné podmienky |                                                                                                                               |
| Výnimky           |                                                                                                                               |

| UC_B_UO_02        | Vstup do sprievodcu žiadosťou o zúčtovanie dane z prijmu                                                                      |
| ----------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| Popis             | Po kliknutí na tlačidlo pre vstup do sprievodcu sa zobrazí prvé dialógové okno sprievodcu.                                    |
| Vstupné podmienky |                                                                                                                               |
| Výnimky           |                                                                                                                               |

| UC_B_UO_03        | Prekliknutie sa do registračného a prihlasovacieho formulára                                                                  |
| ----------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| Popis             | Po kliknutí na toto tlačidlo sa zobrazí registračná a prihlasovacia obrazovka.                                                |
| Vstupné podmienky |                                                                                                                               |
| Výnimky           |                                                                                                                               |

| UC_B_UO_04        | Zobrazenie informačného formulára o používaní cookies                                                                         |
| ----------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| Popis             | Zobrazenie informačného panela o tom, že stránka využíva cookies. Formulár sa dodatočne zobrazí, ak je hodnota cookie cookie_consent rovná false.  Po zatvorení informačného formulára o použivaní cookies musí toto okno zmiznúť a uloží sa, že sme súhlasili s použitím cookies. |
| Vstupné podmienky | Zapnutý Javascript v prehliadači.                                                                                             |
| Výnimky           | NO_JAVASCRIPT - formulár sa nezobrazí.                                                                                        |

| UC_B_UO_05        | Automatické presmerovanie prihláseného používateľa na jeho Používateľsky panel                                                |
| ------------------| ----------------------------------------------------------------------------------------------------------------------------- |
| Popis             | Ak prehliadač pri načítaní úvodnej obrazovky úspešne odošle authToken, ktorý je priradený k použivateľovi aplikácie, používateľ je presmerovaný na obrazovku /pouzivatelsky_panel. |
| Vstupné podmienky | autentifikačný cookie uložený v cache prehliadača.                                                                            |
| Výnimky           | NO_COOKIE - normálne sa zobrazí úvodná obrazovka.                                                                             |

| UC_B_UO_06        | Prekliknutie na obrazovku s licenčnými informáciami                                                                           |
| ------------------| ----------------------------------------------------------------------------------------------------------------------------- |
| Popis             | Po kliknutí na tlačidlo sa zobrazí obrazovka s licenčnými informáciami.                                                       |
| Vstupné podmienky |                                                                                                                               |
| Výnimky           |                                                                                                                               |


## B_PO Prihlasovacia obrazovka

Prihlasovacia obrazovka slúži na registráciu používateľa a / alebo prihlásenie cez interný systém či externé autorizačné služby.

Endpoint obrazovky: /prihlasenie

### Požiadavky

| ID      | Požiadavka                                                                                                            | Mapovanie na UC |
| ------- | --------------------------------------------------------------------------------------------------------------------- | --------------- |
| R_PO_01 | Formulár na registráciu. Tento formulár obsahuje vstupné polia na používateľsky email, prihlasovacie meno, heslo a tlačidlo na vykonanie registrácie. Pri vstupných poliach sú textové popisy. | UC_B_PO_01 UC_R_PO_02 |
| R_PO_02 | Formulár na prihlásenie cez interný databázový systém. Formulár obsahuje vstupné polia pre používateľske meno, heslo, tlačidlo pre vykonanie prihlásenia a tlačidlo na obnovenie hesla. Pri vstupných poliach sú textové popisy. | UC_B_PO_01 UC_B_PO_04 UC_B_PO_08 |
| R_PO_03 | Formulár s tlačidlami pre prihlásenie sa cez externé autentifikačné služby, napríklad cez Google alebo Facebook. Jedna sluzba = jedno tlačidlo. | UC_B_PO_01 UC_B_PO_05 UC_B_PO_06 UC_B_PO_07 |
| R_PO_04 | tlačidlo na návrat na úvodnú obrazovku.                                                                               | UC_B_PO_09      |


### Use cases

| UC_B_PO_01        | Zobrazenie prihlasovacej obrazovky                                                                                            |
| ----------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| Popis             | Zobrazenie prvkov R_PO_01, R_PO_02, R_PO_03 a R_PO04 na obrazovke.                                                            |
| Vstupné podmienky |                                                                                                                               |
| Výnimky           |                                                                                                                               |

| UC_R_PO_02        | Registrácia používateľa do interného databázového systému
| ----------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| Popis             | Stlačením tlačidla pre registráciu sa odošle email, prihlasovacie meno a heslo na server. Následne sa v databáze vytvorí záznam o novom používateľovi s vyššie spomenutými údajmi. Heslo používateľa je zahashované algoritmom SHA-512 a "osolené". Na emailovú adresu sa odošle email s linkom pre aktiváciu. | 
| Vstupné podmienky | Heslo musí používateľ zadať do dvoch rozdielnych vstupných polí, aby sa predišlo preklepom v hesle. Heslo musí mať aspoň 7 znakov. |
| Výnimky           | EMAIL_USED, EMAIL_NOT_FOUND, PASSWORDS_DONT_MATCH, PASSWORD_MATCHES_LOGIN, PASSWORD_IS_SHORT                                  |

| UC_R_PO_03        | Aktivácia účtu
| ----------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| Popis             | Hash obsiahnutý v linku aktivačného e-mailu sa porovná s aktivačným hashom účtu uloženým v DB. Ak sú hashe zhodné, je možné sa s týmto účtom prihlásiť.
| Vstupné podmienky | Používateľ vykonal registráciu cez krok UC_R_PO_02
| Výnimky           | 

| UC_B_PO_04        | Prihlásenie sa do aplikácie cez e-mail                                                                                        |
| ----------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| Popis             | Po vyplnení mena a hesla a kliknutí na prihlásenie sa cez mail sa odošle do DB požiadavka na overenie hashu hesla a zaslanie autorizačného cookie. Ak tento cookie bude nájdený v tabuľke ACTIVE_USERS, používateľovi bude umožnené načítavať personalizované stránky s jeho osobnými údajmi. Doba resetovania auth cookie je 7 dní. Pri zadaní zlých údajov kvoli bezpečnosti neprezrádzame, či je zlé meno, heslo alebo oboje |
| Vstupné podmienky | Používateľ vykonal aktiváciu svojho účtu cez krok UC_B_PO_03                                                                  |
| Výnimky           | WRONG_CREDENTIALS, ACCOUNT_NOT_ACTIVE                                                                                         |

| UC_B_PO_05        | Prihlásenie sa cez Google                                                                                                     |
| ----------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| Popis             | Po prihlásení sa cez Google formulár sa uloží autentifikačný token vygenerovaný službou Google OpenAuth. Implementovať nasledovne: https://developers.google.com/identity/openid-connect/openid-connect . Ak google účet nie je namapovaný na žiadne ID používateľa, aplikácia sa používateľa spýta na používateľske meno a následne uloží meno a email naviazaný na Google účet |
| Vstupné podmienky | Existujúci účet Google                                                                                                        |
| Výnimky           | WRONG_CREDENTIALS                                                                                                             |

| UC_B_PO_06        | Prihlásenie sa cez účet Légie                                                                                                 |
| ----------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| Popis             | Po prihlásení sa cez prihlasovací formulár Légie je prihlasovací request zasladný na našu hlavnú databázu, kde je spracvoaný a do DB taxlessu a prehliadača používateľa sa odošle autentifikačný cookie, cez ktorý sa následne dajú prehliadať personalizované stránky. |
| Vstupné podmienky | Vytvorený účet na stránke Légia Slobody.                                                                                      |
| Výnimky           | WRONG_CREDENTIALS                                                                                                             |

| UC_B_PO_07        | Prihlásenie sa cez Facebook                                                                                                   |
| ----------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| Popis             | Po prihlásení sa cez Facebook formulár sa do databázy sa uloží autentifikačný token vygenerovaný Facebookom. Implementovať nasledovne: https://developers.facebook.com/docs/facebook-login/web/login-button/ . Ak Facebook účet nie je namapovaný na žiadne ID používateľa, aplikácia sa používateľa spýta na používateľske meno a následne uloží meno a email naviazaný na Facebook účet. |
| Vstupné podmienky | Existujúci účet Facebook.                                                                                                     |
| Výnimky           | WRONG_CREDENTIALS                                                                                                             |

| UC_B_PO_08        | Obnovenie hesla                                                                                                               |
| ----------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| Popis             | kliknutie na textový link "Zabudol som heslo" sa vykoná presmerovanie na obrazovku pre obnovenie hesla.                       |
| Vstupné podmienky |                                                                                                                               |
| Výnimky           | WRONG_EMAIL                                                                                                                   |

| UC_B_PO_09        | Návrat na úvodnú obrazovku                                                                                                    |
| ----------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| Popis             | Vykoná návraz na úvodnú obrazovku: /                                                                                          |
| Vstupné podmienky |                                                                                                                               |
| Výnimky           |                                                                                                                               |


## B_ZRH Obrazovka žiadosti o reset hesla

Tu vykonáme odoslanie žiadosti o reset hesla.

endpoint obrazovky: /reset_hesla_ziadost

### Požiadavky

Prvky, ktoré obrazovka musí obsahovať

| ID       | Požiadavka                                                                                                           | Mapovanie na UC |
| -------- | -------------------------------------------------------------------------------------------------------------------- | --------------- |
| R_ZRH_01 | Formulár na odoslanie žiadosti o reset hesla. Obsahuje vstupné pole pre email, jeho popis a tlačidlo na odoslanie žiadosti. | UC_B_ZRH_01 UC_B_ZRH_02 |

### Use cases

| UC_B_ZRH_01       | Zobrazenie formulára na žiadosť o reset hesla                                                                                 |
| ----------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| Popis             | Zobrazenie R_ZRH_01 na obrazovke.                                                                                             |
| Vstupné podmienky |                                                                                                                               |
| Výnimky           |                                                                                                                               |

| UC_B_ZRH_02       | Odoslanie žiadosti o nové heslo                                                                                               |
| ----------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| Popis             | Po kliknutí na tlačidlo žiadosti o reset hesla na e-mail sa v DB v tabuľke s používateľmi pridá údaj RESET_HASH, na mail sa odošle výzva na resetovanie hesla v ktorej bude link na stránku aplikácie, ktorý bude obsahovať tento hash. Ak je hash na reset hesla nájdený v DB, zobrazí sa obrazovka na resetovanie hesla, na ktorú sa v requeste odošle resetovací hash. |
| Vstupné podmienky | E-mailová adresa musí byť evidovaná v DB.                                                                                     |
| Výnimky           | NO_EMAIL - email nie je v našej db: vrátime používateľa na obrazovku žiadosti o reset hesla, kde uvidí chybové hlásenie.      |


## B_RH Obrazovka reset hesla

Na tejto obrazovke sa vykoná obnovenie hesla používateľa

endpoint obrazovky: /reset_hesla

### Požiadavky

| ID       | Požiadavka                                                                                                           | Mapovanie na UC |
| -------- | -------------------------------------------------------------------------------------------------------------------- | --------------- |
| R_RH_01  | Formulár na obnovenie hesla. Obsahuje informáciu o mene používateľa dve vstupné polia na zadanie hesla a tlačidlo na vykonanie resetu hesla | UC_B_RH_01 |


## Use cases
| UC_B_RH_01        | Obnovenie hesla                                                                                                               |
| ----------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| Popis             | Po stlačení tlačidla na reset hesla sa uloží nové heslo do DB a používateľ je presmerovaný na prihlasovaciu obrazovku.        |
| Vstupné podmienky | Heslá v oboch vstupných poliach pre nové heslo sa musia zhodovať.                                                             |
| Výnimky           | PASSWORDS_DONT_MATCH, PASSWORD_MATCHES_LOGIN, PASSWORD_IS_SHORT                                                               |

## B_N Navigačný panel

Navigačný panel je viditeľný na každej obrazovke

endpoint: nie je


### Požiadavky

| ID      | Požiadavka                                                                                                            | Mapovanie na UC |
| ------- | --------------------------------------------------------------------------------------------------------------------- | --------------- |
| R_NP_01 | Vloženie navigačného panelu, ako súčasti každej obrazovky v aplikácii.                                                |                 |
| R_NP_02 | Tlačidlo na prepínanie farebných tém aplikácie.                                                                       |                 |
| R_NP_03 | Zobrazenie informácii o prihlásenom používateľovi, ak je používateľ prihlásený. Ak je používateľ odhlásený, má meno Anonym. |           |
| R_NP_04 | Text s copyrightom vo footeri stránky: © 2024 Ľuboš Mudrák, Pal Szidoran, Niekto Nejaký, Flying Pigeon. |                |
| R_NP_05 | Rozcestník - zoznam linkov, cez ktoré sa dá rýchlo prekliknúť na domovskú obrazovku(/), obrazovku prihlásenia(/prihlasenie) ak je používatel odhlásený, alebo používateľsky panel(/pouzivatelsky_panel) ak je prihlásený, obrazovku wizarda na vyplnenie žiadosti o zúčtovanie(/zuctovanie_ziadost), wizarda na vyplnenie formulára pre darovanie 2% dane z prijmu (/dve_percenta) |  |


### Use cases

| UC_B_NP_01        | Zobrazenie navigačného panelu na obrazovke                                                                                    |
| ----------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| Popis             | Načítanie navigačného panelu, ktorý sa dá ako template vložiť do každej obrazovky. Obsahuje prvky R_NP_01, R_NP_02 R_NP_03 R_NP_04 R_NP_05. |
| Vstupné podmienky |                                                                                                                               |
| Výnimky           |                                                                                                                               |


| UC_B_NP_02        | Prepínanie farebných tém aplikácie                                                                                            |
| ----------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| Popis             | Po výbere témy z dropdownu sa zmení CSS štýl použitý na stránke.                                                              |
| Vstupné podmienky |                                                                                                                               |
| Výnimky           |                                                                                                                               |


| UC_B_NP_03        | Preklikávanie sa na obrazovky cez rozcestník                                                                                  |
| ----------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| Popis             | každé tlačidlo v R_NP_05 musí po kliknutí otvoriť príslušný endpoint v jeho popise.                                           |
| Vstupné podmienky |                                                                                                                               |
| Výnimky           |                                                                                                                               |


