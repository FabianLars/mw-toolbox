<?php

//  --- WEEKLY EXECUTION VIA CRON-TASK ---

// 1.) Get Current Champion Rotation from Riot's API

$requestUrl = 'https://euw1.api.riotgames.com/lol/platform/v3/champion-rotations?api_key='.getenv('RIOT_API_KEY');

$champList = json_decode(file_get_contents('./wikibot/champsById.json'), true);

$rotationResult = file_get_contents($requestUrl, true);
$rotationDecoded = json_decode($rotationResult, true);
$rotation = $rotationDecoded['freeChampionIds'];

foreach ($rotation as $key => $value) {
    $rotation[$key] = $champList[$value];
}

sort($rotation);

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
curl_setopt($ch, CURLOPT_URL, $wApiUrl.'?'.http_build_query(array('action' => 'query', 'format' => 'json', 'prop' => 'revisions', 'rvprop' => 'content', 'titles' => 'Vorlage:Aktuelle_Championrotation')));

$server_output3 = json_decode(curl_exec($ch), true)['query']['pages'];

$page_content = $server_output3[array_keys($server_output3)[0]]['revisions'][0]['*'];

preg_match('/^(\|([A-Za-z\'&\. \|\s]+))}}/m', $page_content, $matches);

$page_content = str_replace($matches[1], '|' . implode('|', $rotation), $page_content);

curl_setopt($ch, CURLOPT_URL, $wApiUrl.'?'.http_build_query(array('action' => 'query', 'format' => 'json', 'prop' => 'info', 'intoken' => 'edit', 'titles' => 'Vorlage:Aktuelle_Championrotation')));

$editToken = json_decode(curl_exec($ch), true);

$editToken = reset($editToken['query']['pages'])['edittoken'];

curl_setopt($ch, CURLOPT_URL, $wApiUrl);
curl_setopt($ch, CURLOPT_POST, 1);
curl_setopt($ch, CURLOPT_POSTFIELDS, http_build_query(array('action' => 'edit', 'format' => 'json', 'summary' => 'automated process', 'bot' => 1, 'title' => 'Vorlage:Aktuelle_Championrotation', 'text' => $page_content, 'token' => $editToken)));

curl_exec($ch);

curl_close($ch);
