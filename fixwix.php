<?php
$champs = json_decode(file_get_contents('champsById.json'), true);

//https://raw.communitydragon.org/pbe/plugins/rcp-be-lol-game-data/global/de_de/v1/champion-summary.json

function getStat(array $arr, string $stat, $fallback = -1)
{
    foreach ($arr as $val) {
        if ($val[0] == $stat) {
            return $val[1];
        }
    }
    return ($fallback);
}

function convert($champtoconvert)
{
    $configs = [
        'wikia' => [
            'botName' => 'FabianLars.bot',
            'botPassword' => 'X12wZPT9V4cnLO',
        ],
    ];
    $champs = json_decode(file_get_contents('champsById.json'), true);
    $_GET['pbe'] = false;
    $champsRito = json_decode(file_get_contents('champsByIdRito.json'), true);
    $binJson = json_decode(file_get_contents('http://raw.communitydragon.org/latest/game/data/characters/' . $champsRito[$champtoconvert] . '/' . $champsRito[$champtoconvert] . '.bin.json'), true);

    $wApiUrl = 'http://de.leagueoflegends.wikia.com/api.php';

    $ch2 = curl_init();

    curl_setopt($ch2, CURLOPT_URL, $wApiUrl);
    curl_setopt($ch2, CURLOPT_COOKIEFILE, '');
    curl_setopt($ch2, CURLOPT_POST, 1);
    curl_setopt($ch2, CURLOPT_POSTFIELDS, http_build_query(array('action' => 'login', 'format' => 'json', 'lgname' => $configs['wikia']['botName'], 'lgpassword' => $configs['wikia']['botPassword'])));
    curl_setopt($ch2, CURLOPT_RETURNTRANSFER, true);

    $server_output = curl_exec($ch2);

    $json = json_decode($server_output);

    $token = (string)$json->login->token;

    curl_setopt($ch2, CURLOPT_POSTFIELDS, http_build_query(array('action' => 'login', 'format' => 'json', 'lgname' => $configs['wikia']['botName'], 'lgpassword' => $configs['wikia']['botPassword'], 'lgtoken' => $token)));

    curl_exec($ch2);

    curl_setopt($ch2, CURLOPT_HTTPGET, 1);
    curl_setopt($ch2, CURLOPT_URL, $wApiUrl . '?' . http_build_query(array('action' => 'query', 'format' => 'json', 'prop' => 'revisions', 'rvprop' => 'content', 'titles' => ('Vorlage:Data_' . $champs[$champtoconvert]))));

    $server_output3 = json_decode(curl_exec($ch2), true)['query']['pages'];

    $wikiFetched = $server_output3[array_keys($server_output3)[0]]['revisions'][0]['*'];

    $wikiArr = explode("\n", $wikiFetched);

    $wArr = [];

    foreach ($wikiArr as $val) {
        $wArr[trim(substr($val, 1, (strpos($val, "=") - 1)))] = trim(substr($val, strpos($val, "=") + 1));
    }

    $arrResource = [
        '0' => 'Mana',
        '1' => 'Energie',
    ];

    $arrStat = [
        'baseStaticHPRegen' => 'hp5_base',
        'HPRegenPerLevel' => 'hp5_lvl',
        'PrimaryAbilityResource' => 'resource',
    ];

    $arrStatW = array_flip($arrStat);

    $statsBinJson = $binJson;

    foreach ($binJson as $val) {
        if ($val[0][0] == 'mCharacterName') {
            $statsBinJson = $val;
        }
    }

    $resource = getStat($statsBinJson, 'PrimaryAbilityResource');

    $wikiFetched = preg_replace('/^(\|hp5_base).+/m', '|hp5_base         = '.(round(5.0 * getStat($statsBinJson, $arrStatW['hp5_base'], $wArr['hp5_base']), 2) . (getStat($statsBinJson, $arrStatW['hp5_base']) == -1 ? '<!--VALUE NOT FOUND. REUSING OLD VALUE-->' : '')), $wikiFetched);
    $wikiFetched = preg_replace('/^(\|hp5_lvl).+/m', '|hp5_lvl          = '.(round(5.0 * getStat($statsBinJson, $arrStatW['hp5_lvl'], $wArr['hp5_lvl']), 2) . (getStat($statsBinJson, $arrStatW['hp5_lvl']) == -1 ? '<!--VALUE NOT FOUND. REUSING OLD VALUE-->' : '')), $wikiFetched);
    if ($arrResource[$resource['arType']]) {
        $wikiFetched = preg_replace('/^(\|res5_base).+/m', '|res5_base        = '.(round(5.0 * $resource['arBaseStaticRegen'], 2)), $wikiFetched);
        $wikiFetched = preg_replace('/^(\|res5_lvl).+/m', '|res5_lvl         = '.(round(5.0 * $resource['arRegenPerLevel'], 2)), $wikiFetched);
    }

    curl_setopt($ch2, CURLOPT_URL, $wApiUrl.'?'.http_build_query(array('action' => 'query', 'format' => 'json', 'prop' => 'info', 'intoken' => 'edit', 'titles' => 'Vorlage:Data_'.$champs[$champtoconvert])));

    $editToken = json_decode(curl_exec($ch2), true);

    $editToken = reset($editToken['query']['pages'])['edittoken'];

    curl_setopt($ch2, CURLOPT_URL, $wApiUrl);
    curl_setopt($ch2, CURLOPT_POST, 1);
    curl_setopt($ch2, CURLOPT_POSTFIELDS, http_build_query(array('action' => 'edit', 'format' => 'json', 'summary' => 'wofür ist dieser knopf hier? (automated process)', 'bot' => 1, 'title' => 'Vorlage:Data_'.$champs[$champtoconvert], 'text' => $wikiFetched, 'token' => $editToken)));

    curl_exec($ch2);

    curl_close($ch2);

    print_r($wikiFetched);

}

foreach ($champs as $key => $val) {
    convert($key);
    sleep(5);
}