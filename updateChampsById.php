<?php

$baseLink = "https://raw.communitydragon.org/pbe/plugins/rcp-be-lol-game-data/global/default/v1/champions/";
$dir = file_get_contents($baseLink);

preg_match_all('/^(<tr><td><a href=).+/m', $dir, $matches);

$matches = $matches[0];

$jsonLinks = [];

foreach ($matches as $v) {
    preg_match('/(\d)+\.json/', $v, $m);
    array_push($jsonLinks, $m[0]);
}

$jsons = [];
foreach ($jsonLinks as $v) {
    $json = json_decode(file_get_contents($baseLink . $v), true);
    $arr = array(
        "id" => $json["id"],
        "name" => $json["name"],
        "rito" => $json["alias"]
    );
    array_push($jsons, $arr);
}

$champlist = [];
$champlistRito = [];

foreach ($jsons as $v) {
    $champlist[$v["id"]] = $v["name"];
    $champlistRito[$v["id"]] = $v["name"];
}

asort($champlist);
asort($champlistRito);

file_put_contents('champsById.json', json_encode($champlist, JSON_PRETTY_PRINT));
file_put_contents('champsByIdRito.json', json_encode($champlistRito, JSON_PRETTY_PRINT));
