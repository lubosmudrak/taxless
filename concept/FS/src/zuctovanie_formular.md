# Formulár pre žiadosť o zúčtovanie dane z prijmu

Táto časť aplikácie zrozumiteľným spôsobom prevedie používateľa vyplnením formulára pre žiadosť o zúčtovanie dane z prijmu. Každý krok musí sprevádzať jednoduchý popis konkrétnej časti formulára s vysvetlením, prípadne môže byť sprevádzaný uvedením konkrétnych príkladov.


## ZF_OU Osobné údaje

Tu je používateľ oboznámený s tým, čo bude robiť a vyplní o sebe základné osobné údaje

endpoint obrazovky: /zuctovanie_ziadost/osobne_udaje


### Požiadavky

Prvky, ktoré obrazovka musí obsahovať. Stránka obsahuje vstupné polia pre údaje, ktoré musia byť vyplnené. Ak nie sú vyplnené, po neúspešnom pokuse o odoslanie takto vyplneného formulára sa stránka načíta znova a pri vstupnom poli sa objaví textové upozornenie typu "*Toto pole je povinné".

V dokumentácii sú taktiež povinné polia vyznačené hviezdičkou

| ID         | Požiadavka                                                                                                            | Mapovanie na UC |
| ---------- | --------------------------------------------------------------------------------------------------------------------- | --------------- |
| R_ZF_OU_01 | Uvítací text, ktorý oboznámi používateľa s procesom podania žiadosti o vykonanie zúčtovania dane z prijmu a vysvetlí mu, že pri položkách, ktorým nerozumie má kliknúť na tlačidlo, na ktoré má kliknúť, ak potrebuje asistenciu | UC_ZF_OU_01 |
| R_ZF_OU_02 | Formulár na súhlas so spracovaním osobných údajov na účely vytvorenia žiadosti o ročné zúčtovanie, tlačiva na poukázanie 2% z prijmu, asistenciu so získavaním podkladov pre žiadosť o zúčtovanie a uložením používateľskych údajov na účely predvyplnenia formulárov v budúcnosti." Formulár pozostáva z textu a checkboxu* | UC_ZF_OU_01 |
| R_ZF_OU_03 | Formulár na vyplnenie osobných údajov. Obsahuje vstupné polia s popismi pre krstné meno*, priezvisko*, titul za menom, titul pred menom, a rodné číslo* | UC_ZF_OU_01 |
| R_ZF_OU_04 | Formulár na vyplnenie údajov o adrese trvalého pobytu. Obsahuje vstupné polia s popismi pre ulicu, obec*, súpisné/orientačné číslo* (s vysvetlením), PSČ* a štát*. Vstupné polia pre ulicu, obec a štát počas písania dávajú používateľovi návrhy na názvy, ktoré začal písať. | UC_ZF_OU_01 |
| R_ZF_OU_05 | Formulár na vyplnenie údajov o zamestnávateľovi. Vo vstupnom poli sa počas písania objavujú návrhy na názov firmy, ktorý začal písať | UC_ZF_OU_01  | 
| R_ZF_OU_06 | Checkbox + sprievodný text pre potvrdenie, že som rezident s obmedzenou daňovou povinnosťou. Obsahuje tlačidlo na otvorenie dotazníka pre pomoc s vyplnením tejto položky | UC_ZF_OU_01, UC_ZF_OU_02 |
| R_ZF_OU_07 | Checkbox pre potvrdenie, že som poberateľom dôchodku                                                                  | UC_ZF_OU_01     |
| R_ZF_OU_08 | Tlačidlo na prechod na ďaľší formulár. Tlačidlo má text v zmysle Ďalej                                                | UC_ZF_OU_03     |
| R_ZF_OU_09 | Dotazník pre určenie, či je používateľ daňový rezident. Obsahuje checkboxy pre tri otázky: "Mám na Slovensku trvalý pobyt.", "Počas minulého roka som bol na Slovensku aspoň 183 dní." a tlačidlo na odoslanie úfajov | UC_ZF_OU_02 |


### Use cases

| UC_ZF_OU_01       | Zobrazenie formuláru na vyplnenie osobných údajov na obrazovke                                                                   |
| ----------------- | -------------------------------------------------------------------------------------------------------------------------------- |
| Popis             | Zobrazenie prvkov R_ZF_OU_01, R_ZF_OU_02, R_ZF_OU_03, R_ZF_OU_04, R_ZF_OU_05, R_ZF_OU_06, R_ZF_OU_07, R_ZF_OU_08 Zoznam zamestnáveteľov ktorý sa v dropdowne bude používať ako suggestions je dostupný cez toto api: https://ekosystem.slovensko.digital/otvorene-api |
| Vstupné podmienky |                                                                                                                                  |
| Výnimky           |                                                                                                                                  |

| UC_ZF_OU_02       | Zobrazenie pomocného formuláru pre určenie, či je používateľ rezident s obmedzenou daňovou povinnosťou                           |
| ----------------- | -------------------------------------------------------------------------------------------------------------------------------- |
| Popis             | Zobrazenie dotazníka  R_ZF_OU_09 po kliknutí na tlačidlo asistencie v prvku R_ZF_OU_06.                                          |
| Vstupné podmienky |                                                                                                                                  |
| Výnimky           |                                                                                                                                  |

| UC_ZF_OU_03       | Odoslanie osobných údajov                                                                                                        |
| ----------------- | -------------------------------------------------------------------------------------------------------------------------------- |
| Popis             | Odoslanie údajov sa vykoná po kliknutí na tlačidlo Ďalej (R_ZF_OU_06). Všetky polia formulára musia byť vyplnené a číslo občianskeho preukazu musí mať validny formát|
| Vstupné podmienky | Vyplnené povinné polia v dotazníku                                                                                               |
| Výnimky           | MISSING_FIELDS: stránka sa znova načíta a pri povinných vstupných poliach sa zobrazí text "*Toto pole je povinné"                |
|                   | WRONG_ID_FORMAT: stránka sa znova načíta a pri vstupnom poli pre číslo OP sa zobrazí varovanie "Zlý formát čísla občianskeho preukazu" |

| UC_ZF_OU_04       | Osodlanie údajov z dotazníka pre určenie, či je používateľ daňový rezident s obmedtenou daňovou povinnosťou                      |
| ----------------- | -------------------------------------------------------------------------------------------------------------------------------- |
| Popis             | Po kliknutí na tlačidlo Odoslať sa spracuje výsledok na základe vstupu používateľa. Ak boli jeden alebo oba checkboxy v pomocnom formulári zaškrtnuté, NEzaškrtne sa v hlavnom formulári checkbox pre prvok R_ZF_OU_06. V opačnom prípade sa zaškrtne |
| Vstupné podmienky |                                                                                                                                  |
| Výnimky           |                                                                                                                                  |
