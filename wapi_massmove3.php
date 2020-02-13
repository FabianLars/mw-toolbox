<?php
$wApiUrl = 'https://leagueoflegends.fandom.com/de/api.php';
$configs = require('./private/config.php');
$files = require('movelist2.php');
$ch = curl_init();
curl_setopt($ch, CURLOPT_URL, $wApiUrl);
curl_setopt($ch, CURLOPT_FOLLOWLOCATION, true);
curl_setopt($ch, CURLOPT_SSL_VERIFYPEER, false);
curl_setopt($ch, CURLOPT_COOKIEFILE, '');
curl_setopt($ch, CURLOPT_POST, 1);
curl_setopt($ch, CURLOPT_POSTFIELDS, http_build_query(array('action' => 'login', 'format' => 'json', 'lgname' => $configs['wikia']['botName'], 'lgpassword' => $configs['wikia']['botPassword'])));
curl_setopt($ch, CURLOPT_RETURNTRANSFER, true);
$server_output = curl_exec($ch);
$json = json_decode($server_output);
$token = (string)$json->login->token;
curl_setopt($ch, CURLOPT_POSTFIELDS, http_build_query(array('action' => 'login', 'format' => 'json', 'lgname' => $configs['wikia']['botName'], 'lgpassword' => $configs['wikia']['botPassword'], 'lgtoken' => $token)));
$server_output2 = curl_exec($ch);

$titles = implode('|', $files);

curl_setopt($ch, CURLOPT_HTTPGET, 1);
curl_setopt($ch, CURLOPT_URL, $wApiUrl . '?' . http_build_query(array('action' => 'query', 'format' => 'json', 'prop' => 'info', 'intoken' => 'move', 'titles' => $titles)));

$moveToken = reset(json_decode(curl_exec($ch), true)['query']['pages'])['movetoken'];

curl_setopt($ch, CURLOPT_URL, $wApiUrl);
curl_setopt($ch, CURLOPT_POST, 1);


foreach ($files as $val) {

    $dest = str_replace("-", " ", $val);

    curl_setopt($ch, CURLOPT_POSTFIELDS, http_build_query(array('action' => 'move', 'from' => $val,
        'to' => $dest, 'format' => 'json', 'reason' => 'automated process', 'movetalk' => 1, 'token' => $moveToken)));
    print_r(curl_exec($ch));
    echo 'xxxxx';
    sleep(1);

}


curl_close($ch);