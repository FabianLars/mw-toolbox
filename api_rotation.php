<?php
/* config storage. api key will be added to it.
$configsjson = file_get_contents($_SERVER["DOCUMENT_ROOT"] . '/../httpdocs/configs.json');
$configs = json_decode($configsjson, true);
$apiKey = $configs['riotapi']['productionKey'];
*/

$apiKey = 'RGAPI-ef5b6cf4-3fe3-40e3-92dd-2b3a2b516d1a';
$requestUrl = 'https://euw1.api.riotgames.com/lol/platform/v3/champion-rotations?api_key=' . $apiKey;

$champList = json_decode(file_get_contents('champsById.json'), true);

$rotationResult = file_get_contents($requestUrl, true);
$rotationDecoded = json_decode($rotationResult, true);
$rotation = $rotationDecoded['freeChampionIds'];

foreach ($rotation as $key => $value) {
    $rotation[$key] = $champList[$value];
}

sort($rotation);

echo "<pre>";
print_r($rotation);