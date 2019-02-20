<?php
$wApiUrl = 'https://leagueoflegends.fandom.com/de/api.php';
$configs = require('./private/config.php');

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

curl_setopt($ch, CURLOPT_HTTPGET, 1);

curl_setopt($ch, CURLOPT_URL, 'https://leagueoflegends.fandom.com/de/api.php?action=query&format=json&list=allimages&ailimit=5000');
file_put_contents('files.txt', curl_exec($ch), FILE_APPEND);

curl_setopt($ch, CURLOPT_URL, 'https://leagueoflegends.fandom.com/de/api.php?action=query&format=json&list=allimages&ailimit=5000&aifrom=LeBlanc%20Verzerrung%20Zurück');
file_put_contents('files.txt', curl_exec($ch), FILE_APPEND);

curl_close($ch);