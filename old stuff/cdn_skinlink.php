<?php
$decJson = json_decode(file_get_contents('https://raw.communitydragon.org/pbe/plugins/rcp-be-lol-game-data/global/de_de/v1/skins.json'), true);
$skinId = '';
$skin = urldecode(str_replace('_', ' ', $_SERVER['QUERY_STRING']));

foreach (array_values($decJson) as $value) {
    if ($value['name'] == $skin) {
        $skinId=$value['id'];
    }
}

header('Location: https://www.teemo.gg/model-viewer?skinid='.$skinId);
die();