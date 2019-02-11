<?php
$wApiUrl = 'https://leagueoflegends.fandom.com/de/api.php';
$configs = require('./private/config.php');
$champs = json_decode(file_get_contents('champsById.json'), true);
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



function updateSkins($champId) {
    global $champs;
    global $wApiUrl;
    global $ch;
    $cdragonSkins = json_decode(file_get_contents('https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/de_de/v1/champions/'.$champId.'.json'), true)['skins'];
    array_shift($cdragonSkins);
    $skins = [];
    foreach ($cdragonSkins as $val) {
        $skins[] = $val["name"];
    }

    curl_setopt($ch, CURLOPT_HTTPGET, 1);
    curl_setopt($ch, CURLOPT_URL, $wApiUrl . '?' . http_build_query(array('action' => 'query', 'format' => 'json', 'prop' => 'revisions', 'rvprop' => 'content', 'titles' => ('Vorlage:Data_'.$champs[$champId]))));

    $server_output3 = json_decode(curl_exec($ch), true)['query']['pages'];

    $wikiFetched = $server_output3[array_keys($server_output3)[0]]['revisions'][0]['*'];

    $wikiFetched = preg_replace('/^(\|skins ).+/m', '|skins            = '.implode(';', $skins), $wikiFetched);

    curl_setopt($ch, CURLOPT_URL, $wApiUrl.'?'.http_build_query(array('action' => 'query', 'format' => 'json', 'prop' => 'info', 'intoken' => 'edit', 'titles' => 'Vorlage:Data_'.$champs[$champId])));

    $editToken = json_decode(curl_exec($ch), true);

    $editToken = reset($editToken['query']['pages'])['edittoken'];

    curl_setopt($ch, CURLOPT_URL, $wApiUrl);
    curl_setopt($ch, CURLOPT_POST, 1);
    curl_setopt($ch, CURLOPT_POSTFIELDS, http_build_query(array('action' => 'edit', 'format' => 'json', 'summary' => 'achtung, profi am werk (automated process)', 'bot' => 1, 'title' => 'Vorlage:Data_'.$champs[$champId], 'text' => $wikiFetched, 'token' => $editToken)));

    curl_exec($ch);

    print_r($wikiFetched);

}

foreach ($champs as $key => $val) {
    updateSkins($key);
    sleep(2);
}

curl_close($ch);