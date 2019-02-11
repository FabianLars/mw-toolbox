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



function move($champId) {
    global $champs;
    $cdragonSkins = json_decode(file_get_contents('https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/de_de/v1/champions/'.$champId.'.json'), true)['skins'];
    global $wApiUrl;
    global $ch;

    curl_setopt($ch, CURLOPT_HTTPGET, 1);
    curl_setopt($ch, CURLOPT_URL, $wApiUrl . '?' . http_build_query(array('action' => 'query', 'format' => 'json', 'prop' => 'revisions', 'rvprop' => 'content', 'titles' => ('Vorlage:Data_'.$champs[$champId]))));

    $server_output3 = json_decode(curl_exec($ch), true)['query']['pages'];

    $wikiFetched = $server_output3[array_keys($server_output3)[0]]['revisions'][0]['*'];

    $wikiArr = explode("\n", $wikiFetched);

    $wArr = [];

    foreach ($wikiArr as $val) {
        $wArr[trim(substr($val, 1, (strpos($val, "=") - 1)))] = trim(substr($val, strpos($val, "=") + 1));
    }

    $wSkins = explode(';', $wArr['skins']);
    array_unshift($wSkins, "Standard");

    //print_r($wSkins);

    //print_r($cdragonSkins);

    $titles = '';
    foreach ($wSkins as $key => $val) {
        $titles = $titles . "Datei:" . $champs[$champId] . " " . $val . 'Splash.jpg|' ."Datei:" . $champs[$champId] . " " . $val . 'Loading.jpg|';
    }

    //print_r($titles);

    curl_setopt($ch, CURLOPT_URL, $wApiUrl . '?' . http_build_query(array('action' => 'query', 'format' => 'json', 'prop' => 'info', 'intoken' => 'move', 'titles' => $titles)));

    $moveToken = reset(json_decode(curl_exec($ch), true)['query']['pages'])['movetoken'];

    print_r($moveToken);

    curl_setopt($ch, CURLOPT_URL, $wApiUrl);
    curl_setopt($ch, CURLOPT_POST, 1);

    foreach ($wSkins as $key => $val) {
        if ($key == 0) {
            curl_setopt($ch, CURLOPT_POSTFIELDS, http_build_query(array('action' => 'move', 'from' => 'Datei:' . str_replace(array(":", "/"), array("-", ""), $champs[$champId]) . ' ' . $val . 'Splash.jpg',
                    'to' => 'Datei:' . $champs[$champId] . ' Standard ' . $cdragonSkins[$key]['name'] . ' S.jpg', 'format' => 'json', 'reason' => 'automated process', 'movetalk' => 1, 'token' => $moveToken)));
            print_r(curl_exec($ch));
            echo 'xxxxx';
            sleep(1);

            curl_setopt($ch, CURLOPT_POSTFIELDS, http_build_query(array('action' => 'move', 'from' => 'Datei:' . $champs[$champId] . ' ' . $val . 'Loading.jpg',
                    'to' => 'Datei:' . $champs[$champId] . ' Standard ' . $cdragonSkins[$key]['name'] . ' L.jpg', 'format' => 'json', 'reason' => 'automated process', 'movetalk' => 1, 'token' => $moveToken)));
            print_r(curl_exec($ch));
            echo 'xxxxx';
            sleep(1);

        } else {
            curl_setopt($ch, CURLOPT_POSTFIELDS, http_build_query(array('action' => 'move', 'from' => 'Datei:' . $champs[$champId] . ' ' . $val . 'Splash.jpg',
                    'to' => 'Datei:' . $champs[$champId] . ' ' . $cdragonSkins[$key]['name'] . ' S.jpg', 'format' => 'json', 'reason' => 'automated process', 'movetalk' => 1, 'token' => $moveToken)));
            print_r(curl_exec($ch));
            echo 'xxxxx';
            sleep(1);

            curl_setopt($ch, CURLOPT_POSTFIELDS, http_build_query(array('action' => 'move', 'from' => 'Datei:' . $champs[$champId] . ' ' . $val . 'Loading.jpg',
                    'to' => 'Datei:' . $champs[$champId] . ' ' . $cdragonSkins[$key]['name'] . ' L.jpg', 'format' => 'json', 'reason' => 'automated process', 'movetalk' => 1, 'token' => $moveToken)));
            print_r(curl_exec($ch));
            echo 'xxxxx';
            sleep(1);
        }
    }
}

foreach ($champs as $key => $val) {
    move($key);
}

curl_close($ch);