<?php

//  --- WEEKLY EXECUTION VIA CRON-TASK ---
setlocale(LC_TIME, array('de_DE.UTF-8','de_DE@euro','de_DE','german'));

// 0.) heredoc-String of wiki-template
function getWikiTemplate($rotations, $rotationNewPlayers) {
    $df = "%e. %B %Y";
    $datesFrom = [
        strftime($df,strftime(date_timestamp_get(date_sub(date_create(), date_interval_create_from_date_string('14 days'))))),
        strftime($df,date_timestamp_get(date_sub(date_create(), date_interval_create_from_date_string('7 days')))),
        strftime($df),
    ];

    $datesTo = [
        strftime($df,date_timestamp_get(date_sub(date_create(), date_interval_create_from_date_string('7 days')))),
        strftime("%e. %B %Y"),
        strftime($df,date_timestamp_get(date_add(date_create(), date_interval_create_from_date_string('7 days')))),
    ];

    return <<<EOT
<div style="text-align:center; font-size: 125%; font-weight:bold; margin: 2px 0 0;">[[Kostenlose Championrotation]]</div><div style="text-align:left; font-size: 80%; font-weight:bold; margin: 2px 0 0;">[[Vorlage:Aktuelle Championrotation|Bearbeiten]]</div>
<tabber>Klassisch=
{{#ifeq:{{FULLPAGENAME}}|Vorlage:Aktuelle Championrotation|{{#ifeq:{{#time:N|{{CURRENTTIMESTAMP}}}}|2|{{#ifexpr:{{#expr:{{#time:U|{{REVISIONTIMESTAMP}}}}+100000}}<{{#time:U|{{CURRENTTIMESTAMP}}}}|[[Kategorie:Datumskategorie Championrotation]]}}}}}}{{Aktuelle Championrotation/var
|specialweek      = <!-- Nur für Sonderfälle, sonst leer lassen! -->
|specialstartdate = <!-- Nur für Sonderfälle, sonst leer lassen! -->
|specialenddate   = <!-- Nur für Sonderfälle, sonst leer lassen! -->
|datefrom         = <!-- Nur für die ARAM-Rotation verwendet, sonst leer lassen! -->
|dateto           = <!-- Nur für die ARAM-Rotation verwendet, sonst leer lassen! -->
|lastchecked      = <!-- Nur für die Rotation neuer Accounts, sonst leer lassen! -->
$rotations[0]}}


|-|ARAM=
<p style="text-align: center; margin: 0 15%;">''Alle Zufällig''-Spiele erlauben es Spielern, Champions aus den letzten beiden Championrotationen sowie aus der aktuellen zu rollen. Dopplungen erhohen hierbei nicht die Wahrscheinlichkeit, den Champion zu ziehen.</p>
{{Aktuelle Championrotation/var
|specialweek      = <!-- Nur für Sonderfälle, sonst leer lassen! -->
|specialstartdate = <!-- Nur für Sonderfälle, sonst leer lassen! -->
|specialenddate   = <!-- Nur für Sonderfälle, sonst leer lassen! -->
|datefrom         = $datesFrom[0]
|dateto           = $datesTo[0]
|lastchecked      = <!-- Nur für die Rotation neuer Accounts, sonst leer lassen! -->
$rotations[2]}}

{{Aktuelle Championrotation/var
|specialweek      = <!-- Nur für Sonderfälle, sonst leer lassen! -->
|specialstartdate = <!-- Nur für Sonderfälle, sonst leer lassen! -->
|specialenddate   = <!-- Nur für Sonderfälle, sonst leer lassen! -->
|datefrom         = $datesFrom[1]
|dateto           = $datesTo[1]
|lastchecked      = <!-- Nur für die Rotation neuer Accounts, sonst leer lassen!-->
$rotations[1]}}

{{Aktuelle Championrotation/var
|specialweek      = <!-- Nur für Sonderfälle, sonst leer lassen! -->
|specialstartdate = <!-- Nur für Sonderfälle, sonst leer lassen! -->
|specialenddate   = <!-- Nur für Sonderfälle, sonst leer lassen! -->
|datefrom         = $datesFrom[2]
|dateto           = $datesTo[2]
|lastchecked      = <!-- Nur für die Rotation neuer Accounts, sonst leer lassen! -->
$rotations[0]}}


|-|Neue Accounts=
<p style="text-align: center; margin: 0 15%;">Vor [[Erfahrung (Beschwörer)|Stufe 11]] haben Spieler Zugriff auf eine andere Championrotation. Diese wird seltener aktualisiert, deshalb könnte es sein, dass die folgende Liste nicht mehr korrekt ist.</p>
{{Aktuelle Championrotation/var
|specialweek      = <!-- Nur für Sonderfälle, sonst leer lassen! -->
|specialstartdate = <!-- Nur für Sonderfälle, sonst leer lassen! -->
|specialenddate   = <!-- Nur für Sonderfälle, sonst leer lassen! -->
|datefrom         = <!-- Nur für die ARAM-Rotation verwendet, sonst leer lassen! -->
|dateto           = <!-- Nur für die ARAM-Rotation verwendet, sonst leer lassen! -->
|lastchecked      = $datesFrom[2]
$rotationNewPlayers}}
</tabber><noinclude>{{Dokumentation}}<noinclude>
EOT;
}



// 1.) Get Current Champion Rotation from Riot's API

$requestUrl = 'https://euw1.api.riotgames.com/lol/platform/v3/champion-rotations?api_key='.getenv('RIOT_API_KEY');

$champList = json_decode(file_get_contents('/home/fabianlars/wikibot/champsById.json'), true);

$rotationResult = file_get_contents($requestUrl, true);
$rotationDecoded = json_decode($rotationResult, true);
$rotation = $rotationDecoded['freeChampionIds'];
$newPlayers = $rotationDecoded['freeChampionIdsForNewPlayers'];

foreach ($rotation as $key => $value) {
    $rotation[$key] = $champList[$value];
}

foreach ($newPlayers as $key => $value) {
    $newPlayers[$key] = $champList[$value];
}

sort($rotation);
sort($newPlayers);

$rotation_arr = unserialize(file_get_contents("/home/fabianlars/wikibot/rotation.txt"));
array_unshift($rotation_arr, '|'. implode('|', $rotation));
array_pop($rotation_arr);
file_put_contents("/home/fabianlars/wikibot/rotation.txt", serialize($rotation_arr));

// 2.) wikia bot account login; get current page content; regex replace the updated champs; save to wiki

$wApiUrl = 'https://leagueoflegends.fandom.com/de/api.php';

$ch = curl_init();

curl_setopt($ch, CURLOPT_URL, $wApiUrl);
curl_setopt($ch, CURLOPT_COOKIEFILE, '');
curl_setopt($ch, CURLOPT_POST, 1);
curl_setopt($ch, CURLOPT_POSTFIELDS, http_build_query(array('action' => 'login', 'format' => 'json', 'lgname' => getenv('FANDOM_BOT_NAME'), 'lgpassword' => getenv('FANDOM_BOT_PASSWORD'))));
curl_setopt($ch, CURLOPT_RETURNTRANSFER, true);

$server_output = curl_exec($ch);

$json = json_decode($server_output);

$token = (string) $json->login->token;

curl_setopt($ch, CURLOPT_POSTFIELDS, http_build_query(array('action' => 'login', 'format' => 'json', 'lgname' => getenv('FANDOM_BOT_NAME'), 'lgpassword' => getenv('FANDOM_BOT_PASSWORD'), 'lgtoken' => $token)));

$server_output2 = curl_exec($ch);

curl_setopt($ch, CURLOPT_HTTPGET, 1);
curl_setopt($ch, CURLOPT_URL, $wApiUrl.'?'.http_build_query(array('action' => 'query', 'format' => 'json', 'prop' => 'info', 'intoken' => 'edit', 'titles' => 'Vorlage:Aktuelle_Championrotation')));

$editToken = json_decode(curl_exec($ch), true);

$editToken = reset($editToken['query']['pages'])['edittoken'];

$page_content = getWikiTemplate($rotation_arr, '|' . implode('|', $newPlayers));

curl_setopt($ch, CURLOPT_URL, $wApiUrl);
curl_setopt($ch, CURLOPT_POST, 1);
curl_setopt($ch, CURLOPT_POSTFIELDS, http_build_query(array('action' => 'edit', 'format' => 'json', 'summary' => 'automated process', 'bot' => 1, 'title' => 'Vorlage:Aktuelle_Championrotation', 'text' => $page_content, 'token' => $editToken)));

curl_exec($ch);

curl_close($ch);
