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
    $configs = require('./private/config.php');
    $champs = json_decode(file_get_contents('champsById.json'), true);
    $_GET['pbe'] = false;
    $champsRito = json_decode(file_get_contents('champsByIdRito.json'), true);
    $pluginJson = $_GET['pbe'] == true ? json_decode(file_get_contents('https://raw.communitydragon.org/pbe/plugins/rcp-be-lol-game-data/global/de_de/v1/champions/' . $champtoconvert . '.json'), true) : json_decode(file_get_contents('https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/de_de/v1/champions/' . $champtoconvert . '.json'), true);
    $binJson = json_decode(file_get_contents('http://raw.communitydragon.org/latest/game/data/characters/' . $champsRito[$champtoconvert] . '/' . $champsRito[$champtoconvert] . '.bin.json'), true);
    $ddragon = json_decode(file_get_contents('https://ddragon.leagueoflegends.com/cdn/8.24.1/data/de_DE/champion/' . $pluginJson['alias'] . '.json'), true);

    $ch = curl_init();
    curl_setopt($ch, CURLOPT_URL, 'https://universe-meeps.leagueoflegends.com/v1/de_de/champions/' . $champsRito[$champtoconvert] . '/index.json');
    curl_setopt($ch, CURLOPT_RETURNTRANSFER, 1);
    curl_setopt($ch, CURLOPT_AUTOREFERER, 1);
    curl_setopt($ch, CURLOPT_FOLLOWLOCATION, 1);
    curl_setopt($ch, CURLOPT_USERAGENT, 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/70.0.3538.113 Safari/537.36 Vivaldi/2.1.1337.51');
    $output = curl_exec($ch);
    curl_close($ch);

    $wApiUrl = 'https://leagueoflegends.fandom.com/de/api.php';

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

    $universeJson = json_decode($output, true);

    $arrFaction = [
        'bilgewater' => 'Bilgewasser',
        'ionia' => 'Ionia',
        'void' => 'Die Leere',
        'noxus' => 'Noxus',
        'mount-targon' => 'Targon',
        'demacia' => 'Demacia',
        'zaun' => 'Zhaun',
        'piltover' => 'Piltover',
        'shadow-isles' => 'Schatteninseln',
        'shurima' => 'Shurima',
        'freljord' => 'Freljord',
        'bandle-city' => 'Bandle',
        'unaffiliated' => 'Runeterra',
    ];

    $arrRole = [
        'mage' => 'Magier',
        'assassin' => 'Assassine',
        'support' => 'Unterstützer',
        'fighter' => 'Kämpfer',
        'marksman' => 'Schütze',
        'tank' => 'Tank',
    ];

    $arrDmgtype = [
        'kMagic' => 'Magisch',
        'kPhysical' => 'Normal',
        'kMixed' => 'Gemischt',
    ];

    $arrResource = [
        '0' => 'Mana',
        '1' => 'Energie',
    ];

    $arrStat = [
        'baseHP' => 'hp_base',
        'HPPerLevel' => 'hp_lvl',
        'baseStaticHPRegen' => 'hp5_base',
        'HPRegenPerLevel' => 'hp5_lvl',
        'BaseDamage' => 'dam_base',
        'DamagePerLevel' => 'dam_lvl',
        'baseArmor' => 'arm_base',
        'ArmorPerLevel' => 'arm_lvl',
        'baseSpellBlock' => 'mr_base',
        'SpellBlockPerLevel' => 'mr_lvl',
        'baseMoveSpeed' => 'ms',
        'attackRange' => 'range',
        'AttackSpeedRatio' => 'as_base',
        'AttackSpeedPerLevel' => 'as_lvl',
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

    $conChamps = "";

    foreach ($universeJson['related-champions'] as $val) {
        if ($conChamps != "") {
            $conChamps .= ';';
            $conChamps .= $val['name'];
        } else {
            $conChamps = $val['name'];
        }
    }

    $quote = $universeJson['champion']['biography']['quote'] . ' ~ ';
    if ($universeJson['champion']['biography']['quote-author'] != '') {
        $quote .= $universeJson['champion']['biography']['quote-author'];
    } else {
        $quote .= $pluginJson['name'];
    }

    $allyTips = '';
    $enemyTips = '';

    foreach ($ddragon['data'][$pluginJson['alias']]['allytips'] as $val) {
        $allyTips .= "\n* " . $val;
    }

    foreach ($ddragon['data'][$pluginJson['alias']]['enemytips'] as $val) {
        $enemyTips .= "\n* " . $val;
    }

    /*$skins = '';

    foreach ($pluginJson['skins'] as $val) {
        if ($val == $pluginJson['name']) {
            continue;
        }
        if ($skins != "") {
            $skins .= ';';
            $skins .= $val['name'];
        } else {
            $skins = $val['name'];
        }
    }*/

    $output = "{{ {{{1<noinclude>|Champion data</noinclude>}}}|$pluginJson[name]|{{{2|}}}|{{{3|}}}|{{{4|}}}|{{{5|}}}
|fullname         = " . (($wArr['fullname'] != "" && $wArr['fullname'] != $pluginJson['name']) ? $wArr['fullname'] : '') . "
|title            = $pluginJson[title]
|mainrole         = {$arrRole[$pluginJson['roles'][0]]}
|altrole          = {$arrRole[$pluginJson['roles'][1]]}
|attr_dmg         = {$pluginJson['playstyleInfo']['damage']}
|attr_def         = {$pluginJson['playstyleInfo']['durability']}
|attr_cc          = {$pluginJson['playstyleInfo']['crowdControl']}
|attr_mobility    = {$pluginJson['playstyleInfo']['mobility']}
|attr_sup         = {$pluginJson['playstyleInfo']['utility']}
|rangetype        = " . ((getStat($statsBinJson, $arrStatW['range'], $wArr['range']) >= 275) ? 'Fernkämpfer' : 'Nahkämpfer') . "
|damagetype       = {$arrDmgtype[$pluginJson['tacticalInfo']['damageType']]}
|style            = {$pluginJson['tacticalInfo']['style']}
|difficulty       = {$pluginJson['tacticalInfo']['difficulty']}
|resource         = " . ($arrResource[$resource['arType']] ?? ($wArr['resource']).'<!--VALUE NOT FOUND. REUSING OLD VALUE-->') . "
|hp_base          = " . round(getStat($statsBinJson, $arrStatW['hp_base'], $wArr['hp_base']), 2) . (getStat($statsBinJson, $arrStatW['hp_base']) == -1 ? '<!--VALUE NOT FOUND. REUSING OLD VALUE-->' : '') . "
|hp_lvl           = " . round(getStat($statsBinJson, $arrStatW['hp_lvl'], $wArr['hp_lvl']), 2) . (getStat($statsBinJson, $arrStatW['hp_lvl']) == -1 ? '<!--VALUE NOT FOUND. REUSING OLD VALUE-->' : '') . "
|hp5_base         = " . round(getStat($statsBinJson, $arrStatW['hp5_base'], $wArr['hp5_base']), 2) . (getStat($statsBinJson, $arrStatW['hp5_base']) == -1 ? '<!--VALUE NOT FOUND. REUSING OLD VALUE-->' : '') . "
|hp5_lvl          = " . round(getStat($statsBinJson, $arrStatW['hp5_lvl'], $wArr['hp5_lvl']), 2) . (getStat($statsBinJson, $arrStatW['hp5_lvl']) == -1 ? '<!--VALUE NOT FOUND. REUSING OLD VALUE-->' : '') . "
|res_base         = " . ((round($resource['arBase'], 2) == '0.0') ? ($wArr['arBase'].'<!--VALUE NOT FOUND. REUSING OLD VALUE-->') : round($resource['arBase'], 2)) . "
|res_lvl          = " . ((round($resource['arPerLevel'], 2) == '0.0') ? ($wArr['arPerLevel'].'<!--VALUE NOT FOUND. REUSING OLD VALUE-->') : round($resource['arPerLevel'], 2)) . "
|res5_base        = " . ((round($resource['arBaseStaticRegen'], 2) == '0.0') ? ($wArr['arBaseStaticRegen'].'<!--VALUE NOT FOUND. REUSING OLD VALUE-->') : round($resource['arBaseStaticRegen'], 2)) . "
|res5_lvl         = " . ((round($resource['arRegenPerLevel'], 2) == '0.0') ? ($wArr['arRegenPerLevel'].'<!--VALUE NOT FOUND. REUSING OLD VALUE-->') : round($resource['arRegenPerLevel'], 2)) . "
|ad_base          = " . round(getStat($statsBinJson, $arrStatW['dam_base'], $wArr['dam_base']), 2) . (getStat($statsBinJson, $arrStatW['dam_base']) == -1 ? '<!--VALUE NOT FOUND. REUSING OLD VALUE-->' : '') . "
|ad_lvl           = " . round(getStat($statsBinJson, $arrStatW['dam_lvl'], $wArr['dam_lvl']), 2) . (getStat($statsBinJson, $arrStatW['dam_lvl']) == -1 ? '<!--VALUE NOT FOUND. REUSING OLD VALUE-->' : '') . "
|armor_base       = " . round(getStat($statsBinJson, $arrStatW['arm_base'], $wArr['arm_base']), 2) . (getStat($statsBinJson, $arrStatW['arm_base']) == -1 ? '<!--VALUE NOT FOUND. REUSING OLD VALUE-->' : '') . "
|armor_lvl        = " . round(getStat($statsBinJson, $arrStatW['arm_lvl'], $wArr['arm_lvl']), 2) . (getStat($statsBinJson, $arrStatW['arm_lvl']) == -1 ? '<!--VALUE NOT FOUND. REUSING OLD VALUE-->' : '') . "
|mr_base          = " . round(getStat($statsBinJson, $arrStatW['mr_base'], $wArr['mr_base']), 2) . (getStat($statsBinJson, $arrStatW['mr_base']) == -1 ? '<!--VALUE NOT FOUND. REUSING OLD VALUE-->' : '') . "
|mr_lvl           = " . round(getStat($statsBinJson, $arrStatW['mr_lvl'], $wArr['mr_lvl']), 2) . (getStat($statsBinJson, $arrStatW['mr_lvl']) == -1 ? '<!--VALUE NOT FOUND. REUSING OLD VALUE-->' : '') . "
|as_base          = " . round(getStat($statsBinJson, $arrStatW['as_base'], $wArr['as_base']), 3) . (getStat($statsBinJson, $arrStatW['as_base']) == -1 ? '<!--VALUE NOT FOUND. REUSING OLD VALUE-->' : '') . "
|as_lvl           = " . round(getStat($statsBinJson, $arrStatW['as_lvl'], $wArr['as_lvl']), 2) . (getStat($statsBinJson, $arrStatW['as_lvl']) == -1 ? '<!--VALUE NOT FOUND. REUSING OLD VALUE-->' : '') . "
|as_lvl1_bonus    = $wArr[as_lvl1_bonus]
|range            = " . getStat($statsBinJson, $arrStatW['range'], $wArr['range']) . (getStat($statsBinJson, $arrStatW['range']) == -1 ? '<!--VALUE NOT FOUND. REUSING OLD VALUE-->' : '') . "
|ms               = " . getStat($statsBinJson, $arrStatW['ms'], $wArr['ms']) . (getStat($statsBinJson, $arrStatW['ms']) == -1 ? '<!--VALUE NOT FOUND. REUSING OLD VALUE-->' : '') . "
|image2           = $wArr[image2]
|image3           = $wArr[image3]
|date             = $wArr[date]
|patch            = $wArr[patch]
|changes          = $wArr[changes]
|be               = $wArr[be]
|rp               = $wArr[rp]
|id               = $champtoconvert
|skins            = $wArr[skins]
|skinsspecialname = " . str_replace(',', ';', $wArr['skinsspecialname']) . "
|skinsdate        = " . str_replace(',', ';', $wArr['skinsdate']) . "
|skinsprice       = " . str_replace(',', ';', $wArr['skinsprice']) . "
|chromas          = " . str_replace(',', ';', $wArr['chromas']) . "
|chromanames      = $wArr[chromanames]
|chromacondition  = $wArr[chromacondition]
|skill_p          = {$pluginJson['passive']['name']}
|skill_q          = {$pluginJson['spells'][0]['name']}
|skill_w          = {$pluginJson['spells'][1]['name']}
|skill_e          = {$pluginJson['spells'][2]['name']}
|skill_r          = {$pluginJson['spells'][3]['name']}
|region           = " . ($arrFaction[$universeJson['champion']['associated-faction-slug']] ?? ('NO REGION FOUND: ' . $universeJson['champion']['associated-faction-slug'])) . "
|race             = " . ($wArr['race'] != '' ? $wArr['race'] : ($universeJson['champion']['races'][0]['name'] ?? 'Mensch')) . "
|conchamps        = $conChamps
|quote            = $quote
|shortstory       = " . str_replace(['<p>', '</p>'], '',  $universeJson['champion']['biography']['short']) . "
|allytips         = $allyTips
|enemytips        = $enemyTips
}}";

    curl_setopt($ch2, CURLOPT_URL, $wApiUrl.'?'.http_build_query(array('action' => 'query', 'format' => 'json', 'prop' => 'info', 'intoken' => 'edit', 'titles' => 'Vorlage:Data_'.$champs[$champtoconvert])));

    $editToken = json_decode(curl_exec($ch2), true);

    $editToken = reset($editToken['query']['pages'])['edittoken'];

    curl_setopt($ch2, CURLOPT_URL, $wApiUrl);
    curl_setopt($ch2, CURLOPT_POST, 1);
    curl_setopt($ch2, CURLOPT_POSTFIELDS, http_build_query(array('action' => 'edit', 'format' => 'json', 'summary' => 'wofür ist dieser knopf hier? (automated process)', 'bot' => 1, 'title' => 'Vorlage:Data_'.$champs[$champtoconvert], 'text' => $output, 'token' => $editToken)));

    curl_exec($ch2);

    curl_close($ch2);

    print_r($output);

}

foreach ($champs as $key => $val) {
    convert($key);
    sleep(5);
}
