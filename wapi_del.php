<?php

// Batch Delete

$configs = require('./private/config.php');
$arrToDelete = file('todelete.txt', FILE_IGNORE_NEW_LINES);

$wApiUrl = 'https://leagueoflegends.fandom.com/de/api.php';

$ch = curl_init();

curl_setopt($ch, CURLOPT_URL, $wApiUrl);
curl_setopt($ch, CURLOPT_COOKIEFILE, '');
curl_setopt($ch, CURLOPT_POST, 1);
curl_setopt($ch, CURLOPT_POSTFIELDS, http_build_query(array('action' => 'login', 'format' => 'json', 'lgname' => $configs['wikia']['botName'], 'lgpassword' => $configs['wikia']['botPassword'])));
curl_setopt($ch, CURLOPT_RETURNTRANSFER, true);

$server_output = curl_exec($ch);

$json = json_decode($server_output);

$token = (string) $json->login->token;

curl_setopt($ch, CURLOPT_POSTFIELDS, http_build_query(array('action' => 'login', 'format' => 'json', 'lgname' => $configs['wikia']['botName'], 'lgpassword' => $configs['wikia']['botPassword'], 'lgtoken' => $token)));

$server_output2 = curl_exec($ch);

curl_setopt($ch, CURLOPT_POSTFIELDS, http_build_query(array('action' => 'query', 'format' => 'json', 'prop' => 'info', 'intoken' => 'delete', 'titles' => implode("|", $arrToDelete))));

$deleteToken = json_decode(curl_exec($ch), true);

$deleteToken = reset($deleteToken['query']['pages'])['deletetoken'];

foreach ($arrToDelete as $page) {
    curl_setopt($ch, CURLOPT_POSTFIELDS, http_build_query(array('action' => 'delete', 'reason' => 'Frühjahrsputz im Spätsommer kurz vor Weihnachten (automated process)', 'title' => $page, 'token' => $deleteToken)));
    curl_exec($ch);
    sleep(1);
}

curl_close($ch);