<?php
$wApiUrl = 'https://leagueoflegends.fandom.com/de/api.php';
$configs = require('./private/config.php');
$files = require('movelist.php');
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

    $spacePos = strrpos($val, ' ');

    $champ = substr($val, 0, $spacePos);
    $champ = substr($champ, strpos($val, ':')+1);

    $color = substr($val, $spacePos+1);
    preg_match('/[A-Z]/', $color, $matches, PREG_OFFSET_CAPTURE, 1);

    $chromaColor = substr($color, 0, $matches[0][1]);

    if (strpos($val, 'ChromaRender')) {
        $skin = substr($color, $matches[0][1]);
        $skin = substr($skin, 0, strpos($skin, '-')+1);

        $dest = 'Datei:' . $champ . ' ' . $skin . $champ . ' (' . $chromaColor . ') M.png';
    } else {
        $skin = substr($color, 0, strpos($color, '-')+1);
        $dest = 'Datei:' . $champ . ' ' . $skin . $champ . ' M.png';
    }

    print_r($dest);

    curl_setopt($ch, CURLOPT_POSTFIELDS, http_build_query(array('action' => 'move', 'from' => $val,
        'to' => $dest, 'format' => 'json', 'reason' => 'automated process', 'movetalk' => 1, 'token' => $moveToken)));
    print_r(curl_exec($ch));
    echo 'xxxxx';
    sleep(1);

}


curl_close($ch);